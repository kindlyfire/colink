use serde_json::{Value, json};

use crate::tests::{auth::create_test_user_and_login, setup_test_server};

#[tokio::test]
async fn test_post_endpoints() {
    let (repo, server) = setup_test_server().await;
    let (test_user, session_cookie) =
        create_test_user_and_login(&repo, &server, "testuser", "password123").await;

    // Check there's no posts
    let get_posts_response = server
        .get("/api/posts")
        .add_cookie(session_cookie.clone())
        .await;
    get_posts_response.assert_status_ok();
    let body: Value = get_posts_response.json();
    let posts = body["data"].as_array().unwrap();
    assert_eq!(posts.len(), 0);

    // Create a post
    let create_post_response = server
        .post("/api/posts")
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
        .get("/api/posts")
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
        .post(&format!("/api/posts/{}", post_id))
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
        .get(&format!("/api/posts/{}", post_id))
        .add_cookie(session_cookie.clone())
        .await;
    get_post_response.assert_status_ok();
    let body: Value = get_post_response.json();
    let post = &body["data"];
    assert_eq!(post["text"], "This is an updated post");
    assert_eq!(post["id"], post_id);

    // Delete the post
    let delete_post_response = server
        .delete(&format!("/api/posts/{}", post_id))
        .add_cookie(session_cookie.clone())
        .await;
    delete_post_response.assert_status_ok();
    let body: Value = delete_post_response.json();
    assert_eq!(body["success"], true);

    // Verify post was deleted by checking posts list is empty
    let get_posts_response = server
        .get("/api/posts")
        .add_cookie(session_cookie.clone())
        .await;
    get_posts_response.assert_status_ok();
    let body: Value = get_posts_response.json();
    let posts = body["data"].as_array().unwrap();
    assert_eq!(posts.len(), 0);
}
