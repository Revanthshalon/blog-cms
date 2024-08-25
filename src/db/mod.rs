use sqlx::MySqlPool;

use crate::config::Config;

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new(config: &Config) -> Self {
        let database_url = config.get_database_url();
        let pool = MySqlPool::connect(database_url)
            .await
            .expect("Error establishing database connection");

        Database { pool }
    }

    pub fn get_pool(&self) -> MySqlPool {
        self.pool.clone()
    }
}
