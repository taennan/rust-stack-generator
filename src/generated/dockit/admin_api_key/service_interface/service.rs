use crate::{
    common::DeleteOutput,
    admin_api_key::{
        CreateAdminApiKeyInput, AdminApiKey, SearchManyAdminApiKeysInput, SearchAdminApiKeyInput,
        UpdateAdminApiKeyInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait AdminApiKeyService {
    async fn create(&self, input: CreateAdminApiKeyInput) -> DockitResult<AdminApiKey>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<AdminApiKey>;

    async fn get_one(&self, input: SearchAdminApiKeyInput) -> DockitResult<Option<AdminApiKey>>;

    async fn get_many(&self, input: SearchManyAdminApiKeysInput) -> DockitResult<Vec<AdminApiKey>>;

    async fn update(&self, input: UpdateAdminApiKeyInput) -> DockitResult<AdminApiKey>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
