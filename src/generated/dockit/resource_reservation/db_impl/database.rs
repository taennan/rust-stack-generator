use crate::{
    error::SeaOrmErrorConverter,
    resource_reservation::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateResourceReservationInputConverter,
            SearchResourceReservationInputConverter,
            UpdateResourceReservationInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::resource_reservation::{
    CreateResourceReservationInput, SearchResourceReservationInput, SearchManyResourceReservationsInput,
    ResourceReservation, UpdateResourceReservationInput,
};
pub use db_interface::resource_reservation::ResourceReservationDB as ResourceReservationDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct ResourceReservationDB {
    connection: DatabaseConnection,
}

impl ResourceReservationDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ResourceReservationDBTrait for ResourceReservationDB {
    async fn create(&self, input: CreateResourceReservationInput) -> DBResult<ResourceReservation> {
        let input_converter = CreateResourceReservationInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resource_reservation = ResourceReservation::from(model);
        Ok(resource_reservation)
    }

    async fn get_one(&self, input: SearchResourceReservationInput) -> DBResult<Option<ResourceReservation>> {
        let converter = SearchResourceReservationInputConverter::from(input);
        let resource_reservation = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(ResourceReservation::from(model)));

        Ok(resource_reservation)
    }

    async fn get_many(&self, input: SearchManyResourceReservationsInput) -> DBResult<Vec<ResourceReservation>> {
        let converter = SearchResourceReservationInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resource_reservations = models.into_iter().map(ResourceReservation::from).collect();
        Ok(resource_reservations)
    }

    async fn update(&self, input: UpdateResourceReservationInput) -> DBResult<ResourceReservation> {
        let converter = UpdateResourceReservationInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resource_reservation = ResourceReservation::from(model);
        Ok(resource_reservation)
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
