use crate::{
    common::select::{SelectExactFilter, SelectRangedFilter},
    {entity_lowercase}::entity::{Column, Entity},
};
use {project_lower}_db_models::{entity_lowercase}::Search{entity}Input;
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr, QueryTrait, Select};

pub(crate) struct Search{entity}InputConverter(Search{entity}Input);

impl From<Search{entity}Input> for Search{entity}InputConverter {
    fn from(input: Search{entity}Input) -> Self {
        Self(input)
    }
}

impl From<Search{entity}InputConverter> for Select<Entity> {
    fn from(converter: Search{entity}InputConverter) -> Self {
        let input = converter.0;

        Entity::find()
            .apply_if(input.id, |select, id| {
                let expr = SimpleExpr::from(SelectExactFilter::new(Column::Id, id));
                select.filter(expr)
            })
{mapped_methods}
            .apply_if(input.created, |select, created| {
                let expr = SimpleExpr::from(SelectRangedFilter::new(Column::Created, created));
                select.filter(expr)
            })
            .apply_if(input.updated, |select, updated| {
                let expr = SimpleExpr::from(SelectRangedFilter::new(Column::Updated, updated));
                select.filter(expr)
            })
    }
}
