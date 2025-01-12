use crate::{
    common::DeleteOutput,
    reservation::{
        CreateReservationInput, Reservation, SearchManyReservationsInput, SearchReservationInput,
        UpdateReservationInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ReservationService {
    async fn create(&self, input: CreateReservationInput) -> DockitResult<Reservation>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Reservation>;

    async fn get_one(&self, input: SearchReservationInput) -> DockitResult<Option<Reservation>>;

    async fn get_many(&self, input: SearchManyReservationsInput) -> DockitResult<Vec<Reservation>>;

    async fn update(&self, input: UpdateReservationInput) -> DockitResult<Reservation>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
