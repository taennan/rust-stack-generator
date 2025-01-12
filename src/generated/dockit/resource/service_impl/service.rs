use crate::{
    resource::utils::{
        CreateResourceInputConverter, SearchManyResourcesInputConverter,
        SearchResourceInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::ResourceDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::resource::ResourceService as ResourceServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    resource::{
        CreateResourceInput, Resource, SearchManyResourcesInput, SearchResourceInput,
        UpdateResourceInput,
    },
};
use uuid::Uuid;

pub struct ResourceService<D> {
    resource_db: D,
    org_id: Uuid,
}

impl<D> ResourceService<D> {
    pub fn new(resource_db: D, org_id: Uuid) -> Self {
        Self {
            resource_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> ResourceServiceTrait for ResourceService<D>
where
    D: ResourceDB + Sync,
{
    async fn create(&self, input: CreateResourceInput) -> DockitResult<Resource> {
        service_impl::simple_create_impl!(self, resource_db, input, CreateResourceInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Resource> {
        service_impl::simple_get_by_id_impl!(self, id, SearchResourceInput)
    }

    async fn get_one(&self, input: SearchResourceInput) -> DockitResult<Option<Resource>> {
        service_impl::simple_get_one_impl!(self, resource_db, input, SearchResourceInputConverter)
    }

    async fn get_many(&self, input: SearchManyResourcesInput) -> DockitResult<Vec<Resource>> {
        service_impl::simple_get_many_impl!(
            self,
            resource_db,
            input,
            SearchManyResourcesInputConverter
        )
    }

    async fn update(&self, input: UpdateResourceInput) -> DockitResult<Resource> {
        service_impl::simple_update_impl!(self, resource_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, resource_db, id)
    }
}
