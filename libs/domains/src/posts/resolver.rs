use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use chrono::Utc;

use super::{
    model::{InputPost, Post},
    service::PostsService,
};

/// The query segment for the post model.
#[derive(Default)]
pub struct PostQuery {}

/// The mutation segment for the post model.
#[derive(Default)]
pub struct PostMutation {}

/// Queries for the `Post` model.
#[Object]
impl PostQuery {
    async fn get_post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The post id")] id: String,
    ) -> Result<Post> {
        info!("get_post");

        let shows = ctx.data_unchecked::<Arc<dyn PostsService>>();

        Ok(shows.get(id.as_str()).await?)
        // Ok(Some(Post {
        //     id: "111".to_string(),
        //     title: "testing".to_string(),
        //     content: "testing body".to_string(),
        //     created_at: Utc::now(),
        //     updated_at: Utc::now(),
        // }))
    }
}

/// Mutations for the `Post` model.
#[Object]
impl PostMutation {
    /// Creates a new post.
    async fn create_post(&self, _ctx: &Context<'_>, post: InputPost) -> Result<Option<Post>> {
        Ok(Some(Post {
            id: "222".to_string(),
            title: "xxxx".to_string(),
            content: "yyyy".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }))
    }
}
