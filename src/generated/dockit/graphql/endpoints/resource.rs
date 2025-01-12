use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::resource::*;
use uuid::Uuid;

#[derive(Default)]
pub struct ResourceQueries;

#[Object]
impl ResourceQueries {
    pub async fn get_resource_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Resource> {
        let resource = GQLContextWrapper::from(ctx)
            .resource_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource)
    }

    pub async fn get_one_resource(
        &self,
        ctx: &Context<'_>,
        input: SearchResourceInput,
    ) -> FieldResult<Resource> {
        let resource = GQLContextWrapper::from(ctx)
            .resource_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource)
    }

    pub async fn get_many_resources(
        &self,
        ctx: &Context<'_>,
        input: SearchResourceInput,
    ) -> FieldResult<Resource> {
        let resource = GQLContextWrapper::from(ctx)
            .resource_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource)
    }
}

#[derive(Default)]
pub struct ResourceMutations;

#[Object]
impl ResourceMutations {
    pub async fn create_resource(
        &self,
        ctx: &Context<'_>,
        input: CreateResourceInput,
    ) -> FieldResult<Resource> {
        let resource = GQLContextWrapper::from(ctx)
            .resource_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource)
    }

    pub async fn update_resource(
        &self,
        ctx: &Context<'_>,
        input: UpdateResourceInput,
    ) -> FieldResult<Resource> {
        let resource = GQLContextWrapper::from(ctx)
            .resource_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(resource)
    }
}
