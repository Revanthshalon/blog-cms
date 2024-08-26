use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
    Published,
    Archived,
}

impl PostStatus {
    pub fn to_str(&self) -> &str {
        match self {
            PostStatus::Draft => "draft",
            PostStatus::Published => "published",
            PostStatus::Archived => "archived",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_id: Uuid,
    pub status: PostStatus,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FromRow<'_, MySqlRow> for Post {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        let id_bytes: Vec<u8> = row.try_get("id")?;
        let id = Uuid::from_slice(&id_bytes).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let user_id_bytes: Vec<u8> = row.try_get("user_id")?;
        let user_id =
            Uuid::from_slice(&user_id_bytes).map_err(|e| sqlx::Error::Decode(e.into()))?;
        let status = match row.try_get("status")? {
            "draft" => PostStatus::Draft,
            "published" => PostStatus::Published,
            "archived" => PostStatus::Archived,
            e => return Err(sqlx::Error::Decode(e.into())),
        };
        Ok(Self {
            id,
            title: row.try_get("title")?,
            content: row.try_get("content")?,
            user_id,
            status,
            published_at: row.try_get("published_at")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
