use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::{RoleListResponse, RoleResponse};

pub struct RoleRepository {
    pool: MySqlPool,
}

impl RoleRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl RoleRepository {
    // Find all roles
    pub async fn find_all(&self) -> Result<RoleListResponse, sqlx::Error> {
        let roles = sqlx::query_as!(
            RoleResponse,
            r#"
            SELECT id AS 'id:Uuid', role_name, description
            FROM roles
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(RoleListResponse { roles })
    }

    // Find role by id
    pub async fn create(
        &self,
        role_name: &str,
        description: &str,
    ) -> Result<RoleResponse, sqlx::Error> {
        let id = Uuid::new_v4();
        let id_bytes = id.as_bytes().to_vec();
        sqlx::query!(
            r#"
            INSERT INTO roles (id, role_name, description)
            VALUES (?, ?, ?)
            "#,
            id_bytes,
            role_name,
            description
        )
        .execute(&self.pool)
        .await?;
        let response = RoleResponse {
            id,
            role_name: role_name.to_string(),
            description: Some(description.to_string()),
        };
        Ok(response)
    }

    // Find role by id
    pub async fn find_by_id(&self, id: Uuid) -> Result<RoleResponse, sqlx::Error> {
        let role = sqlx::query_as!(
            RoleResponse,
            r#"
            SELECT id AS 'id:Uuid', role_name, description
            FROM roles
            WHERE id = ?
            "#,
            id.as_bytes().to_vec()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(role)
    }

    // Update role by id
    pub async fn update(
        &self,
        id: Uuid,
        role_name: &str,
        description: &str,
    ) -> Result<RoleResponse, sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE roles
            SET role_name = ?, description = ?
            WHERE id = ?
            "#,
            role_name,
            description,
            id.as_bytes().to_vec()
        )
        .execute(&self.pool)
        .await?;
        let response = RoleResponse {
            id,
            role_name: role_name.to_string(),
            description: Some(description.to_string()),
        };
        Ok(response)
    }

    // Delete role by id
    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM roles
            WHERE id = ?
            "#,
            id.as_bytes().to_vec()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
