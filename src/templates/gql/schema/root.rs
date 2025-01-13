use crate::schema::{
    {object_imports}
};
use actix_web::{
    guard,
    web::{self, Data},
    HttpRequest, HttpResponse, Resource,
};
use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[derive(MergedObject, Default)]
pub struct RootQueries(
{queries}
);

#[derive(MergedObject, Default)]
pub struct RootMutations(
{mutations}
);

pub type {project_prefix}Schema = Schema<RootQueries, RootMutations, EmptySubscription>;

impl From<{project_prefix}AppData> for {project_prefix}Schema {
    fn from(app_data: {project_prefix}AppData) -> Self {
        Self::build(
            RootQueries::default(),
            RootMutations::default(),
            EmptySubscription,
        )
        .data(app_data)
        .finish()
    }
}