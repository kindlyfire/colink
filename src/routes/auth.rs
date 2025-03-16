use axum::{Json, Router, routing::post};
use serde_json::Value;

use super::AppError;

pub(crate) fn get_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
}

async fn login() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}

async fn register() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}

async fn logout() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}
