use crate::{
    common::select::{SelectExactFilter, SelectRangedFilter},
    admin_api_key::entity::{Column, Entity},
};
use db_models::admin_api_key::SearchAdminApiKeyInput;
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr, QueryTrait, Select};

pub(crate) struct SearchAdminApiKeyInputConverter(SearchAdminApiKeyInput);

impl From<SearchAdminApiKeyInput> for SearchAdminApiKeyInputConverter {
    fn from(input: SearchAdminApiKeyInput) -> Self {
        Self(input)
    }
}

impl From<SearchAdminApiKeyInputConverter> for Select<Entity> {
    fn from(converter: SearchAdminApiKeyInputConverter) -> Self {
        let input = converter.0;

        Entity::find()
            .apply_if(input.id, |select, id| {
                let expr = SimpleExpr::from(SelectExactFilter::new(Column::Id, id));
                select.filter(expr)
            })

            .apply_if(input.hash, |select, hash| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::Hash, hash));
                select.filter(expression)
            })
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
