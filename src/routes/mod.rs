use axum::Router;
use health::create_health_routes;
use role::create_role_routes;

use crate::services::ServiceContainer;

mod health;
mod role;
mod user;

pub fn create_api_routes(services: ServiceContainer) -> Router {
    let role_routes = Router::new().nest("/role", create_role_routes(services.clone()));
    let health_routes = Router::new().nest("/health", create_health_routes());
    let user_routes = Router::new().nest("/user", user::create_user_routes(services.clone()));
    let merged_routes = Router::new()
        .merge(role_routes)
        .merge(health_routes)
        .merge(user_routes);
    Router::new().nest("/api", merged_routes)
}
