use roles::RoleService;
use user::UserService;

use crate::repositories::RepositoryContainer;

mod roles;
mod user;

#[derive(Debug, Clone)]
pub struct ServiceContainer {
    pub role_service: RoleService,
    pub user_service: UserService,
}

impl ServiceContainer {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        ServiceContainer {
            role_service: RoleService::new(repository_container.role_repository),
            user_service: UserService::new(repository_container.user_repository),
        }
    }
}
