use crate::{
    common::DeleteOutput,
    resource::{
        CreateResourceInput, Resource, SearchManyResourcesInput, SearchResourceInput,
        UpdateResourceInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ResourceService {
    async fn create(&self, input: CreateResourceInput) -> DockitResult<Resource>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Resource>;

    async fn get_one(&self, input: SearchResourceInput) -> DockitResult<Option<Resource>>;

    async fn get_many(&self, input: SearchManyResourcesInput) -> DockitResult<Vec<Resource>>;

    async fn update(&self, input: UpdateResourceInput) -> DockitResult<Resource>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
