mod config;

pub async fn run_app() -> Result<(), std::io::Error> {
    // Load configuration
    let _config = Config::new();
    Ok(())
}
