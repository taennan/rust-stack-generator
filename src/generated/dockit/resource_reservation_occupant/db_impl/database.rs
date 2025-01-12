use crate::{
    error::SeaOrmErrorConverter,
    resource_reservation_occupant::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateResourceReservationOccupantInputConverter,
            SearchResourceReservationOccupantInputConverter,
            UpdateResourceReservationOccupantInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::resource_reservation_occupant::{
    CreateResourceReservationOccupantInput, SearchResourceReservationOccupantInput, SearchManyResourceReservationOccupantsInput,
    ResourceReservationOccupant, UpdateResourceReservationOccupantInput,
};
pub use db_interface::resource_reservation_occupant::ResourceReservationOccupantDB as ResourceReservationOccupantDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct ResourceReservationOccupantDB {
    connection: DatabaseConnection,
}

impl ResourceReservationOccupantDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ResourceReservationOccupantDBTrait for ResourceReservationOccupantDB {
    async fn create(&self, input: CreateResourceReservationOccupantInput) -> DBResult<ResourceReservationOccupant> {
        let input_converter = CreateResourceReservationOccupantInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resource_reservation_occupant = ResourceReservationOccupant::from(model);
        Ok(resource_reservation_occupant)
    }

    async fn get_one(&self, input: SearchResourceReservationOccupantInput) -> DBResult<Option<ResourceReservationOccupant>> {
        let converter = SearchResourceReservationOccupantInputConverter::from(input);
        let resource_reservation_occupant = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(ResourceReservationOccupant::from(model)));

        Ok(resource_reservation_occupant)
    }

    async fn get_many(&self, input: SearchManyResourceReservationOccupantsInput) -> DBResult<Vec<ResourceReservationOccupant>> {
        let converter = SearchResourceReservationOccupantInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resource_reservation_occupants = models.into_iter().map(ResourceReservationOccupant::from).collect();
        Ok(resource_reservation_occupants)
    }

    async fn update(&self, input: UpdateResourceReservationOccupantInput) -> DBResult<ResourceReservationOccupant> {
        let converter = UpdateResourceReservationOccupantInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resource_reservation_occupant = ResourceReservationOccupant::from(model);
        Ok(resource_reservation_occupant)
    }

    async fn delete_by_id(&self, id: Uuid) -> DBResult<usize> {
        let result = Entity::delete_many()
            .filter(Column::Id.eq(id))
            .exec(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        if result.rows_affected == 0 {
            Err(DBError::NotFound)
        } else {
            Ok(1)
        }
    }
}
