use crate::{
    error::DBResult,
    reservee::{CreateReserveeInput, Reservee, SearchReserveeInput, UpdateReserveeInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ReserveeDB {
    async fn create(&self, input: CreateReserveeInput) -> DBResult<Reservee>;

    async fn get_one(&self, input: SearchReserveeInput) -> DBResult<Option<Reservee>>;

    async fn get_many(&self, input: SearchReserveeInput) -> DBResult<Vec<Reservee>>;

    async fn update(&self, input: UpdateReserveeInput) -> DBResult<Reservee>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
