use crate::{
    error::DBResult,
    resource_reservation::{CreateResourceReservationInput, ResourceReservation, SearchResourceReservationInput, UpdateResourceReservationInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ResourceReservationDB {
    async fn create(&self, input: CreateResourceReservationInput) -> DBResult<ResourceReservation>;

    async fn get_one(&self, input: SearchResourceReservationInput) -> DBResult<Option<ResourceReservation>>;

    async fn get_many(&self, input: SearchResourceReservationInput) -> DBResult<Vec<ResourceReservation>>;

    async fn update(&self, input: UpdateResourceReservationInput) -> DBResult<ResourceReservation>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
