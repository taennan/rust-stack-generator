use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::reservee_user::*;
use uuid::Uuid;

#[derive(Default)]
pub struct ReserveeUserQueries;

#[Object]
impl ReserveeUserQueries {
    pub async fn get_reservee_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<ReserveeUser> {
        let reservee_user = GQLContextWrapper::from(ctx)
            .reservee_user_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee_user)
    }

    pub async fn get_one_reservee_user(
        &self,
        ctx: &Context<'_>,
        input: SearchReserveeUserInput,
    ) -> FieldResult<ReserveeUser> {
        let reservee_user = GQLContextWrapper::from(ctx)
            .reservee_user_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee_user)
    }

    pub async fn get_many_reservee_users(
        &self,
        ctx: &Context<'_>,
        input: SearchReserveeUserInput,
    ) -> FieldResult<ReserveeUser> {
        let reservee_user = GQLContextWrapper::from(ctx)
            .reservee_user_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee_user)
    }
}

#[derive(Default)]
pub struct ReserveeUserMutations;

#[Object]
impl ReserveeUserMutations {
    pub async fn create_reservee_user(
        &self,
        ctx: &Context<'_>,
        input: CreateReserveeUserInput,
    ) -> FieldResult<ReserveeUser> {
        let reservee_user = GQLContextWrapper::from(ctx)
            .reservee_user_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee_user)
    }

    pub async fn update_reservee_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateReserveeUserInput,
    ) -> FieldResult<ReserveeUser> {
        let reservee_user = GQLContextWrapper::from(ctx)
            .reservee_user_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(reservee_user)
    }
}
