use crate::{
    common::DeleteOutput,
    org_user::{
        CreateOrgUserInput, OrgUser, SearchManyOrgUsersInput, SearchOrgUserInput,
        UpdateOrgUserInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait OrgUserService {
    async fn create(&self, input: CreateOrgUserInput) -> DockitResult<OrgUser>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<OrgUser>;

    async fn get_one(&self, input: SearchOrgUserInput) -> DockitResult<Option<OrgUser>>;

    async fn get_many(&self, input: SearchManyOrgUsersInput) -> DockitResult<Vec<OrgUser>>;

    async fn update(&self, input: UpdateOrgUserInput) -> DockitResult<OrgUser>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
