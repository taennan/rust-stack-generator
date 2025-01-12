use crate::{
    error::SeaOrmErrorConverter,
    reservation::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateReservationInputConverter,
            SearchReservationInputConverter,
            UpdateReservationInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::reservation::{
    CreateReservationInput, SearchReservationInput, SearchManyReservationsInput,
    Reservation, UpdateReservationInput,
};
pub use db_interface::reservation::ReservationDB as ReservationDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct ReservationDB {
    connection: DatabaseConnection,
}

impl ReservationDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ReservationDBTrait for ReservationDB {
    async fn create(&self, input: CreateReservationInput) -> DBResult<Reservation> {
        let input_converter = CreateReservationInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservation = Reservation::from(model);
        Ok(reservation)
    }

    async fn get_one(&self, input: SearchReservationInput) -> DBResult<Option<Reservation>> {
        let converter = SearchReservationInputConverter::from(input);
        let reservation = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(Reservation::from(model)));

        Ok(reservation)
    }

    async fn get_many(&self, input: SearchManyReservationsInput) -> DBResult<Vec<Reservation>> {
        let converter = SearchReservationInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservations = models.into_iter().map(Reservation::from).collect();
        Ok(reservations)
    }

    async fn update(&self, input: UpdateReservationInput) -> DBResult<Reservation> {
        let converter = UpdateReservationInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservation = Reservation::from(model);
        Ok(reservation)
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
