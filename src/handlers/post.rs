use crate::models::{CreatePost, UpdatePost};
use crate::services::ServiceContainer;
use axum::extract::Path;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

// Create a new post
pub async fn create_post(
    State(service): State<ServiceContainer>,
    Json(payload): Json<CreatePost>,
) -> Response {
    let post_result = service
        .post_service
        .create(
            payload.title,
            payload.content,
            payload.status,
            payload.published_at,
            payload.user_id,
        )
        .await;
    match post_result {
        Ok(post) => {
            let status_code = StatusCode::CREATED;
            let body = Json(json!({
                "status": StatusCode::CREATED.to_string(),
                "code": StatusCode::CREATED.as_u16(),
                "message": "Post created successfully",
                "data": post,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to create post",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Find all posts
pub async fn get_posts(State(service): State<ServiceContainer>) -> Response {
    let posts_result = service.post_service.find_all().await;
    match posts_result {
        Ok(posts) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "Posts retrieved successfully",
                "data": posts,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to retrieve posts",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Find a post by id
pub async fn get_post_by_id(
    State(service): State<ServiceContainer>,
    Path(id): Path<Uuid>,
) -> Response {
    let post_result = service.post_service.find_by_id(id).await;
    match post_result {
        Ok(post) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "Post retrieved successfully",
                "data": post,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to retrieve post",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Update a post by id
pub async fn update_post_by_id(
    State(service): State<ServiceContainer>,
    Json(payload): Json<UpdatePost>,
) -> Response {
    let post_result = service
        .post_service
        .update_by_id(
            payload.id,
            payload.title,
            payload.content,
            payload.status,
            payload.published_at,
            payload.user_id,
        )
        .await;
    match post_result {
        Ok(post) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "Post updated successfully",
                "data": post,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to update post",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Delete a post by id
pub async fn delete_post_by_id(
    State(service): State<ServiceContainer>,
    Path(id): Path<Uuid>,
) -> Response {
    let post_result = service.post_service.delete_by_id(id).await;
    match post_result {
        Ok(_) => {
            let status_code = StatusCode::NO_CONTENT;
            let body = Json(json!({
                "status": StatusCode::NO_CONTENT.to_string(),
                "code": StatusCode::NO_CONTENT.as_u16(),
                "message": "Post deleted successfully",
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to delete post",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Find all posts by user id
pub async fn get_posts_by_user_id(
    State(service): State<ServiceContainer>,
    Path(id): Path<Uuid>,
) -> Response {
    let posts_result = service.post_service.find_all_by_user_id(id).await;
    match posts_result {
        Ok(posts) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "Posts retrieved successfully",
                "data": posts,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to retrieve posts",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}