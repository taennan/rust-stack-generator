use crate::{
    error::DBResult,
    resource_reservation_occupant::{CreateResourceReservationOccupantInput, ResourceReservationOccupant, SearchResourceReservationOccupantInput, UpdateResourceReservationOccupantInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ResourceReservationOccupantDB {
    async fn create(&self, input: CreateResourceReservationOccupantInput) -> DBResult<ResourceReservationOccupant>;

    async fn get_one(&self, input: SearchResourceReservationOccupantInput) -> DBResult<Option<ResourceReservationOccupant>>;

    async fn get_many(&self, input: SearchResourceReservationOccupantInput) -> DBResult<Vec<ResourceReservationOccupant>>;

    async fn update(&self, input: UpdateResourceReservationOccupantInput) -> DBResult<ResourceReservationOccupant>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
