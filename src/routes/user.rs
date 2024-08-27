use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::{create_user, delete_user_by_id, get_posts_by_user_id, get_user_by_id, get_users, update_user_by_id},
    services::ServiceContainer,
};

pub fn create_user_routes(services: ServiceContainer) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_users))
        .route("/:id", get(get_user_by_id))
        .route("/:id", put(update_user_by_id))
        .route("/:id", delete(delete_user_by_id))
        .route("/:id/posts", get(get_posts_by_user_id))
        .with_state(services)
}
