use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SimpleObject};
use axum_graphql_domains::posts::resolver::PostQuery;
// use axum_graphql_domains::posts::resolver::PostQuery;

/// GraphQL top-level query.
#[derive(MergedObject, Default)]
pub struct Query(PostQuery);

/// GraphQL top-level mutation.
// #[derive(MergedObject, Default)]
// pub struct Mutation();

/// The application's top-level schema alias.
// pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;
pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// `GraphQLSchema` initialization.
pub async fn get_schema() -> Result<GraphQLSchema> {
    // Ok(Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish())
    Ok(Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish())
}
