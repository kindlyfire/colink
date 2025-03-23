use serde_json::{Value, json};

use crate::tests::{auth::create_test_user_and_login, setup_test_server};

#[tokio::test]
async fn test_posts_crud() {
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

    // Test pagination with offset - should return empty list
    let get_posts_paginated_response = server
        .get("/api/posts?offset=1")
        .add_cookie(session_cookie.clone())
        .await;
    get_posts_paginated_response.assert_status_ok();
    let body: Value = get_posts_paginated_response.json();
    let paginated_posts = body["data"].as_array().unwrap();
    assert_eq!(paginated_posts.len(), 0);

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

#[tokio::test]
async fn test_posts_search() {
    let (repo, server) = setup_test_server().await;
    let (_, session_cookie) =
        create_test_user_and_login(&repo, &server, "testuser", "password123").await;

    // Create first post
    let create_post1_response = server
        .post("/api/posts")
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "text": "This is a post about cats"
        }))
        .await;
    create_post1_response.assert_status_ok();
    let body: Value = create_post1_response.json();
    let post1 = &body["data"];
    let post1_id = post1["id"].as_str().unwrap();

    // Create second post
    let create_post2_response = server
        .post("/api/posts")
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "text": "This is another post about dogs"
        }))
        .await;
    create_post2_response.assert_status_ok();
    let body: Value = create_post2_response.json();
    let post2 = &body["data"];
    let post2_id = post2["id"].as_str().unwrap();

    // Search for "post" which should return both posts
    let search_posts_response = server
        .get("/api/posts/search?query=post")
        .add_cookie(session_cookie.clone())
        .await;
    search_posts_response.assert_status_ok();
    let body: Value = search_posts_response.json();
    let search_results = body["data"].as_array().unwrap();
    assert_eq!(search_results.len(), 2);

    // Verify both posts are in the results
    let result_ids: Vec<&str> = search_results
        .iter()
        .map(|post| post["id"].as_str().unwrap())
        .collect();
    assert!(result_ids.contains(&post1_id));
    assert!(result_ids.contains(&post2_id));

    // Search for "cats" which should return only the first post
    let search_cats_response = server
        .get("/api/posts/search?query=cats")
        .add_cookie(session_cookie.clone())
        .await;
    search_cats_response.assert_status_ok();
    let body: Value = search_cats_response.json();
    let search_results = body["data"].as_array().unwrap();
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0]["id"], post1_id);

    // Search for something that doesn't exist
    let search_none_response = server
        .get("/api/posts/search?query=nonexistent")
        .add_cookie(session_cookie.clone())
        .await;
    search_none_response.assert_status_ok();
    let body: Value = search_none_response.json();
    let search_results = body["data"].as_array().unwrap();
    assert_eq!(search_results.len(), 0);

    // Clean up by deleting the posts
    server
        .delete(&format!("/api/posts/{}", post1_id))
        .add_cookie(session_cookie.clone())
        .await;
    server
        .delete(&format!("/api/posts/{}", post2_id))
        .add_cookie(session_cookie.clone())
        .await;
}

#[tokio::test]
async fn test_post_links() {
    let (repo, server) = setup_test_server().await;
    let (_, session_cookie) =
        create_test_user_and_login(&repo, &server, "testuser", "password123").await;

    // Create a post with two links
    let post_text = "Check out these sites: https://example.com and https://test.org";
    let create_post_response = server
        .post("/api/posts")
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "text": post_text
        }))
        .await;
    create_post_response.assert_status_ok();
    let body: Value = create_post_response.json();
    let post = &body["data"];
    let post_id = post["id"].as_str().unwrap();

    // Verify both links were extracted and stored in the database
    let links = repo.links_by_post_id(post_id).await.unwrap();
    assert_eq!(links.len(), 2);

    // Create a set of URLs for easier comparison
    let link_urls: std::collections::HashSet<String> =
        links.iter().map(|link| link.url.clone()).collect();

    assert!(link_urls.contains("https://example.com"));
    assert!(link_urls.contains("https://test.org"));

    // Update the post to replace one link with a different one
    let updated_post_text = "Check out these sites: https://example.com and https://newsite.com";
    let update_post_response = server
        .post(&format!("/api/posts/{}", post_id))
        .add_cookie(session_cookie.clone())
        .json(&json!({
            "text": updated_post_text
        }))
        .await;
    update_post_response.assert_status_ok();

    // Verify links were updated correctly
    let updated_links = repo.links_by_post_id(post_id).await.unwrap();
    assert_eq!(updated_links.len(), 2);

    let updated_link_urls: std::collections::HashSet<String> =
        updated_links.iter().map(|link| link.url.clone()).collect();

    assert!(updated_link_urls.contains("https://example.com")); // This URL should still be there
    assert!(!updated_link_urls.contains("https://test.org")); // This URL should be gone
    assert!(updated_link_urls.contains("https://newsite.com")); // This URL should be new

    // Clean up by deleting the post
    server
        .delete(&format!("/api/posts/{}", post_id))
        .add_cookie(session_cookie.clone())
        .await;
}
