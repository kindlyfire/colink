use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
};

mod auth;
mod links;
mod posts;

pub(crate) fn get_router() -> Router {
    Router::new()
        .nest("/auth", auth::get_router())
        .nest("/links", links::get_router())
        .nest("/posts", posts::get_router())
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // TODO: Better error handling
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
