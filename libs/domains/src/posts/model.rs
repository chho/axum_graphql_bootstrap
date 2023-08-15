use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

/// The post GraphQL and database model.
#[derive(SimpleObject, Default, Deserialize, Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
}
