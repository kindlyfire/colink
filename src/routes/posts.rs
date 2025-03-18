use std::sync::Arc;

use axum::{Json, Router, routing::get};
use serde_json::Value;

use super::{AppError, AppState};

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

async fn get_posts() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}

async fn create_post() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}

async fn get_post_by_id() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}

async fn update_post_by_id() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}

async fn delete_post_by_id() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}
