use crate::entities::PostStatus;
use crate::models::{PostListResponse, PostResponse};
use crate::repositories::PostRepository;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostService {
    post_repo: PostRepository,
}

impl PostService {
    pub fn new(post_repo: PostRepository) -> Self {
        Self { post_repo }
    }
}

impl PostService {
    // Find all posts
    pub async fn find_all(&self) -> Result<PostListResponse, sqlx::Error> {
        self.post_repo.find_all().await
    }

    // Create Post
    pub async fn create(
        &self,
        title: String,
        content: String,
        status: PostStatus,
        published_at: Option<DateTime<Utc>>,
        user_id: Uuid,
    ) -> Result<PostResponse, sqlx::Error> {
        self.post_repo
            .create(title, content, status, published_at, user_id)
            .await
    }

    // Find post by id
    pub async fn find_by_id(&self, id: Uuid) -> Result<PostResponse, sqlx::Error> {
        self.post_repo.find_by_id(id).await
    }

    // Update post by id
    pub async fn update_by_id(
        &self,
        id: Uuid,
        title: Option<String>,
        content: Option<String>,
        status: Option<PostStatus>,
        published_at: Option<DateTime<Utc>>,
        user_id: Option<Uuid>,
    ) -> Result<PostResponse, sqlx::Error> {
        self.post_repo
            .update(id, title, content, status, published_at, user_id)
            .await
    }

    // Delete post by id
    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.post_repo.delete(id).await
    }

    // Find all posts by user id
    pub async fn find_all_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<PostListResponse, sqlx::Error> {
        self.post_repo.find_by_user_id(user_id).await
    }
}
