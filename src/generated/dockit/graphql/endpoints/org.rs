use crate::{utils::GQLContextWrapper, error::EndpointError};
use async_graphql::{Context, FieldResult, Object};
use core_services_interface::org::*;
use uuid::Uuid;

#[derive(Default)]
pub struct OrgQueries;

#[Object]
impl OrgQueries {
    pub async fn get_org_by_id(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Org> {
        let org = GQLContextWrapper::from(ctx)
            .org_service()
            .get_by_id(id)
            .await
            .map_err(EndpointError::from)?;
        Ok(org)
    }

    pub async fn get_one_org(
        &self,
        ctx: &Context<'_>,
        input: SearchOrgInput,
    ) -> FieldResult<Org> {
        let org = GQLContextWrapper::from(ctx)
            .org_service()
            .get_one(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(org)
    }

    pub async fn get_many_orgs(
        &self,
        ctx: &Context<'_>,
        input: SearchOrgInput,
    ) -> FieldResult<Org> {
        let org = GQLContextWrapper::from(ctx)
            .org_service()
            .get_many(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(org)
    }
}

#[derive(Default)]
pub struct OrgMutations;

#[Object]
impl OrgMutations {
    pub async fn create_org(
        &self,
        ctx: &Context<'_>,
        input: CreateOrgInput,
    ) -> FieldResult<Org> {
        let org = GQLContextWrapper::from(ctx)
            .org_service()
            .create(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(org)
    }

    pub async fn update_org(
        &self,
        ctx: &Context<'_>,
        input: UpdateOrgInput,
    ) -> FieldResult<Org> {
        let org = GQLContextWrapper::from(ctx)
            .org_service()
            .update(input)
            .await
            .map_err(EndpointError::from)?;
        Ok(org)
    }
}
