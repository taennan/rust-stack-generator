use crate::{
    common::DeleteOutput,
    reservee_user::{
        CreateReserveeUserInput, ReserveeUser, SearchManyReserveeUsersInput, SearchReserveeUserInput,
        UpdateReserveeUserInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ReserveeUserService {
    async fn create(&self, input: CreateReserveeUserInput) -> DockitResult<ReserveeUser>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<ReserveeUser>;

    async fn get_one(&self, input: SearchReserveeUserInput) -> DockitResult<Option<ReserveeUser>>;

    async fn get_many(&self, input: SearchManyReserveeUsersInput) -> DockitResult<Vec<ReserveeUser>>;

    async fn update(&self, input: UpdateReserveeUserInput) -> DockitResult<ReserveeUser>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
