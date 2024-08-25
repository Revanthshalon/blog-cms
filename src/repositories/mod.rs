use sqlx::MySqlPool;

mod role;
pub use role::RoleRepository;

pub struct RepositoryContainer {
    pub role_repository: RoleRepository,
}

impl RepositoryContainer {
    pub fn new(pool: MySqlPool) -> Self {
        RepositoryContainer {
            role_repository: RoleRepository::new(pool.clone()),
        }
    }
}
