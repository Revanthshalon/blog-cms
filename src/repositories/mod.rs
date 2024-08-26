use sqlx::MySqlPool;

mod post;
mod role;
mod user;

pub use post::PostRepository;
pub use role::RoleRepository;
pub use user::UserRepository;

pub struct RepositoryContainer {
    pub role_repository: RoleRepository,
    pub user_repository: UserRepository,
    pub post_repository: PostRepository,
}

impl RepositoryContainer {
    pub fn new(pool: MySqlPool) -> Self {
        RepositoryContainer {
            role_repository: RoleRepository::new(pool.clone()),
            user_repository: UserRepository::new(pool.clone()),
            post_repository: PostRepository::new(pool.clone()),
        }
    }
}
