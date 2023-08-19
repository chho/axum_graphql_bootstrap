use std::sync::Arc;

use async_graphql::{Context, Object, Result};

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
        let posts = ctx.data_unchecked::<Arc<dyn PostsService>>();

        Ok(posts.get(id.as_str()).await?)
    }
}

/// Mutations for the `Post` model.
#[Object]
impl PostMutation {
    /// Creates a new post.
    async fn create_post(&self, ctx: &Context<'_>, inputpost: InputPost) -> Result<Post> {
        let posts = ctx.data_unchecked::<Arc<dyn PostsService>>();

        Ok(posts.create(inputpost).await?)
    }
}
