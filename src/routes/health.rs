use axum::{routing::get, Router};

use crate::handlers::check_app_health;

pub fn create_health_routes() -> Router {
    Router::new().route("/health", get(check_app_health))
}
