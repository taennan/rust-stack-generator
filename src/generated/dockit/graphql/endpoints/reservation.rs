use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::reservation::*;
use uuid::Uuid;

#[derive(Default)]
pub struct ReservationQueries;

#[Object]
impl ReservationQueries {
    pub async fn get_reservation_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Reservation> {
        let reservation = GQLContextWrapper::from(ctx)
            .reservation_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservation)
    }

    pub async fn get_one_reservation(
        &self,
        ctx: &Context<'_>,
        input: SearchReservationInput,
    ) -> FieldResult<Reservation> {
        let reservation = GQLContextWrapper::from(ctx)
            .reservation_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservation)
    }

    pub async fn get_many_reservations(
        &self,
        ctx: &Context<'_>,
        input: SearchReservationInput,
    ) -> FieldResult<Reservation> {
        let reservation = GQLContextWrapper::from(ctx)
            .reservation_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservation)
    }
}

#[derive(Default)]
pub struct ReservationMutations;

#[Object]
impl ReservationMutations {
    pub async fn create_reservation(
        &self,
        ctx: &Context<'_>,
        input: CreateReservationInput,
    ) -> FieldResult<Reservation> {
        let reservation = GQLContextWrapper::from(ctx)
            .reservation_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservation)
    }

    pub async fn update_reservation(
        &self,
        ctx: &Context<'_>,
        input: UpdateReservationInput,
    ) -> FieldResult<Reservation> {
        let reservation = GQLContextWrapper::from(ctx)
            .reservation_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservation)
    }
}
