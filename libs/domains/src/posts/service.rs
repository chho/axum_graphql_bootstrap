use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::DBPool;

use super::model::{InputPost, Post};

/// PostsService is an interface for interacting with posts.
#[async_trait]
pub trait PostsService: Sync + Send {
    /// Get an individual `Post` by id.
    async fn get(&self, id: &str) -> Result<Post>;

    /// Create a `Post` with the given data.
    async fn create(&self, post: InputPost) -> Result<Post>;
}

/// DefaultPostsService is a default implementation of `PostsService`.
pub struct DefaultPostsService {
    /// The database connection.
    db: Arc<DBPool>,
}

/// The default `PostsService` implementation.
impl DefaultPostsService {
    pub fn new(db: Arc<DBPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PostsService for DefaultPostsService {
    async fn get(&self, id: &str) -> Result<Post> {
        let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db.0)
            .await?;

        Ok(post)
    }

    async fn create(&self, inputpost: InputPost) -> Result<Post> {
        let post = sqlx::query_as::<_, Post>("INSERT INTO posts (id, title, content, created_at, updated_at) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING *")
            .bind(Uuid::new_v4().to_string()).bind(inputpost.title).bind(inputpost.content)
            .fetch_one(&self.db.0)
            .await?;

        Ok(post)
    }
}
