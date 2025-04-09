use std::{collections::HashSet, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{AppError, AppState, ExtractAppState, ExtractUser};
use crate::db::{
    IdType,
    models::{links, post_links, posts},
    now_utc,
};

#[derive(Deserialize)]
struct CreatePostRequest {
    text: String,
}

pub(crate) fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_posts).post(create_post))
        .route("/search", get(get_posts_search))
        .route(
            "/{id}",
            get(get_post_by_id)
                .post(update_post_by_id)
                .delete(delete_post_by_id),
        )
}

#[derive(Deserialize)]
struct GetPostsParams {
    limit: Option<u64>,
    offset: Option<u64>,
}

async fn get_posts(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Query(params): Query<GetPostsParams>,
) -> Result<Json<Value>, AppError> {
    let limit = params.limit.unwrap_or(10).clamp(1, 500);
    let offset = params.offset.unwrap_or(0);

    let total = posts::Entity::find()
        .filter(posts::Column::UserId.eq(user.id.clone()))
        .count(&state.repo.conn)
        .await?;

    let posts = posts::Entity::find()
        .find_with_related(post_links::Entity)
        .filter(posts::Column::UserId.eq(user.id.clone()))
        .order_by_desc(posts::Column::CreatedAt)
        .limit(limit)
        .offset(offset)
        .all(&state.repo.conn)
        .await?;

    let link_ids = posts
        .iter()
        .flat_map(|(_post, links)| links.iter().map(|link| link.link_id.clone()))
        .collect::<HashSet<_>>();
    let all_links = links::Entity::find()
        .filter(links::Column::Id.is_in(link_ids))
        .all(&state.repo.conn)
        .await?;

    let posts = posts
        .iter()
        .map(|(post, links)| {
            let mut json = json!(post);
            let links = all_links
                .iter()
                .filter(|link| links.iter().any(|l| l.link_id == link.id))
                .collect::<Vec<_>>();
            json["links"] = json!(links);
            json
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({
        "data": posts,
        "total": total
    })))
}

#[derive(Deserialize)]
struct SearchPostsParams {
    query: String,
    limit: Option<usize>,
    offset: Option<usize>,
}

async fn get_posts_search(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Query(params): Query<SearchPostsParams>,
) -> Result<Json<Value>, AppError> {
    if state.get_search().is_none() {
        return Err(AppError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "Search service not available",
        ));
    }
    let search = state.get_search().unwrap();

    let search_params = crate::search::PostSearch {
        query: params.query,
        user_id: Some(user.id.clone()),
        offset: params.offset,
        limit: params.limit,
    };
    let posts = search
        .post_search(&search_params)
        .await
        .map_err(|err| AppError::new(StatusCode::SERVICE_UNAVAILABLE, err.to_string()))?;

    Ok(Json(json!({
        "data": posts
    })))
}

async fn create_post(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<Value>, AppError> {
    let now = now_utc();

    let post = posts::ActiveModel {
        id: Set(IdType::Post.create()),
        created_at: Set(now.clone()),
        updated_at: Set(now),
        user_id: Set(user.id.clone()),
        text: Set(payload.text),
    };

    let post = post.insert(&state.repo.conn).await?;
    post.update_links(&state.repo).await?;

    if let Some(search) = state.get_search() {
        search.post_upsert((&post).try_into()?).await?;
    }

    Ok(Json(json!({ "data": post })))
}

async fn get_post_by_id(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let post = state
        .repo
        .user_post_by_id(&id, &user.id)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Post not found"))?;

    Ok(Json(json!({ "data": post })))
}

#[derive(Deserialize)]
struct UpdatePostRequest {
    text: String,
}

async fn update_post_by_id(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<Value>, AppError> {
    let post = state
        .repo
        .user_post_by_id(&id, &user.id)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Post not found"))?;

    let mut post = post.into_active_model();
    post.text = Set(payload.text);
    post.updated_at = Set(now_utc());

    let post = post.update(&state.repo.conn).await?;
    post.update_links(&state.repo).await?;

    if let Some(search) = state.get_search() {
        search.post_upsert((&post).try_into()?).await?;
    }

    Ok(Json(json!({ "data": post })))
}

async fn delete_post_by_id(
    State(state): ExtractAppState,
    ExtractUser(user): ExtractUser,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let post = state
        .repo
        .user_post_by_id(&id, &user.id)
        .await?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Post not found"))?;
    let post_id = post.id.clone();
    post.delete(&state.repo.conn).await?;

    if let Some(search) = state.get_search() {
        search.post_delete(&post_id).await?;
    }

    Ok(Json(json!({ "success": true })))
}
