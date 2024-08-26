use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::{UserListResponse, UserResponse};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        UserRepository { pool }
    }
}

impl UserRepository {
    // Find all users
    pub async fn find_all(&self) -> Result<UserListResponse, sqlx::Error> {
        let users = sqlx::query_as!(
            UserResponse,
            r#"
            SELECT id AS 'id:Uuid', username, email, role_id AS 'role_id:Uuid'
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(UserListResponse { users })
    }

    // Create User
    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password: &str,
        role_id: &str,
    ) -> Result<UserResponse, sqlx::Error> {
        let id = Uuid::new_v4();
        let id_bytes = id.as_bytes().to_vec();
        let role_id = Uuid::parse_str(role_id).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let role_id_bytes = role_id.as_bytes().to_vec();
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, password_hash, role_id)
            VALUES (?, ?, ?, ?, ?)
            "#,
            id_bytes,
            username,
            email,
            password,
            role_id_bytes
        )
        .execute(&self.pool)
        .await?;
        let response = UserResponse {
            id,
            username: username.to_string(),
            email: email.to_string(),
            role_id,
        };
        Ok(response)
    }

    // Find user by id
    pub async fn find_by_id(&self, id: &str) -> Result<UserResponse, sqlx::Error> {
        let id = Uuid::parse_str(id).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let id_bytes = id.as_bytes().to_vec();
        let user = sqlx::query_as!(
            UserResponse,
            r#"
            SELECT id AS 'id:Uuid', username, email, role_id AS 'role_id:Uuid'
            FROM users
            WHERE id = ?
            "#,
            id_bytes
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    // Update user
    pub async fn update(
        &self,
        id: &str,
        username: &str,
        email: &str,
        role_id: &str,
    ) -> Result<UserResponse, sqlx::Error> {
        // Check if the user exists
        self.find_by_id(id)
            .await
            .map_err(|_| sqlx::Error::RowNotFound)?;

        let id = Uuid::parse_str(id).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let id_bytes = id.as_bytes().to_vec();
        let role_id = Uuid::parse_str(role_id).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let role_id_bytes = role_id.as_bytes().to_vec();
        sqlx::query_as!(
            UserResponse,
            r#"
            UPDATE users
            SET username = COALESCE(?, username),
                email = COALESCE(?, email),
                role_id = COALESCE(?, role_id)
            WHERE id = ?
            "#,
            username,
            email,
            role_id_bytes,
            id_bytes
        )
        .execute(&self.pool)
        .await?;
        let user = UserResponse {
            id,
            username: username.to_string(),
            email: email.to_string(),
            role_id,
        };
        Ok(user)
    }

    // Delete user
    pub async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        // Check if the user exists
        self.find_by_id(id)
            .await
            .map_err(|_| sqlx::Error::RowNotFound)?;

        let id = Uuid::parse_str(id).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let id_bytes = id.as_bytes().to_vec();
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#,
            id_bytes
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
