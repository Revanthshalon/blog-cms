use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::{create_post, delete_post_by_id, get_post_by_id, get_posts, update_post_by_id},
    services::ServiceContainer,
};

pub fn create_post_routes(services: ServiceContainer) -> Router {
    Router::new()
        .route("/", post(create_post))
        .route("/", get(get_posts))
        .route("/:id", get(get_post_by_id))
        .route("/:id", put(update_post_by_id))
        .route("/:id", delete(delete_post_by_id))
        .with_state(services)
}
