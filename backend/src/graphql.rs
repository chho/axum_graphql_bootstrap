use anyhow::Result;
use async_graphql::{EmptySubscription, MergedObject, Schema};

/// GraphQL top-level query.
#[derive(MergedObject, Default)]
pub struct Query();

/// GraphQL top-level mutation.
#[derive(MergedObject, Default)]
pub struct Mutation();

/// The application's top-level schema alias.
pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// `GraphQLSchema` initialization.
pub async fn get_schema() -> Result<GraphQLSchema> {
    Ok(Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish())
}
