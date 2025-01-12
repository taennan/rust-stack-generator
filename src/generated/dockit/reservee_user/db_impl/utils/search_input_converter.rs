use crate::{
    common::select::{SelectExactFilter, SelectRangedFilter},
    reservee_user::entity::{Column, Entity},
};
use db_models::reservee_user::SearchReserveeUserInput;
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr, QueryTrait, Select};

pub(crate) struct SearchReserveeUserInputConverter(SearchReserveeUserInput);

impl From<SearchReserveeUserInput> for SearchReserveeUserInputConverter {
    fn from(input: SearchReserveeUserInput) -> Self {
        Self(input)
    }
}

impl From<SearchReserveeUserInputConverter> for Select<Entity> {
    fn from(converter: SearchReserveeUserInputConverter) -> Self {
        let input = converter.0;

        Entity::find()
            .apply_if(input.id, |select, id| {
                let expr = SimpleExpr::from(SelectExactFilter::new(Column::Id, id));
                select.filter(expr)
            })

            .apply_if(input.claw_auth_id, |select, claw_auth_id| {
                let expression = SimpleExpr::from(SelectExactFilter::new(Column::ClawAuthId, claw_auth_id));
                select.filter(expression)
            })
            .apply_if(input.email, |select, email| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::Email, email));
                select.filter(expression)
            })
            .apply_if(input.description, |select, description| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::Description, description));
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
