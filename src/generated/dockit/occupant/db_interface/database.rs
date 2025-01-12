use crate::{
    error::DBResult,
    occupant::{CreateOccupantInput, Occupant, SearchOccupantInput, UpdateOccupantInput}
};
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait OccupantDB {
    async fn create(&self, input: CreateOccupantInput) -> DBResult<Occupant>;

    async fn get_one(&self, input: SearchOccupantInput) -> DBResult<Option<Occupant>>;

    async fn get_many(&self, input: SearchOccupantInput) -> DBResult<Vec<Occupant>>;

    async fn update(&self, input: UpdateOccupantInput) -> DBResult<Occupant>;

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize>;
}
