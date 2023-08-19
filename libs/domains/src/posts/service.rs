use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::DBPool;

use super::model::Post;

/// PostsService is an interface for interacting with posts.
#[async_trait]
pub trait PostsService: Sync + Send {
    /// Get an individual `Post` by id.
    async fn get(&self, id: &str) -> Result<Post>;
}

/// DefaultPostsService is a default implementation of `PostsService`.
pub struct DefaultPostsService {
    /// The database connection.
    db: Arc<DBPool>,
}

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
}
