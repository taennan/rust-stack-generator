use crate::{
    common::select::{SelectExactFilter, SelectRangedFilter},
    reservee::entity::{Column, Entity},
};
use db_models::reservee::SearchReserveeInput;
use sea_orm::{entity::prelude::*, sea_query::SimpleExpr, QueryTrait, Select};

pub(crate) struct SearchReserveeInputConverter(SearchReserveeInput);

impl From<SearchReserveeInput> for SearchReserveeInputConverter {
    fn from(input: SearchReserveeInput) -> Self {
        Self(input)
    }
}

impl From<SearchReserveeInputConverter> for Select<Entity> {
    fn from(converter: SearchReserveeInputConverter) -> Self {
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
            .apply_if(input.reservee_user_id, |select, reservee_user_id| {
                let expression = SimpleExpr::from(SelectExactFilter::new(Column::ReserveeUserId, reservee_user_id));
                select.filter(expression)
            })
            .apply_if(input.first_name, |select, first_name| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::FirstName, first_name));
                select.filter(expression)
            })
            .apply_if(input.middle_names, |select, middle_names| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::MiddleNames, middle_names));
                select.filter(expression)
            })
            .apply_if(input.last_name, |select, last_name| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::LastName, last_name));
                select.filter(expression)
            })
            .apply_if(input.email, |select, email| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::Email, email));
                select.filter(expression)
            })
            .apply_if(input.phone, |select, phone| {
                let expression = SimpleExpr::from(SelectIterableFilter::new(Column::Phone, phone));
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
