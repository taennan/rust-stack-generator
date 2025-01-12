use crate::{
    common::DeleteOutput,
    reservee::{
        CreateReserveeInput, Reservee, SearchManyReserveesInput, SearchReserveeInput,
        UpdateReserveeInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ReserveeService {
    async fn create(&self, input: CreateReserveeInput) -> DockitResult<Reservee>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Reservee>;

    async fn get_one(&self, input: SearchReserveeInput) -> DockitResult<Option<Reservee>>;

    async fn get_many(&self, input: SearchManyReserveesInput) -> DockitResult<Vec<Reservee>>;

    async fn update(&self, input: UpdateReserveeInput) -> DockitResult<Reservee>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
