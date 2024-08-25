use axum::Router;

use crate::services::ServiceContainer;

mod role;

pub fn create_api_routes(services: ServiceContainer) -> Router {
    let role_routes = Router::new().nest("/role", role::create_role_routes(services.clone()));
    let merged_routes = Router::new().merge(role_routes);
    Router::new().nest("/api", merged_routes)
}
