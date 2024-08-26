use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::{CreateRole, UpdateRole},
    services::ServiceContainer,
};

//Create a new role
pub async fn create_role(
    State(service): State<ServiceContainer>,
    Json(payload): Json<CreateRole>,
) -> Response {
    let role_result = service
        .role_service
        .create(&payload.role_name, &payload.description.unwrap_or_default())
        .await;
    match role_result {
        Ok(role) => {
            let status_code = StatusCode::CREATED;
            let body = Json(json!({
                "status": StatusCode::CREATED.to_string(),
                "code": StatusCode::CREATED.as_u16(),
                "message": "Role created successfully",
                "data": role,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to create role",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

//Get all roles
pub async fn get_roles(State(service): State<ServiceContainer>) -> Response {
    let roles_result = service.role_service.find_all().await;
    match roles_result {
        Ok(roles) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "Roles fetched successfully",
                "data": roles,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to get roles",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Get role by id
pub async fn get_role_by_id(
    State(service): State<ServiceContainer>,
    Path(id): Path<String>,
) -> Response {
    let role_id_result = Uuid::parse_str(&id);
    let role_id = match role_id_result {
        Ok(role_id) => role_id,
        Err(e) => {
            let status_code = StatusCode::BAD_REQUEST;
            let body = Json(json!({
                "status": StatusCode::BAD_REQUEST.to_string(),
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "message": "Invalid role id",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            return (status_code, body).into_response();
        }
    };
    let role_result = service.role_service.find_by_id(role_id).await;
    match role_result {
        Ok(role) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "Role fetched successfully",
                "data": role,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to get role",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Update role by id
pub async fn update_role_by_id(
    State(service): State<ServiceContainer>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateRole>,
) -> Response {
    let role_id_result = Uuid::parse_str(&id);
    let role_id = match role_id_result {
        Ok(role_id) => role_id,
        Err(e) => {
            let status_code = StatusCode::BAD_REQUEST;
            let body = Json(json!({
                "status": StatusCode::BAD_REQUEST.to_string(),
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "message": "Invalid role id",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            return (status_code, body).into_response();
        }
    };
    let role_result = service
        .role_service
        .update_by_id(role_id, payload.role_name, payload.description)
        .await;
    match role_result {
        Ok(role) => {
            let status_code = StatusCode::OK;
            let body = Json(json!({
                "status": StatusCode::OK.to_string(),
                "code": StatusCode::OK.as_u16(),
                "message": "Role updated successfully",
                "data": role,
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Failed to update role",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}

// Delete role by id
pub async fn delete_role_by_id(
    State(services): State<ServiceContainer>,
    Path(id): Path<String>,
) -> Response {
    let role_id_result = Uuid::parse_str(&id);
    let role_id = match role_id_result {
        Ok(role_id) => role_id,
        Err(e) => {
            let status_code = StatusCode::BAD_REQUEST;
            let body = Json(json!({
                "status": StatusCode::BAD_REQUEST.to_string(),
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "message": "Invalid role id",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            return (status_code, body).into_response();
        }
    };
    let role_result = services.role_service.find_by_id(role_id).await;
    match role_result {
        Ok(role) => {
            let delete_result = services.role_service.delete_by_id(role_id).await;
            match delete_result {
                Ok(_) => {
                    let status_code = StatusCode::OK;
                    let body = Json(json!({
                        "status": StatusCode::OK.to_string(),
                        "code": StatusCode::OK.as_u16(),
                        "message": "Role deleted successfully",
                        "data": role,
                        "timestamp": Utc::now(),
                    }));
                    (status_code, body).into_response()
                }
                Err(e) => {
                    let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                    let body = Json(json!({
                        "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "message": "Failed to delete role",
                        "errors": e.to_string(),
                        "timestamp": Utc::now(),
                    }));
                    (status_code, body).into_response()
                }
            }
        }
        Err(e) => {
            let status_code = StatusCode::INTERNAL_SERVER_ERROR;
            let body = Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Role not found",
                "errors": e.to_string(),
                "timestamp": Utc::now(),
            }));
            (status_code, body).into_response()
        }
    }
}
