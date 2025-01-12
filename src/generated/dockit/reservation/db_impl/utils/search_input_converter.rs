use crate::{
    common::select::{SelectExactFilter, SelectRangedFilter},
    reservation::entity::{Column, Entity},
};
use db_models::reservation::SearchReservationInput;
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr, QueryTrait, Select};

pub(crate) struct SearchReservationInputConverter(SearchReservationInput);

impl From<SearchReservationInput> for SearchReservationInputConverter {
    fn from(input: SearchReservationInput) -> Self {
        Self(input)
    }
}

impl From<SearchReservationInputConverter> for Select<Entity> {
    fn from(converter: SearchReservationInputConverter) -> Self {
        let input = converter.0;

        Entity::find()
            .apply_if(input.id, |select, id| {
                let expr = SimpleExpr::from(SelectExactFilter::new(Column::Id, id));
                select.filter(expr)
            })

            .apply_if(input.org_id, |select, org_id| {
                let expression = SimpleExpr::from(SelectExactFilter::new(Column::OrgId, org_id));
                select.filter(expression)
            })
            .apply_if(input.reservee_id, |select, reservee_id| {
                let expression = SimpleExpr::from(SelectExactFilter::new(Column::ReserveeId, reservee_id));
                select.filter(expression)
            })
            .apply_if(input.status, |select, status| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::Status, status));
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
