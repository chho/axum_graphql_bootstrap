use async_graphql::{Context, Object, Result};

use super::model::{InputPost, Post};

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
        _ctx: &Context<'_>,
        #[graphql(desc = "The post id")] _id: String,
    ) -> Result<Option<Post>> {
        Ok(Some(Post {
            id: "111".to_string(),
            title: "testing".to_string(),
            body: "testing body".to_string(),
        }))
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
            body: "yyyy".to_string(),
        }))
    }
}
