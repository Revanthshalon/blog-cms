use crate::{
    entities::PostStatus,
    models::{PostListResponse, PostResponse},
};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostRepository {
    pool: MySqlPool,
}

impl PostRepository {
    pub fn new(pool: MySqlPool) -> Self {
        PostRepository { pool }
    }
}

impl PostRepository {
    // Find all posts
    pub async fn find_all(&self) -> Result<PostListResponse, sqlx::Error> {
        let posts = sqlx::query_as!(
            PostResponse,
            r#"
            SELECT id AS 'id:Uuid', title, content, user_id AS 'user_id:Uuid', status as 'status:PostStatus', published_at AS 'published_at:DateTime<Utc>'
            FROM posts
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(PostListResponse { posts })
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
        let id = Uuid::new_v4();
        let id_bytes = id.as_bytes().to_vec();
        let user_id_bytes = user_id.as_bytes().to_vec();
        sqlx::query!(
            r#"
            INSERT INTO posts (id, title, content, status, published_at, user_id)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            id_bytes,
            &title,
            &content,
            status.to_str(),
            &published_at,
            user_id_bytes
        )
            .execute(&self.pool)
            .await?;
        let response = PostResponse {
            id,
            title,
            content,
            status,
            published_at,
            user_id,
        };
        Ok(response)
    }

    // Find post by id
    pub async fn find_by_id(&self, id: Uuid) -> Result<PostResponse, sqlx::Error> {
        let post = sqlx::query_as!(
            PostResponse,
            r#"
            SELECT id AS 'id:Uuid', title, content, user_id AS 'user_id:Uuid', status as 'status:PostStatus', published_at AS 'published_at:DateTime<Utc>'
            FROM posts
            WHERE id = ?
            "#,
            id.as_bytes().to_vec()
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(post)
    }

    // Find Post by user_id
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<PostListResponse, sqlx::Error> {
        let posts = sqlx::query_as!(
            PostResponse,
            r#"
            SELECT id AS 'id:Uuid', title, content, user_id AS 'user_id:Uuid', status as 'status:PostStatus', published_at AS 'published_at:DateTime<Utc>'
            FROM posts
            WHERE user_id = ?
            "#,
            user_id.as_bytes().to_vec()
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(PostListResponse { posts })
    }

    // Update Post
    pub async fn update(
        &self,
        id: Uuid,
        title: Option<String>,
        content: Option<String>,
        status: Option<PostStatus>,
        published_at: Option<DateTime<Utc>>,
        user_id: Option<Uuid>,
    ) -> Result<PostResponse, sqlx::Error> {
        let status = status.unwrap();
        let user_id = user_id.unwrap();
        sqlx::query!(
            r#"
            UPDATE posts
            SET
                title = COALESCE(?, title),
                content = COALESCE(?, content),
                status = COALESCE(?, status),
                published_at = COALESCE(?, published_at),
                user_id = COALESCE(?, user_id)
            WHERE id = ?
            "#,
            title,
            content,
            status.to_str(),
            published_at,
            Some(user_id.as_bytes().to_vec()),
            id.as_bytes().to_vec()
        )
            .execute(&self.pool)
            .await?;
        let response = self.find_by_id(id).await?;
        Ok(response)
    }

    // Delete Post
    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = ?
            "#,
            id.as_bytes().to_vec()
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
