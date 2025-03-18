use super::{AppError, AppState};
use crate::db::{
    IdType, create_id,
    models::{sessions, users},
    now_utc,
};
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
    routing::post,
};
use bcrypt::verify;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tracing::debug;

pub(crate) fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

#[derive(Deserialize, Debug)]
struct LoginBody {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: String,
    username: String,
    created_at: String,
    updated_at: String,
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginBody>,
) -> Result<Response, AppError> {
    debug!("Login request: {:?}", payload);

    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&payload.username))
        .one(&state.db)
        .await?
        .ok_or(AppError::new(StatusCode::UNAUTHORIZED, "User not found"))?;

    // Verify password
    let valid = verify(&payload.password, &user.password)?;
    if !valid {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid username or password",
        ));
    }

    // Save session to database
    let now = now_utc();
    let session = sessions::ActiveModel {
        id: Set(create_id(IdType::Session)),
        created_at: Set(now.clone()),
        updated_at: Set(now.clone()),
        user_id: Set(user.id.clone()),
        token: Set(format!("{}{}", cuid2::create_id(), cuid2::create_id())),
        last_seen: Set(now.clone()),
        label: Set(Some("Web Login".to_string())),
    };
    let session = session.insert(&state.db).await?;

    let response_body = UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };
    let mut response = Json(response_body).into_response();

    let cookie = format!(
        "colink_session={}; Path=/; HttpOnly; SameSite=Strict",
        session.token
    );
    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

    Ok(response)
}

async fn logout() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}
