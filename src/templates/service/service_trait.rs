use crate::{
    common::DeleteOutput,
    {entity_name_lowercase}::{
        Create{entity_name}Input, {entity_name}, SearchMany{entity_name}sInput, Search{entity_name}Input,
        Update{entity_name}Input,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait {entity_name}Service {
    async fn create(&self, input: Create{entity_name}Input) -> DockitResult<{entity_name}>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<{entity_name}>;

    async fn get_one(&self, input: Search{entity_name}Input) -> DockitResult<Option<{entity_name}>>;

    async fn get_many(&self, input: SearchMany{entity_name}sInput) -> DockitResult<Vec<{entity_name}>>;

    async fn update(&self, input: Update{entity_name}Input) -> DockitResult<{entity_name}>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
