use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FromRow<'_, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id_bytes: Vec<u8> = row.try_get("id")?;
        let id = Uuid::from_slice(&id_bytes).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let role_id_bytes: Vec<u8> = row.try_get("role_id")?;
        let role_id =
            Uuid::from_slice(&role_id_bytes).map_err(|e| sqlx::Error::Decode(e.into()))?;
        Ok(Self {
            id,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            role_id,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
