use crate::{
    reservation::utils::{
        CreateReservationInputConverter, SearchManyReservationsInputConverter,
        SearchReservationInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::ReservationDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::reservation::ReservationService as ReservationServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    reservation::{
        CreateReservationInput, Reservation, SearchManyReservationsInput, SearchReservationInput,
        UpdateReservationInput,
    },
};
use uuid::Uuid;

pub struct ReservationService<D> {
    reservation_db: D,
    org_id: Uuid,
}

impl<D> ReservationService<D> {
    pub fn new(reservation_db: D, org_id: Uuid) -> Self {
        Self {
            reservation_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> ReservationServiceTrait for ReservationService<D>
where
    D: ReservationDB + Sync,
{
    async fn create(&self, input: CreateReservationInput) -> DockitResult<Reservation> {
        service_impl::simple_create_impl!(self, reservation_db, input, CreateReservationInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Reservation> {
        service_impl::simple_get_by_id_impl!(self, id, SearchReservationInput)
    }

    async fn get_one(&self, input: SearchReservationInput) -> DockitResult<Option<Reservation>> {
        service_impl::simple_get_one_impl!(self, reservation_db, input, SearchReservationInputConverter)
    }

    async fn get_many(&self, input: SearchManyReservationsInput) -> DockitResult<Vec<Reservation>> {
        service_impl::simple_get_many_impl!(
            self,
            reservation_db,
            input,
            SearchManyReservationsInputConverter
        )
    }

    async fn update(&self, input: UpdateReservationInput) -> DockitResult<Reservation> {
        service_impl::simple_update_impl!(self, reservation_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, reservation_db, id)
    }
}
