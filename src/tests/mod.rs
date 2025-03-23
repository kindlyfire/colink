use axum_test::TestServer;

use crate::{
    db::repository::Repository,
    routes::{AppState, get_router},
    search::Search,
};

mod auth;
mod posts;

async fn setup_test_server() -> (Repository, TestServer) {
    let repo = Repository::new().await.unwrap();
    let app_state = AppState {
        repo: repo.clone(),
        search: Some(Search::new().await.unwrap()),
    };
    let app = get_router(app_state);
    let server = TestServer::new(app).unwrap();
    (repo, server)
}
