use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::reservee::*;
use uuid::Uuid;

#[derive(Default)]
pub struct ReserveeQueries;

#[Object]
impl ReserveeQueries {
    pub async fn get_reservee_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Reservee> {
        let reservee = GQLContextWrapper::from(ctx)
            .reservee_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee)
    }

    pub async fn get_one_reservee(
        &self,
        ctx: &Context<'_>,
        input: SearchReserveeInput,
    ) -> FieldResult<Reservee> {
        let reservee = GQLContextWrapper::from(ctx)
            .reservee_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee)
    }

    pub async fn get_many_reservees(
        &self,
        ctx: &Context<'_>,
        input: SearchReserveeInput,
    ) -> FieldResult<Reservee> {
        let reservee = GQLContextWrapper::from(ctx)
            .reservee_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee)
    }
}

#[derive(Default)]
pub struct ReserveeMutations;

#[Object]
impl ReserveeMutations {
    pub async fn create_reservee(
        &self,
        ctx: &Context<'_>,
        input: CreateReserveeInput,
    ) -> FieldResult<Reservee> {
        let reservee = GQLContextWrapper::from(ctx)
            .reservee_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee)
    }

    pub async fn update_reservee(
        &self,
        ctx: &Context<'_>,
        input: UpdateReserveeInput,
    ) -> FieldResult<Reservee> {
        let reservee = GQLContextWrapper::from(ctx)
            .reservee_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee)
    }
}
