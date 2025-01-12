use crate::{
    admin_api_key::utils::{
        CreateAdminApiKeyInputConverter, SearchManyAdminApiKeysInputConverter,
        SearchAdminApiKeyInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::AdminApiKeyDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::admin_api_key::AdminApiKeyService as AdminApiKeyServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    admin_api_key::{
        CreateAdminApiKeyInput, AdminApiKey, SearchManyAdminApiKeysInput, SearchAdminApiKeyInput,
        UpdateAdminApiKeyInput,
    },
};
use uuid::Uuid;

pub struct AdminApiKeyService<D> {
    admin_api_key_db: D,
    org_id: Uuid,
}

impl<D> AdminApiKeyService<D> {
    pub fn new(admin_api_key_db: D, org_id: Uuid) -> Self {
        Self {
            admin_api_key_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> AdminApiKeyServiceTrait for AdminApiKeyService<D>
where
    D: AdminApiKeyDB + Sync,
{
    async fn create(&self, input: CreateAdminApiKeyInput) -> DockitResult<AdminApiKey> {
        service_impl::simple_create_impl!(self, admin_api_key_db, input, CreateAdminApiKeyInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<AdminApiKey> {
        service_impl::simple_get_by_id_impl!(self, id, SearchAdminApiKeyInput)
    }

    async fn get_one(&self, input: SearchAdminApiKeyInput) -> DockitResult<Option<AdminApiKey>> {
        service_impl::simple_get_one_impl!(self, admin_api_key_db, input, SearchAdminApiKeyInputConverter)
    }

    async fn get_many(&self, input: SearchManyAdminApiKeysInput) -> DockitResult<Vec<AdminApiKey>> {
        service_impl::simple_get_many_impl!(
            self,
            admin_api_key_db,
            input,
            SearchManyAdminApiKeysInputConverter
        )
    }

    async fn update(&self, input: UpdateAdminApiKeyInput) -> DockitResult<AdminApiKey> {
        service_impl::simple_update_impl!(self, admin_api_key_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, admin_api_key_db, id)
    }
}
