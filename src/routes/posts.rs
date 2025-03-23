use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{AppError, AppState, ExtractAppState, ExtractUser};
use crate::{
    db::{IdType, models::posts, now_utc},
    search::Post,
};

#[derive(Deserialize)]
struct CreatePostRequest {
    text: String,
}

pub(crate) fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_posts).post(create_post))
        .route(
            "/{id}",
            get(get_post_by_id)
                .post(update_post_by_id)
                .delete(delete_post_by_id),
        )
}

async fn get_posts(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
) -> Result<Json<Value>, AppError> {
    let posts = posts::Entity::find()
        .filter(posts::Column::UserId.eq(user.id.clone()))
        .order_by_desc(posts::Column::CreatedAt)
        .all(&state.repo.conn)
        .await?;

    Ok(Json(json!({ "data": posts })))
}

async fn create_post(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<Value>, AppError> {
    let now = now_utc();

    let post = posts::ActiveModel {
        id: Set(IdType::Post.create()),
        created_at: Set(now.clone()),
        updated_at: Set(now),
        user_id: Set(user.id.clone()),
        text: Set(payload.text),
    };

    let post = post.insert(&state.repo.conn).await?;

    if let Some(search) = state.get_search() {
        search
            .post_upsert(Post {
                id: post.id.clone(),
                user_id: user.id.clone(),
                text: post.text.clone(),
            })
            .await?;
    }

    Ok(Json(json!({ "data": post })))
}

async fn get_post_by_id(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let post = state
        .repo
        .user_post_by_id(&id, &user.id)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Post not found"))?;

    Ok(Json(json!({ "data": post })))
}

#[derive(Deserialize)]
struct UpdatePostRequest {
    text: String,
}

async fn update_post_by_id(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<Value>, AppError> {
    let post = state
        .repo
        .user_post_by_id(&id, &user.id)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Post not found"))?;

    let mut post = post.into_active_model();
    post.text = Set(payload.text);
    post.updated_at = Set(now_utc());

    let post = post.update(&state.repo.conn).await?;

    if let Some(search) = state.get_search() {
        search
            .post_upsert(Post {
                id: post.id.clone(),
                user_id: user.id.clone(),
                text: post.text.clone(),
            })
            .await?;
    }

    Ok(Json(json!({ "data": post })))
}

async fn delete_post_by_id(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let post = state
        .repo
        .user_post_by_id(&id, &user.id)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Post not found"))?;
    let post_id = post.id.clone();
    post.delete(&state.repo.conn).await?;

    if let Some(search) = state.get_search() {
        search.post_delete(&post_id).await?;
    }

    Ok(Json(json!({ "success": true })))
}
