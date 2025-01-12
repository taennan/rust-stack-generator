use crate::{
    error::DBResult,
    org::{CreateOrgInput, Org, SearchOrgInput, UpdateOrgInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait OrgDB {
    async fn create(&self, input: CreateOrgInput) -> DBResult<Org>;

    async fn get_one(&self, input: SearchOrgInput) -> DBResult<Option<Org>>;

    async fn get_many(&self, input: SearchOrgInput) -> DBResult<Vec<Org>>;

    async fn update(&self, input: UpdateOrgInput) -> DBResult<Org>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
