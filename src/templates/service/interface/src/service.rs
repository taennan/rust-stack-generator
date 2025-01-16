use crate::{
    {entity_lower}::{
        Create{entity}Input, {entity}, SearchMany{entity}sInput, Search{entity}Input,
        Update{entity}Input,
    },
};
use {project_lower}_common_models::{delete::DeleteOutput, search::CountOutput};
use async_trait::async_trait;
use {project_lower}_error::{project_prefix}Result;
use mockall::automock;
use uuid::Uuid;

pub type {entity}ServiceTrait = dyn {entity}Service;

#[automock]
#[async_trait]
pub trait {entity}Service {
    async fn create(&self, input: Create{entity}Input) -> {project_prefix}Result<{entity}>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> {project_prefix}Result<{entity}>;

    async fn get_one(&self, input: Search{entity}Input) -> {project_prefix}Result<Option<{entity}>>;

    async fn get_many(&self, input: SearchMany{entity}sInput) -> {project_prefix}Result<Vec<{entity}>>;

    async fn count(&self, input: Search{entity}Input) -> {project_prefix}Result<CountOutput>;

    async fn update(&self, input: Update{entity}Input) -> {project_prefix}Result<{entity}>;

    async fn delete_by_id(&self, id: Uuid) -> {project_prefix}Result<DeleteOutput>;
}
