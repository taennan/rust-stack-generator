use crate::{
    org_user::utils::{
        CreateOrgUserInputConverter, SearchManyOrgUsersInputConverter,
        SearchOrgUserInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::OrgUserDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::org_user::OrgUserService as OrgUserServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    org_user::{
        CreateOrgUserInput, OrgUser, SearchManyOrgUsersInput, SearchOrgUserInput,
        UpdateOrgUserInput,
    },
};
use uuid::Uuid;

pub struct OrgUserService<D> {
    org_user_db: D,
    org_id: Uuid,
}

impl<D> OrgUserService<D> {
    pub fn new(org_user_db: D, org_id: Uuid) -> Self {
        Self {
            org_user_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> OrgUserServiceTrait for OrgUserService<D>
where
    D: OrgUserDB + Sync,
{
    async fn create(&self, input: CreateOrgUserInput) -> DockitResult<OrgUser> {
        service_impl::simple_create_impl!(self, org_user_db, input, CreateOrgUserInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<OrgUser> {
        service_impl::simple_get_by_id_impl!(self, id, SearchOrgUserInput)
    }

    async fn get_one(&self, input: SearchOrgUserInput) -> DockitResult<Option<OrgUser>> {
        service_impl::simple_get_one_impl!(self, org_user_db, input, SearchOrgUserInputConverter)
    }

    async fn get_many(&self, input: SearchManyOrgUsersInput) -> DockitResult<Vec<OrgUser>> {
        service_impl::simple_get_many_impl!(
            self,
            org_user_db,
            input,
            SearchManyOrgUsersInputConverter
        )
    }

    async fn update(&self, input: UpdateOrgUserInput) -> DockitResult<OrgUser> {
        service_impl::simple_update_impl!(self, org_user_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, org_user_db, id)
    }
}
