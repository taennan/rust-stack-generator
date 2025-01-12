use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::admin_api_key::*;
use uuid::Uuid;

#[derive(Default)]
pub struct AdminApiKeyQueries;

#[Object]
impl AdminApiKeyQueries {
    pub async fn get_admin_api_key_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<AdminApiKey> {
        let admin_api_key = GQLContextWrapper::from(ctx)
            .admin_api_key_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(admin_api_key)
    }

    pub async fn get_one_admin_api_key(
        &self,
        ctx: &Context<'_>,
        input: SearchAdminApiKeyInput,
    ) -> FieldResult<AdminApiKey> {
        let admin_api_key = GQLContextWrapper::from(ctx)
            .admin_api_key_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(admin_api_key)
    }

    pub async fn get_many_admin_api_keys(
        &self,
        ctx: &Context<'_>,
        input: SearchAdminApiKeyInput,
    ) -> FieldResult<AdminApiKey> {
        let admin_api_key = GQLContextWrapper::from(ctx)
            .admin_api_key_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(admin_api_key)
    }
}

#[derive(Default)]
pub struct AdminApiKeyMutations;

#[Object]
impl AdminApiKeyMutations {
    pub async fn create_admin_api_key(
        &self,
        ctx: &Context<'_>,
        input: CreateAdminApiKeyInput,
    ) -> FieldResult<AdminApiKey> {
        let admin_api_key = GQLContextWrapper::from(ctx)
            .admin_api_key_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(admin_api_key)
    }

    pub async fn update_admin_api_key(
        &self,
        ctx: &Context<'_>,
        input: UpdateAdminApiKeyInput,
    ) -> FieldResult<AdminApiKey> {
        let admin_api_key = GQLContextWrapper::from(ctx)
            .admin_api_key_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(admin_api_key)
    }
}
