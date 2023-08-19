use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// The post GraphQL and database model.
#[derive(SimpleObject, Default, Deserialize, Serialize, FromRow)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// The post GraphQL input model.
#[derive(InputObject, Default, Deserialize, Serialize)]
pub struct InputPost {
    pub title: String,
    pub content: String,
}
