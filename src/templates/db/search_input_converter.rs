use crate::{
    common::select::{SelectExactFilter, SelectRangedFilter},
    {entity_name_lowercase}::entity::{Column, Entity},
};
use db_models::{entity_name_lowercase}::Search{entity_name}Input;
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr, QueryTrait, Select};

pub(crate) struct Search{entity_name}InputConverter(Search{entity_name}Input);

impl From<Search{entity_name}Input> for Search{entity_name}InputConverter {
    fn from(input: Search{entity_name}Input) -> Self {
        Self(input)
    }
}

impl From<Search{entity_name}InputConverter> for Select<Entity> {
    fn from(converter: Search{entity_name}InputConverter) -> Self {
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
