use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde_json::json;

pub async fn check_app_health() -> Response {
    let status_code = StatusCode::OK;
    let body = Json(json!({
        "status": StatusCode::OK.to_string(),
        "code": StatusCode::OK.as_u16(),
        "message": "App is running!",
        "timestamp": Utc::now()
    }));
    (status_code, body).into_response()
}
