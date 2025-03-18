use std::{fmt::Display, sync::Arc};

use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};

mod auth;
mod links;
mod posts;

pub(crate) fn get_router(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::get_router())
        .nest("/links", links::get_router())
        .nest("/posts", posts::get_router())
        .with_state(Arc::new(state))
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

#[derive(Debug, Clone)]
pub struct AppError {
    status_code: StatusCode,
    message: Option<String>,
    body: Option<Value>,
}

#[allow(unused)]
impl AppError {
    pub fn new(status_code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status_code,
            message: Some(message.into()),
            body: None,
        }
    }

    pub fn with_json(status_code: StatusCode, body: Value) -> Self {
        Self {
            status_code,
            message: None,
            body: Some(body),
        }
    }

    pub fn with_status_only(status_code: StatusCode) -> Self {
        Self {
            status_code,
            message: None,
            body: None,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError: {} - {:?}", self.status_code, self.message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match (self.message, self.body) {
            // Custom JSON body
            (_, Some(body)) => (self.status_code, Json(body)).into_response(),

            // Error message as JSON
            (Some(message), _) => {
                (self.status_code, Json(json!({ "error": message }))).into_response()
            }

            // Empty response with just status code
            (None, None) => self.status_code.into_response(),
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        let error: anyhow::Error = err.into();

        match error.downcast::<AppError>() {
            Ok(app_error) => app_error,
            Err(error) => Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(format!("Something went wrong: {}", error)),
                body: None,
            },
        }
    }
}
