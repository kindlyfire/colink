use std::{fmt::Display, sync::Arc};

use axum::{
    Json, Router,
    extract::{FromRequestParts, State},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use serde_json::{Value, json};
use tracing::error;

use crate::db::{models::users, repository::Repository};

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
    pub repo: Repository,
}

pub type ExtractAppState = State<Arc<AppState>>;

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
        write!(f, "AppError: {}", self.status_code)?;
        if let Some(message) = &self.message {
            write!(f, " - {}", message)?;
        }
        if let Some(body) = &self.body {
            write!(f, " - {:?}", body)?;
        }
        Ok(())
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
        let err: anyhow::Error = err.into();

        match err.downcast::<AppError>() {
            Ok(err) => err,
            Err(err) => {
                error!("Internal server error: {:?}", err);
                Self {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: Some(format!("Something went wrong: {}", err)),
                    body: None,
                }
            }
        }
    }
}

struct ExtractUser(pub users::Model);

impl FromRequestParts<Arc<AppState>> for ExtractUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // Get the cookie jar from the request
        let jar = CookieJar::from_request_parts(parts, &())
            .await
            .map_err(|_| AppError::new(StatusCode::UNAUTHORIZED, "Failed to extract cookies"))?;

        // Check if session cookie exists
        let session_token = jar
            .get("colink_session")
            .ok_or(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Session cookie not found",
            ))?
            .value();

        // Find the session
        let session = state
            .repo
            .session_by_token(session_token)
            .await?
            .ok_or(AppError::new(StatusCode::UNAUTHORIZED, "Session not found"))?;

        // Find the user
        let user = state
            .repo
            .user_by_id(&session.user_id)
            .await?
            .ok_or(AppError::new(StatusCode::UNAUTHORIZED, "User not found"))?;

        // Return the authenticated user
        Ok(Self(user))
    }
}
