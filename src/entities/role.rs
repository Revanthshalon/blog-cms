use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub role_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FromRow<'_, MySqlRow> for Role {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id_bytes: Vec<u8> = row.try_get("id")?;
        let id = Uuid::from_slice(&id_bytes).map_err(|e| sqlx::Error::Decode(e.into()))?;
        Ok(Self {
            id,
            role_name: row.try_get("role_name")?,
            description: row.try_get("description")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
