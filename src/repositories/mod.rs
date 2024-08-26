use sqlx::MySqlPool;

mod role;
mod user;

pub use role::RoleRepository;
pub use user::UserRepository;

pub struct RepositoryContainer {
    pub role_repository: RoleRepository,
    pub user_repository: UserRepository,
}

impl RepositoryContainer {
    pub fn new(pool: MySqlPool) -> Self {
        RepositoryContainer {
            role_repository: RoleRepository::new(pool.clone()),
            user_repository: UserRepository::new(pool.clone()),
        }
    }
}
