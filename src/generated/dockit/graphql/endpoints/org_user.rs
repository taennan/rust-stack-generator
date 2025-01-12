use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::org_user::*;
use uuid::Uuid;

#[derive(Default)]
pub struct OrgUserQueries;

#[Object]
impl OrgUserQueries {
    pub async fn get_org_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<OrgUser> {
        let org_user = GQLContextWrapper::from(ctx)
            .org_user_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(org_user)
    }

    pub async fn get_one_org_user(
        &self,
        ctx: &Context<'_>,
        input: SearchOrgUserInput,
    ) -> FieldResult<OrgUser> {
        let org_user = GQLContextWrapper::from(ctx)
            .org_user_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(org_user)
    }

    pub async fn get_many_org_users(
        &self,
        ctx: &Context<'_>,
        input: SearchOrgUserInput,
    ) -> FieldResult<OrgUser> {
        let org_user = GQLContextWrapper::from(ctx)
            .org_user_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(org_user)
    }
}

#[derive(Default)]
pub struct OrgUserMutations;

#[Object]
impl OrgUserMutations {
    pub async fn create_org_user(
        &self,
        ctx: &Context<'_>,
        input: CreateOrgUserInput,
    ) -> FieldResult<OrgUser> {
        let org_user = GQLContextWrapper::from(ctx)
            .org_user_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(org_user)
    }

    pub async fn update_org_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateOrgUserInput,
    ) -> FieldResult<OrgUser> {
        let org_user = GQLContextWrapper::from(ctx)
            .org_user_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(org_user)
    }
}
