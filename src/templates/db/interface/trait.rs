use crate::{entity_lowercase}::{Create{entity}Input, {entity}, Search{entity}Input, Update{entity}Input};
use {project_lower}_error::SSResult;
use {project_lower}_common_models::delete::DeleteOutput;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

pub type {entity}DBTrait = {entity}DB;

#[automock]
#[async_trait]
pub trait {entity}DB {
    async fn create(&self, input: Create{entity}Input) -> SSResult<{entity}>;

    async fn get_one(&self, input: Search{entity}Input) -> SSResult<Option<{entity}>>;

    async fn get_many(&self, input: Search{entity}Input) -> SSResult<Vec<{entity}>>;

    async fn update(&self, input: Update{entity}Input) -> SSResult<{entity}>;

    async fn delete_by_id(&self, id: Uuid) -> SSResult<DeleteOutput>;
}
