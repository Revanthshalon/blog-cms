use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{models::CreateRole, services::ServiceContainer};

//Create a new role
pub async fn create_role(
    State(service): State<ServiceContainer>,
    Json(payload): Json<CreateRole>,
) -> Response {
    let role_result = service
        .role_service
        .create(&payload.role_name, &payload.description.unwrap_or_default())
        .await;
    if role_result.is_err() {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        let body = Json(json!({
            "error": "Internal Server Error",
            "message": "Failed to create role",
        }));
        return (status_code, body).into_response();
    }
    let role = role_result.unwrap();
    let status_code = StatusCode::CREATED;
    let body = Json(json!({
        "status": "created",
        "data": role,
    }));
    (status_code, body).into_response()
}
