use crate::{
    org::utils::{
        CreateOrgInputConverter, SearchManyOrgsInputConverter,
        SearchOrgInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::OrgDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::org::OrgService as OrgServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    org::{
        CreateOrgInput, Org, SearchManyOrgsInput, SearchOrgInput,
        UpdateOrgInput,
    },
};
use uuid::Uuid;

pub struct OrgService<D> {
    org_db: D,
    org_id: Uuid,
}

impl<D> OrgService<D> {
    pub fn new(org_db: D, org_id: Uuid) -> Self {
        Self {
            org_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> OrgServiceTrait for OrgService<D>
where
    D: OrgDB + Sync,
{
    async fn create(&self, input: CreateOrgInput) -> DockitResult<Org> {
        service_impl::simple_create_impl!(self, org_db, input, CreateOrgInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Org> {
        service_impl::simple_get_by_id_impl!(self, id, SearchOrgInput)
    }

    async fn get_one(&self, input: SearchOrgInput) -> DockitResult<Option<Org>> {
        service_impl::simple_get_one_impl!(self, org_db, input, SearchOrgInputConverter)
    }

    async fn get_many(&self, input: SearchManyOrgsInput) -> DockitResult<Vec<Org>> {
        service_impl::simple_get_many_impl!(
            self,
            org_db,
            input,
            SearchManyOrgsInputConverter
        )
    }

    async fn update(&self, input: UpdateOrgInput) -> DockitResult<Org> {
        service_impl::simple_update_impl!(self, org_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, org_db, id)
    }
}
