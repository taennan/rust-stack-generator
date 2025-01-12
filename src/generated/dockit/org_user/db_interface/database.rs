use crate::{
    error::DBResult,
    org_user::{CreateOrgUserInput, OrgUser, SearchOrgUserInput, UpdateOrgUserInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait OrgUserDB {
    async fn create(&self, input: CreateOrgUserInput) -> DBResult<OrgUser>;

    async fn get_one(&self, input: SearchOrgUserInput) -> DBResult<Option<OrgUser>>;

    async fn get_many(&self, input: SearchOrgUserInput) -> DBResult<Vec<OrgUser>>;

    async fn update(&self, input: UpdateOrgUserInput) -> DBResult<OrgUser>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
