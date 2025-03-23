use axum::{
    Router,
    body::Body,
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::get,
};
use rust_embed::Embed;
use std::sync::Arc;

use super::{AppError, AppState};

#[derive(Embed)]
#[folder = "webui/dist/"]
struct SpaFile;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(serve_spa))
        .route("/{*path}", get(serve_spa))
}

fn try_file(path: &str) -> Option<Response> {
    match SpaFile::get(path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(path).first_or_octet_stream();
            let body = Body::from(content.data.to_vec());
            Response::builder()
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .body(body)
                .ok()
        }
        None => None,
    }
}

async fn serve_spa(uri: Uri) -> Result<impl IntoResponse, AppError> {
    // There's no reason to worry about path shenanigans here, as we aren't
    // accessing actual filesystem files, only embedded ones.
    let path = uri.path().trim_start_matches('/');
    let path_index = format!("{}index.html", path);
    let try_paths = vec![path, &path_index, "index.html"];

    for path in try_paths {
        if let Some(response) = try_file(path) {
            return Ok(response);
        }
    }

    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))?)
}
