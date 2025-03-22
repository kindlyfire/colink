use axum_test::TestServer;
use bcrypt::hash;
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{Value, json};

use crate::{
    db::{
        IdType,
        models::{self, users},
        now_utc,
        repository::Repository,
    },
    routes::{AppState, get_router},
};

async fn setup_test_server() -> (TestServer, Repository) {
    let repo = Repository::new().await.unwrap();
    let app_state = AppState { repo: repo.clone() };
    let app = get_router(app_state);
    let server = TestServer::new(app).unwrap();
    (server, repo)
}

async fn create_test_user(repo: &Repository, username: &str, password: &str) -> users::Model {
    let hashed_password = hash(password, 4).unwrap();
    let now = now_utc();

    let user = models::users::ActiveModel {
        id: Set(IdType::User.create()),
        created_at: Set(now.clone()),
        updated_at: Set(now),
        username: Set(username.to_string()),
        password: Set(hashed_password),
    };

    user.insert(&repo.conn).await.unwrap()
}

#[tokio::test]
async fn test_post_endpoints() {
    let (server, repo) = setup_test_server().await;

    let test_user = create_test_user(&repo, "testuser", "password123").await;
    let login_response = server
        .post("/auth/login")
        .json(&json!({
            "username": "testuser",
            "password": "password123"
        }))
        .await;
    login_response.assert_status_ok();

    // Extract the session cookie
    let cookies = login_response.cookies();
    let session_cookie = cookies.get("colink_session").unwrap();

    // Check there's no posts
    let get_posts_response = server
        .get("/posts")
        .add_cookie(session_cookie.clone())
        .await;
    get_posts_response.assert_status_ok();
    let body: Value = get_posts_response.json();
    let posts = body["data"].as_array().unwrap();
    assert_eq!(posts.len(), 0);

    // Create a post
    let create_post_response = server
        .post("/posts")
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "text": "This is a test post"
        }))
        .await;
    create_post_response.assert_status_ok();
    let body: Value = create_post_response.json();
    let post = &body["data"];
    assert_eq!(post["text"], "This is a test post");
    assert_eq!(post["user_id"], test_user.id);
    let post_id = post["id"].as_str().unwrap();

    // Check posts again
    let get_posts_response = server
        .get("/posts")
        .add_cookie(session_cookie.clone())
        .await;
    get_posts_response.assert_status_ok();
    let body: Value = get_posts_response.json();
    let posts = body["data"].as_array().unwrap();
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0]["id"], post_id);
    assert_eq!(posts[0]["text"], "This is a test post");
    assert_eq!(posts[0]["user_id"], test_user.id);

    // Update the post
    let update_post_response = server
        .post(&format!("/posts/{}", post_id))
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "text": "This is an updated post"
        }))
        .await;
    update_post_response.assert_status_ok();
    let body: Value = update_post_response.json();
    let updated_post = &body["data"];
    assert_eq!(updated_post["text"], "This is an updated post");
    assert_eq!(updated_post["id"], post_id);

    // Get post by ID and verify update
    let get_post_response = server
        .get(&format!("/posts/{}", post_id))
        .add_cookie(session_cookie.clone())
        .await;
    get_post_response.assert_status_ok();
    let body: Value = get_post_response.json();
    let post = &body["data"];
    assert_eq!(post["text"], "This is an updated post");
    assert_eq!(post["id"], post_id);

    // Delete the post
    let delete_post_response = server
        .delete(&format!("/posts/{}", post_id))
        .add_cookie(session_cookie.clone())
        .await;
    delete_post_response.assert_status_ok();
    let body: Value = delete_post_response.json();
    assert_eq!(body["success"], true);

    // Verify post was deleted by checking posts list is empty
    let get_posts_response = server
        .get("/posts")
        .add_cookie(session_cookie.clone())
        .await;
    get_posts_response.assert_status_ok();
    let body: Value = get_posts_response.json();
    let posts = body["data"].as_array().unwrap();
    assert_eq!(posts.len(), 0);
}
