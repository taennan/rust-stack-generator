use crate::{
    error::DBResult,
    resource::{CreateResourceInput, Resource, SearchResourceInput, UpdateResourceInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ResourceDB {
    async fn create(&self, input: CreateResourceInput) -> DBResult<Resource>;

    async fn get_one(&self, input: SearchResourceInput) -> DBResult<Option<Resource>>;

    async fn get_many(&self, input: SearchResourceInput) -> DBResult<Vec<Resource>>;

    async fn update(&self, input: UpdateResourceInput) -> DBResult<Resource>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
