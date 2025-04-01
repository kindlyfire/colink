use axum_extra::extract::cookie::Cookie;
use axum_test::TestServer;
use bcrypt::hash;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{
    db::{IdType, models::users, now_utc, repository::Repository},
    tests::setup_test_server,
};

pub(super) async fn create_test_user(
    repo: &Repository,
    username: &str,
    password: &str,
) -> users::Model {
    let hashed_password = hash(password, 4).unwrap();
    let now = now_utc();

    let user = users::ActiveModel {
        id: Set(IdType::User.create()),
        created_at: Set(now.clone()),
        updated_at: Set(now),
        username: Set(username.to_string()),
        password: Set(hashed_password),
    };

    user.insert(&repo.conn).await.unwrap()
}

pub(super) async fn create_test_user_and_login(
    repo: &Repository,
    server: &TestServer,
    username: &str,
    password: &str,
) -> (users::Model, Cookie<'static>) {
    let test_user = create_test_user(repo, username, password).await;

    let login_response = server
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "username": username,
            "password": password
        }))
        .await;
    login_response.assert_status_ok();

    let cookies = login_response.cookies();
    let session_cookie = cookies
        .get("colink_session")
        .expect("Session cookie not found after login");

    (test_user, session_cookie.clone())
}

#[tokio::test]
async fn test_auth() {
    let (repo, mut server) = setup_test_server().await;
    let (test_user, session_cookie) =
        create_test_user_and_login(&repo, &server, "testuser", "password123").await;
    server.add_cookie(session_cookie.clone());

    let response = server.get("/api/auth/me").await;

    response.assert_status_ok();
    let user_data: serde_json::Value = response.json();
    assert_eq!(user_data["id"], test_user.id);
    assert_eq!(user_data["username"], "testuser");
    assert!(user_data["created_at"].is_string());
    assert!(user_data["updated_at"].is_string());
}
