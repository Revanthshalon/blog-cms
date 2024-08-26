use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::{create_role, delete_role_by_id, get_role_by_id, get_roles, update_role_by_id},
    services::ServiceContainer,
};

pub fn create_role_routes(services: ServiceContainer) -> Router {
    Router::new()
        .route("/", post(create_role))
        .route("/", get(get_roles))
        .route("/:id", get(get_role_by_id))
        .route("/:id", put(update_role_by_id))
        .route("/", delete(delete_role_by_id))
        .with_state(services)
}
