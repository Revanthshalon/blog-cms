use roles::RoleService;
use user::UserService;

use crate::repositories::RepositoryContainer;
use crate::services::post::PostService;

mod post;
mod roles;
mod user;

#[derive(Debug, Clone)]
pub struct ServiceContainer {
    pub role_service: RoleService,
    pub user_service: UserService,
    pub post_service: PostService,
}

impl ServiceContainer {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        ServiceContainer {
            role_service: RoleService::new(repository_container.role_repository),
            user_service: UserService::new(repository_container.user_repository),
            post_service: PostService::new(repository_container.post_repository),
        }
    }
}
