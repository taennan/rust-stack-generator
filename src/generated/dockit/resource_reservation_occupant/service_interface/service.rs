use crate::{
    common::DeleteOutput,
    resource_reservation_occupant::{
        CreateResourceReservationOccupantInput, ResourceReservationOccupant, SearchManyResourceReservationOccupantsInput, SearchResourceReservationOccupantInput,
        UpdateResourceReservationOccupantInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ResourceReservationOccupantService {
    async fn create(&self, input: CreateResourceReservationOccupantInput) -> DockitResult<ResourceReservationOccupant>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<ResourceReservationOccupant>;

    async fn get_one(&self, input: SearchResourceReservationOccupantInput) -> DockitResult<Option<ResourceReservationOccupant>>;

    async fn get_many(&self, input: SearchManyResourceReservationOccupantsInput) -> DockitResult<Vec<ResourceReservationOccupant>>;

    async fn update(&self, input: UpdateResourceReservationOccupantInput) -> DockitResult<ResourceReservationOccupant>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
