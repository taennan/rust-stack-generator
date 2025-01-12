use crate::{
    error::DBResult,
    admin_api_key::{CreateAdminApiKeyInput, AdminApiKey, SearchAdminApiKeyInput, UpdateAdminApiKeyInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait AdminApiKeyDB {
    async fn create(&self, input: CreateAdminApiKeyInput) -> DBResult<AdminApiKey>;

    async fn get_one(&self, input: SearchAdminApiKeyInput) -> DBResult<Option<AdminApiKey>>;

    async fn get_many(&self, input: SearchAdminApiKeyInput) -> DBResult<Vec<AdminApiKey>>;

    async fn update(&self, input: UpdateAdminApiKeyInput) -> DBResult<AdminApiKey>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
