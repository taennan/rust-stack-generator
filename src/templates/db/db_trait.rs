use crate::{
    error::DBResult,
    {entity_name_lowercase}::{Create{entity_name}Input, {entity_name}, Search{entity_name}Input, Update{entity_name}Input}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait {entity_name}DB {
    async fn create(&self, input: Create{entity_name}Input) -> DBResult<{entity_name}>;

    async fn get_one(&self, input: Search{entity_name}Input) -> DBResult<Option<{entity_name}>>;

    async fn get_many(&self, input: Search{entity_name}Input) -> DBResult<Vec<{entity_name}>>;

    async fn update(&self, input: Update{entity_name}Input) -> DBResult<{entity_name}>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
