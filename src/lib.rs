use config::Config;
use db::Database;

mod config;
mod db;
mod entities;
mod models;

pub async fn run_app() -> Result<(), std::io::Error> {
    // Load configuration
    let config = Config::new();

    // Create database connection
    let _db = Database::new(&config).await;
    Ok(())
}
