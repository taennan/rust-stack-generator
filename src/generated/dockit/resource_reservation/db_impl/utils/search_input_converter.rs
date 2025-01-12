use crate::{
    common::select::{SelectExactFilter, SelectRangedFilter},
    resource_reservation::entity::{Column, Entity},
};
use db_models::resource_reservation::SearchResourceReservationInput;
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr, QueryTrait, Select};

pub(crate) struct SearchResourceReservationInputConverter(SearchResourceReservationInput);

impl From<SearchResourceReservationInput> for SearchResourceReservationInputConverter {
    fn from(input: SearchResourceReservationInput) -> Self {
        Self(input)
    }
}

impl From<SearchResourceReservationInputConverter> for Select<Entity> {
    fn from(converter: SearchResourceReservationInputConverter) -> Self {
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
            .apply_if(input.resource_id, |select, resource_id| {
                let expression = SimpleExpr::from(SelectExactFilter::new(Column::ResourceId, resource_id));
                select.filter(expression)
            })
            .apply_if(input.reservation_id, |select, reservation_id| {
                let expression = SimpleExpr::from(SelectExactFilter::new(Column::ReservationId, reservation_id));
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
