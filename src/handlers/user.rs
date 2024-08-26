use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::DEFAULT_COST;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::{CreateUser, UpdateUser},
    services::ServiceContainer,
};

//Create a new user
pub async fn create_user(
    State(service): State<ServiceContainer>,
    Json(payload): Json<CreateUser>,
) -> Response {
    let password_hash = match bcrypt::hash(&payload.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to hash password",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            return (status_code, body).into_response();
        }
    };
    let user_result = service
        .user_service
        .create(
            &payload.username,
            &payload.email,
            &password_hash,
            payload.role_id,
        )
        .await;
    match user_result {
        Ok(user) => {
            let status_code = StatusCode::CREATED;
            let body = Json(json!({
                "status": StatusCode::CREATED.to_string(),
                "code": StatusCode::CREATED.as_u16(),
                "message": "User created successfully",
                "data": user,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to create user",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Get all users
pub async fn get_users(State(service): State<ServiceContainer>) -> Response {
    let user_result = service.user_service.find_all().await;
    match user_result {
        Ok(users) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "Users fetched successfully",
                "data": users,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to fetch users",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Get a user by id
pub async fn get_user_by_id(
    State(service): State<ServiceContainer>,
    Path(id): Path<String>,
) -> Response {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => {
            let status_code = StatusCode::BAD_REQUEST;
            let body = Json(json!({
                "status": StatusCode::BAD_REQUEST.to_string(),
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "message": "Invalid user id",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            return (status_code, body).into_response();
        }
    };
    let user_result = service.user_service.find_by_id(id).await;
    match user_result {
        Ok(user) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "User fetched successfully",
                "data": user,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to fetch user",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Update a user by id
pub async fn update_user_by_id(
    State(service): State<ServiceContainer>,
    Json(payload): Json<UpdateUser>,
) -> Response {
    let user_result = service
        .user_service
        .update_by_id(
            payload.id,
            &payload.username,
            &payload.email,
            payload.role_id,
        )
        .await;
    match user_result {
        Ok(user) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "User updated successfully",
                "data": user,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to update user",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Delete a user by id
pub async fn delete_user_by_id(
    State(service): State<ServiceContainer>,
    Path(id): Path<String>,
) -> Response {
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => {
            let status_code = StatusCode::BAD_REQUEST;
            let body = Json(json!({
                "status": StatusCode::BAD_REQUEST.to_string(),
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "message": "Invalid user id",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            return (status_code, body).into_response();
        }
    };
    let user_result = service.user_service.delete_by_id(id).await;
    match user_result {
        Ok(user) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "User deleted successfully",
                "data": user,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to delete user",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}
