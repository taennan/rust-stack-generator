use crate::{
    common::DeleteOutput,
    resource_reservation::{
        CreateResourceReservationInput, ResourceReservation, SearchManyResourceReservationsInput, SearchResourceReservationInput,
        UpdateResourceReservationInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ResourceReservationService {
    async fn create(&self, input: CreateResourceReservationInput) -> DockitResult<ResourceReservation>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<ResourceReservation>;

    async fn get_one(&self, input: SearchResourceReservationInput) -> DockitResult<Option<ResourceReservation>>;

    async fn get_many(&self, input: SearchManyResourceReservationsInput) -> DockitResult<Vec<ResourceReservation>>;

    async fn update(&self, input: UpdateResourceReservationInput) -> DockitResult<ResourceReservation>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
