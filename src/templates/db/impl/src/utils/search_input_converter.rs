use crate::{
    {entity_lowercase}::entity::{Column, Entity},
    utils::SearchFieldConverter,
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
            .apply_if(input.id, SearchFieldConverter::new(Column::Id).id())
{mapped_methods}
            .apply_if(
                input.created,
                SearchFieldConverter::new(Column::Created).date_time(),
            )
            .apply_if(
                input.updated,
                SearchFieldConverter::new(Column::Updated).date_time(),
            )
    }
}
