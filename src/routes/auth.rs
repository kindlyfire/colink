use super::{AppError, AppState, ExtractAppState, ExtractUser};
use crate::db::{IdType, models::sessions, now_utc};
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use bcrypt::verify;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::debug;

pub(crate) fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
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
    State(state): ExtractAppState,
    Json(payload): Json<LoginBody>,
) -> Result<Response, AppError> {
    debug!("Login request: {:?}", payload);

    let user = state
        .repo
        .user_by_username(&payload.username)
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
        id: Set(IdType::Session.create()),
        created_at: Set(now.clone()),
        updated_at: Set(now.clone()),
        user_id: Set(user.id.clone()),
        token: Set(format!("{}{}", cuid2::create_id(), cuid2::create_id())),
        last_seen: Set(now.clone()),
        label: Set(Some("Web Login".to_string())),
    };
    let session = session.insert(&state.repo.conn).await?;

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

// TODO: Make sure this can't be called using a form (check for
// application/json? seems simplest)
async fn logout(State(state): ExtractAppState, jar: CookieJar) -> Result<Response, AppError> {
    let session_token = jar
        .get("colink_session")
        .map(|cookie| cookie.value().to_owned());

    let mut response = (StatusCode::OK, "Logged out").into_response();

    if let Some(token) = &session_token {
        // Delete the session from the database
        sessions::Entity::delete_many()
            .filter(sessions::Column::Token.eq(token))
            .exec(&state.repo.conn)
            .await?;

        let cookie = "colink_session=; Path=/; HttpOnly; SameSite=Strict; Expires=Thu, 01 Jan 1970 00:00:00 GMT";
        response
            .headers_mut()
            .insert(SET_COOKIE, HeaderValue::from_str(cookie).unwrap());
    }

    Ok(response)
}

async fn me(ExtractUser(user): ExtractUser) -> Result<Json<UserResponse>, AppError> {
    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(Json(user_response))
}
