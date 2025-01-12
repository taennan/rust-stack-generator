use async_graphql::{Context, ErrorExtensions, FieldResult, Lookahead};
use db_postgres::{{db_imports}};
use core_services::{{service_imports}};
use sea_orm::DatabaseConnection;

pub struct GQLContextWrapper<'a> {
    ctx: &'a Context<'a>,
}

impl<'a> From<&'a Context<'a>> for GQLContext<'a> {
    fn from(ctx: &'a Context<'a>) -> Self {
        Self { ctx }
    }
}

impl<'a> GQLContextWrapper<'a> {
    fn lookahead(&self) -> Lookahead<'_> {
        self.ctx.look_ahead()
    }

    fn db_connection(&self) -> DatabaseConnection {
        self.ctx.data_unchecked::<DatabaseConnection>().clone()
    }

{methods}
}
