use std::sync::Arc;

use anyhow::Result;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use axum_graphql_domains::{
    posts::{
        resolver::{PostMutation, PostQuery},
        service::{DefaultPostsService, PostsService},
    },
    DBPool,
};

/// GraphQL top-level query.
#[derive(MergedObject, Default)]
pub struct Query(PostQuery);

/// GraphQL top-level mutation.
#[derive(MergedObject, Default)]
pub struct Mutation(PostMutation);

/// The application's top-level schema alias.
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// `GraphQLSchema` initialization.
pub async fn get_schema(dbpool: Arc<DBPool>) -> Result<GraphQLSchema> {
    let postservice: Arc<dyn PostsService> = Arc::new(DefaultPostsService::new(dbpool));

    Ok(
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(postservice)
            .finish(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_schema() {
        let db_pool = Arc::new(DBPool::new_test(true).await.unwrap());

        assert!(get_schema(db_pool).await.is_ok());
    }
}
