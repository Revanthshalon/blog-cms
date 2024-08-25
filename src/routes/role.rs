use axum::{routing::post, Router};

use crate::{handlers::create_role, services::ServiceContainer};

pub fn create_role_routes(services: ServiceContainer) -> Router {
    Router::new()
        .route("/", post(create_role))
        .with_state(services)
}
