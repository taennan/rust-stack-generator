use crate::{
    {entity_name_lowercase}::utils::{
        Create{entity_name}InputConverter, SearchMany{entity_name}sInputConverter,
        Search{entity_name}InputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::{entity_name}DB;
use dockit_error::DockitResult;
pub use dockit_services_interface::{entity_name_lowercase}::{entity_name}Service as {entity_name}ServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    {entity_name_lowercase}::{
        Create{entity_name}Input, {entity_name}, SearchMany{entity_name}sInput, Search{entity_name}Input,
        Update{entity_name}Input,
    },
};
use uuid::Uuid;

pub struct {entity_name}Service<D> {
    {entity_name_lowercase}_db: D,
    org_id: Uuid,
}

impl<D> {entity_name}Service<D> {
    pub fn new({entity_name_lowercase}_db: D, org_id: Uuid) -> Self {
        Self {
            {entity_name_lowercase}_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> {entity_name}ServiceTrait for {entity_name}Service<D>
where
    D: {entity_name}DB + Sync,
{
    async fn create(&self, input: Create{entity_name}Input) -> DockitResult<{entity_name}> {
        service_impl::simple_create_impl!(self, {entity_name_lowercase}_db, input, Create{entity_name}InputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<{entity_name}> {
        service_impl::simple_get_by_id_impl!(self, id, Search{entity_name}Input)
    }

    async fn get_one(&self, input: Search{entity_name}Input) -> DockitResult<Option<{entity_name}>> {
        service_impl::simple_get_one_impl!(self, {entity_name_lowercase}_db, input, Search{entity_name}InputConverter)
    }

    async fn get_many(&self, input: SearchMany{entity_name}sInput) -> DockitResult<Vec<{entity_name}>> {
        service_impl::simple_get_many_impl!(
            self,
            {entity_name_lowercase}_db,
            input,
            SearchMany{entity_name}sInputConverter
        )
    }

    async fn update(&self, input: Update{entity_name}Input) -> DockitResult<{entity_name}> {
        service_impl::simple_update_impl!(self, {entity_name_lowercase}_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, {entity_name_lowercase}_db, id)
    }
}
