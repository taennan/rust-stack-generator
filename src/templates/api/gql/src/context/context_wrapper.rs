use crate::context::{{project_prefix}AppData, DBFactory};
use async_graphql::{Context, ErrorExtensions, FieldResult, Lookahead};
use core_services::{{imports}};

pub struct GQLContextWrapper<'a> {
    ctx: &'a Context<'a>,
}

impl<'a> From<&'a Context<'a>> for GQLContext<'a> {
    fn from(ctx: &'a Context<'a>) -> Self {
        Self { ctx }
    }
}

impl<'a> GQLContextWrapper<'a> {
{methods}

    //
    // Utils
    //

    fn db(&self) -> DBFactory {
        DBFactory::new(self.app_data().db_connection())
    }

    fn app_data(&self) -> {project_prefix}AppData {
        self.ctx.data_unchecked::<{project_prefix}AppData>().clone()
    }

    //fn lookahead(&self) -> Lookahead<'_> {
    //    self.ctx.look_ahead()
    //}
}
