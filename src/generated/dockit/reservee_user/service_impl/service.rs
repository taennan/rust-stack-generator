use crate::{
    reservee_user::utils::{
        CreateReserveeUserInputConverter, SearchManyReserveeUsersInputConverter,
        SearchReserveeUserInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::ReserveeUserDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::reservee_user::ReserveeUserService as ReserveeUserServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    reservee_user::{
        CreateReserveeUserInput, ReserveeUser, SearchManyReserveeUsersInput, SearchReserveeUserInput,
        UpdateReserveeUserInput,
    },
};
use uuid::Uuid;

pub struct ReserveeUserService<D> {
    reservee_user_db: D,
    org_id: Uuid,
}

impl<D> ReserveeUserService<D> {
    pub fn new(reservee_user_db: D, org_id: Uuid) -> Self {
        Self {
            reservee_user_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> ReserveeUserServiceTrait for ReserveeUserService<D>
where
    D: ReserveeUserDB + Sync,
{
    async fn create(&self, input: CreateReserveeUserInput) -> DockitResult<ReserveeUser> {
        service_impl::simple_create_impl!(self, reservee_user_db, input, CreateReserveeUserInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<ReserveeUser> {
        service_impl::simple_get_by_id_impl!(self, id, SearchReserveeUserInput)
    }

    async fn get_one(&self, input: SearchReserveeUserInput) -> DockitResult<Option<ReserveeUser>> {
        service_impl::simple_get_one_impl!(self, reservee_user_db, input, SearchReserveeUserInputConverter)
    }

    async fn get_many(&self, input: SearchManyReserveeUsersInput) -> DockitResult<Vec<ReserveeUser>> {
        service_impl::simple_get_many_impl!(
            self,
            reservee_user_db,
            input,
            SearchManyReserveeUsersInputConverter
        )
    }

    async fn update(&self, input: UpdateReserveeUserInput) -> DockitResult<ReserveeUser> {
        service_impl::simple_update_impl!(self, reservee_user_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, reservee_user_db, id)
    }
}
