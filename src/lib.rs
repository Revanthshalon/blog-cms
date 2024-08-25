use config::Config;
use db::Database;

mod config;
mod db;
mod entities;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

pub async fn run_app() -> Result<(), std::io::Error> {
    // Load configuration
    let config = Config::new();

    // Create database connection
    let db = Database::new(&config).await;

    let repository_container = repositories::RepositoryContainer::new(db.get_pool());
    let service_container = services::ServiceContainer::new(repository_container);

    let app_routes = routes::create_api_routes(service_container);
    let listener = tokio::net::TcpListener::bind(config.get_host())
        .await
        .expect("Error binding to port");

    axum::serve(listener, app_routes.into_make_service()).await
}
