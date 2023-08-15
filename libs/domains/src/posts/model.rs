use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

/// The post GraphQL and database model.
#[derive(SimpleObject, Default, Deserialize, Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
}

/// The post GraphQL input model.
#[derive(InputObject, Default, Deserialize, Serialize)]
pub struct InputPost {
    pub id: String,
    pub title: String,
    pub body: String,
}
