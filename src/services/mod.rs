use roles::RoleService;

use crate::repositories::RepositoryContainer;

mod roles;

#[derive(Debug, Clone)]
pub struct ServiceContainer {
    pub role_service: RoleService,
}

impl ServiceContainer {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        ServiceContainer {
            role_service: RoleService::new(repository_container.role_repository),
        }
    }
}
