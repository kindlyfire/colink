use std::sync::Arc;

use axum::{
    Json, Router,
    routing::{get, post},
};
use serde_json::Value;

use super::{AppError, AppState};

pub(crate) fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_links))
        .route("/{id}", post(create_or_update_link))
}

async fn get_links() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}

async fn create_or_update_link() -> Result<Json<Value>, AppError> {
    Err(anyhow::anyhow!("Not implemented.").into())
}
