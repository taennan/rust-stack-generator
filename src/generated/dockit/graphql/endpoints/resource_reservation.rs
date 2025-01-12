use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::resource_reservation::*;
use uuid::Uuid;

#[derive(Default)]
pub struct ResourceReservationQueries;

#[Object]
impl ResourceReservationQueries {
    pub async fn get_resource_reservation_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<ResourceReservation> {
        let resource_reservation = GQLContextWrapper::from(ctx)
            .resource_reservation_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation)
    }

    pub async fn get_one_resource_reservation(
        &self,
        ctx: &Context<'_>,
        input: SearchResourceReservationInput,
    ) -> FieldResult<ResourceReservation> {
        let resource_reservation = GQLContextWrapper::from(ctx)
            .resource_reservation_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation)
    }

    pub async fn get_many_resource_reservations(
        &self,
        ctx: &Context<'_>,
        input: SearchResourceReservationInput,
    ) -> FieldResult<ResourceReservation> {
        let resource_reservation = GQLContextWrapper::from(ctx)
            .resource_reservation_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation)
    }
}

#[derive(Default)]
pub struct ResourceReservationMutations;

#[Object]
impl ResourceReservationMutations {
    pub async fn create_resource_reservation(
        &self,
        ctx: &Context<'_>,
        input: CreateResourceReservationInput,
    ) -> FieldResult<ResourceReservation> {
        let resource_reservation = GQLContextWrapper::from(ctx)
            .resource_reservation_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation)
    }

    pub async fn update_resource_reservation(
        &self,
        ctx: &Context<'_>,
        input: UpdateResourceReservationInput,
    ) -> FieldResult<ResourceReservation> {
        let resource_reservation = GQLContextWrapper::from(ctx)
            .resource_reservation_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation)
    }
}
