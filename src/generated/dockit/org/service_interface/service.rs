use crate::{
    common::DeleteOutput,
    org::{
        CreateOrgInput, Org, SearchManyOrgsInput, SearchOrgInput,
        UpdateOrgInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait OrgService {
    async fn create(&self, input: CreateOrgInput) -> DockitResult<Org>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Org>;

    async fn get_one(&self, input: SearchOrgInput) -> DockitResult<Option<Org>>;

    async fn get_many(&self, input: SearchManyOrgsInput) -> DockitResult<Vec<Org>>;

    async fn update(&self, input: UpdateOrgInput) -> DockitResult<Org>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
