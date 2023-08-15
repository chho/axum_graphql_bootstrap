use async_graphql::{Context, Object, Result};

use super::model::Post;

/// The query segment for the post model.
#[derive(Default)]
pub struct PostQuery {}

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
