use crate::{
    error::DBResult,
    reservation::{CreateReservationInput, Reservation, SearchReservationInput, UpdateReservationInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ReservationDB {
    async fn create(&self, input: CreateReservationInput) -> DBResult<Reservation>;

    async fn get_one(&self, input: SearchReservationInput) -> DBResult<Option<Reservation>>;

    async fn get_many(&self, input: SearchReservationInput) -> DBResult<Vec<Reservation>>;

    async fn update(&self, input: UpdateReservationInput) -> DBResult<Reservation>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
