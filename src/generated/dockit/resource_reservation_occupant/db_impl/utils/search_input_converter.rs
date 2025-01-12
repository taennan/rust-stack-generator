use crate::{
    common::select::{SelectExactFilter, SelectRangedFilter},
    resource_reservation_occupant::entity::{Column, Entity},
};
use db_models::resource_reservation_occupant::SearchResourceReservationOccupantInput;
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr, QueryTrait, Select};

pub(crate) struct SearchResourceReservationOccupantInputConverter(SearchResourceReservationOccupantInput);

impl From<SearchResourceReservationOccupantInput> for SearchResourceReservationOccupantInputConverter {
    fn from(input: SearchResourceReservationOccupantInput) -> Self {
        Self(input)
    }
}

impl From<SearchResourceReservationOccupantInputConverter> for Select<Entity> {
    fn from(converter: SearchResourceReservationOccupantInputConverter) -> Self {
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
            .apply_if(input.resource_reservation_id, |select, resource_reservation_id| {
                let expression = SimpleExpr::from(SelectExactFilter::new(Column::ResourceReservationId, resource_reservation_id));
                select.filter(expression)
            })
            .apply_if(input.occupant_id, |select, occupant_id| {
                let expression = SimpleExpr::from(SelectExactFilter::new(Column::OccupantId, occupant_id));
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
