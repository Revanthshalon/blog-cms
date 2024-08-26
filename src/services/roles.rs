use uuid::Uuid;

use crate::{
    models::{RoleListResponse, RoleResponse},
    repositories::RoleRepository,
};

#[derive(Debug, Clone)]
pub struct RoleService {
    role_repo: RoleRepository,
}

impl RoleService {
    pub fn new(role_repo: RoleRepository) -> Self {
        Self { role_repo }
    }
}

impl RoleService {
    // Find all roles
    pub async fn find_all(&self) -> Result<RoleListResponse, sqlx::Error> {
        self.role_repo.find_all().await
    }

    // Create role
    pub async fn create(
        &self,
        role_name: &str,
        description: &str,
    ) -> Result<RoleResponse, sqlx::Error> {
        self.role_repo.create(role_name, description).await
    }

    // Find role by id
    pub async fn find_by_id(&self, id: Uuid) -> Result<RoleResponse, sqlx::Error> {
        self.role_repo.find_by_id(id).await
    }

    // Update role by id
    pub async fn update_by_id(
        &self,
        id: Uuid,
        role_name: Option<String>,
        description: Option<String>,
    ) -> Result<RoleResponse, sqlx::Error> {
        self.role_repo.update(id, role_name, description).await
    }

    // Delete role by id
    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.role_repo.delete(id).await
    }
}
