use crate::{
    common::DeleteOutput,
    occupant::{
        CreateOccupantInput, Occupant, SearchManyOccupantsInput, SearchOccupantInput,
        UpdateOccupantInput,
    },
};
use async_trait::async_trait;
use dockit_error::DockitResult;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait OccupantService {
    async fn create(&self, input: CreateOccupantInput) -> DockitResult<Occupant>;

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Occupant>;

    async fn get_one(&self, input: SearchOccupantInput) -> DockitResult<Option<Occupant>>;

    async fn get_many(&self, input: SearchManyOccupantsInput) -> DockitResult<Vec<Occupant>>;

    async fn update(&self, input: UpdateOccupantInput) -> DockitResult<Occupant>;

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput>;
}
