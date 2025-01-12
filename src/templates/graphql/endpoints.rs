use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::{entity_name_lowercase}::*;
use uuid::Uuid;

#[derive(Default)]
pub struct {entity_name}Queries;

#[Object]
impl {entity_name}Queries {
    pub async fn get_{entity_name_lowercase}_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<{entity_name}> {
        let {entity_name_lowercase} = GQLContextWrapper::from(ctx)
            .{entity_name_lowercase}_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok({entity_name_lowercase})
    }

    pub async fn get_one_{entity_name_lowercase}(
        &self,
        ctx: &Context<'_>,
        input: Search{entity_name}Input,
    ) -> FieldResult<{entity_name}> {
        let {entity_name_lowercase} = GQLContextWrapper::from(ctx)
            .{entity_name_lowercase}_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok({entity_name_lowercase})
    }

    pub async fn get_many_{entity_name_lowercase}s(
        &self,
        ctx: &Context<'_>,
        input: Search{entity_name}Input,
    ) -> FieldResult<{entity_name}> {
        let {entity_name_lowercase} = GQLContextWrapper::from(ctx)
            .{entity_name_lowercase}_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok({entity_name_lowercase})
    }
}

#[derive(Default)]
pub struct {entity_name}Mutations;

#[Object]
impl {entity_name}Mutations {
    pub async fn create_{entity_name_lowercase}(
        &self,
        ctx: &Context<'_>,
        input: Create{entity_name}Input,
    ) -> FieldResult<{entity_name}> {
        let {entity_name_lowercase} = GQLContextWrapper::from(ctx)
            .{entity_name_lowercase}_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok({entity_name_lowercase})
    }

    pub async fn update_{entity_name_lowercase}(
        &self,
        ctx: &Context<'_>,
        input: Update{entity_name}Input,
    ) -> FieldResult<{entity_name}> {
        let {entity_name_lowercase} = GQLContextWrapper::from(ctx)
            .{entity_name_lowercase}_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok({entity_name_lowercase})
    }
}
