pub struct Config {
    database_url: String,
    host: String,
}

impl Config {
    pub fn new() -> Self {
        use dotenvy::dotenv;
        use std::env;

        // Load environment variables from .env file
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let host = env::var("HOST").expect("HOST must be set");

        Config { database_url, host }
    }

    pub fn get_database_url(&self) -> &str {
        &self.database_url
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }
}
