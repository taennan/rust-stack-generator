use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::occupant::*;
use uuid::Uuid;

#[derive(Default)]
pub struct OccupantQueries;

#[Object]
impl OccupantQueries {
    pub async fn get_occupant_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Occupant> {
        let occupant = GQLContextWrapper::from(ctx)
            .occupant_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(occupant)
    }

    pub async fn get_one_occupant(
        &self,
        ctx: &Context<'_>,
        input: SearchOccupantInput,
    ) -> FieldResult<Occupant> {
        let occupant = GQLContextWrapper::from(ctx)
            .occupant_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(occupant)
    }

    pub async fn get_many_occupants(
        &self,
        ctx: &Context<'_>,
        input: SearchOccupantInput,
    ) -> FieldResult<Occupant> {
        let occupant = GQLContextWrapper::from(ctx)
            .occupant_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(occupant)
    }
}

#[derive(Default)]
pub struct OccupantMutations;

#[Object]
impl OccupantMutations {
    pub async fn create_occupant(
        &self,
        ctx: &Context<'_>,
        input: CreateOccupantInput,
    ) -> FieldResult<Occupant> {
        let occupant = GQLContextWrapper::from(ctx)
            .occupant_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(occupant)
    }

    pub async fn update_occupant(
        &self,
        ctx: &Context<'_>,
        input: UpdateOccupantInput,
    ) -> FieldResult<Occupant> {
        let occupant = GQLContextWrapper::from(ctx)
            .occupant_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(occupant)
    }
}
