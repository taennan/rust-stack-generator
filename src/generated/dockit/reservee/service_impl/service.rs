use crate::{
    reservee::utils::{
        CreateReserveeInputConverter, SearchManyReserveesInputConverter,
        SearchReserveeInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::ReserveeDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::reservee::ReserveeService as ReserveeServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    reservee::{
        CreateReserveeInput, Reservee, SearchManyReserveesInput, SearchReserveeInput,
        UpdateReserveeInput,
    },
};
use uuid::Uuid;

pub struct ReserveeService<D> {
    reservee_db: D,
    org_id: Uuid,
}

impl<D> ReserveeService<D> {
    pub fn new(reservee_db: D, org_id: Uuid) -> Self {
        Self {
            reservee_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> ReserveeServiceTrait for ReserveeService<D>
where
    D: ReserveeDB + Sync,
{
    async fn create(&self, input: CreateReserveeInput) -> DockitResult<Reservee> {
        service_impl::simple_create_impl!(self, reservee_db, input, CreateReserveeInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Reservee> {
        service_impl::simple_get_by_id_impl!(self, id, SearchReserveeInput)
    }

    async fn get_one(&self, input: SearchReserveeInput) -> DockitResult<Option<Reservee>> {
        service_impl::simple_get_one_impl!(self, reservee_db, input, SearchReserveeInputConverter)
    }

    async fn get_many(&self, input: SearchManyReserveesInput) -> DockitResult<Vec<Reservee>> {
        service_impl::simple_get_many_impl!(
            self,
            reservee_db,
            input,
            SearchManyReserveesInputConverter
        )
    }

    async fn update(&self, input: UpdateReserveeInput) -> DockitResult<Reservee> {
        service_impl::simple_update_impl!(self, reservee_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, reservee_db, id)
    }
}
