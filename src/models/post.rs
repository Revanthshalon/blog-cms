use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::PostStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    pub status: PostStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    pub id: Uuid,
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<PostStatus>,
    pub published_at: Option<DateTime<Utc>>,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostListResponse {
    pub posts: Vec<PostResponse>,
}
