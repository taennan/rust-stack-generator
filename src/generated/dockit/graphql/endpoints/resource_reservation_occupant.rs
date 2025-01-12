use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::resource_reservation_occupant::*;
use uuid::Uuid;

#[derive(Default)]
pub struct ResourceReservationOccupantQueries;

#[Object]
impl ResourceReservationOccupantQueries {
    pub async fn get_resource_reservation_occupant_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<ResourceReservationOccupant> {
        let resource_reservation_occupant = GQLContextWrapper::from(ctx)
            .resource_reservation_occupant_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation_occupant)
    }

    pub async fn get_one_resource_reservation_occupant(
        &self,
        ctx: &Context<'_>,
        input: SearchResourceReservationOccupantInput,
    ) -> FieldResult<ResourceReservationOccupant> {
        let resource_reservation_occupant = GQLContextWrapper::from(ctx)
            .resource_reservation_occupant_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation_occupant)
    }

    pub async fn get_many_resource_reservation_occupants(
        &self,
        ctx: &Context<'_>,
        input: SearchResourceReservationOccupantInput,
    ) -> FieldResult<ResourceReservationOccupant> {
        let resource_reservation_occupant = GQLContextWrapper::from(ctx)
            .resource_reservation_occupant_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation_occupant)
    }
}

#[derive(Default)]
pub struct ResourceReservationOccupantMutations;

#[Object]
impl ResourceReservationOccupantMutations {
    pub async fn create_resource_reservation_occupant(
        &self,
        ctx: &Context<'_>,
        input: CreateResourceReservationOccupantInput,
    ) -> FieldResult<ResourceReservationOccupant> {
        let resource_reservation_occupant = GQLContextWrapper::from(ctx)
            .resource_reservation_occupant_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation_occupant)
    }

    pub async fn update_resource_reservation_occupant(
        &self,
        ctx: &Context<'_>,
        input: UpdateResourceReservationOccupantInput,
    ) -> FieldResult<ResourceReservationOccupant> {
        let resource_reservation_occupant = GQLContextWrapper::from(ctx)
            .resource_reservation_occupant_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource_reservation_occupant)
    }
}
