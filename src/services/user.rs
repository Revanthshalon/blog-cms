use uuid::Uuid;

use crate::{
    models::{UserListResponse, UserResponse},
    repositories::UserRepository,
};

#[derive(Debug, Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }
}

impl UserService {
    // Find all users
    pub async fn find_all(&self) -> Result<UserListResponse, sqlx::Error> {
        self.user_repo.find_all().await
    }

    // Create User
    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password: &str,
        role_id: Uuid,
    ) -> Result<UserResponse, sqlx::Error> {
        self.user_repo
            .create(username, email, password, role_id)
            .await
    }

    // Find user by id
    pub async fn find_by_id(&self, id: Uuid) -> Result<UserResponse, sqlx::Error> {
        self.user_repo.find_by_id(id).await
    }

    // Update user by id
    pub async fn update_by_id(
        &self,
        id: Uuid,
        username: &str,
        email: &str,
        password: &str,
        role_id: Uuid,
    ) -> Result<UserResponse, sqlx::Error> {
        self.user_repo.update(id, username, email, role_id).await
    }

    // Delete user by id
    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.user_repo.delete(id).await
    }
}
