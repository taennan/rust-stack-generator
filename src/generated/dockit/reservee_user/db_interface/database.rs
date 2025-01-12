use crate::{
    error::DBResult,
    reservee_user::{CreateReserveeUserInput, ReserveeUser, SearchReserveeUserInput, UpdateReserveeUserInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ReserveeUserDB {
    async fn create(&self, input: CreateReserveeUserInput) -> DBResult<ReserveeUser>;

    async fn get_one(&self, input: SearchReserveeUserInput) -> DBResult<Option<ReserveeUser>>;

    async fn get_many(&self, input: SearchReserveeUserInput) -> DBResult<Vec<ReserveeUser>>;

    async fn update(&self, input: UpdateReserveeUserInput) -> DBResult<ReserveeUser>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
