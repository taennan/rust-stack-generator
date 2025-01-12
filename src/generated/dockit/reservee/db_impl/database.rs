use crate::{
    error::SeaOrmErrorConverter,
    reservee::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateReserveeInputConverter,
            SearchReserveeInputConverter,
            UpdateReserveeInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::reservee::{
    CreateReserveeInput, SearchReserveeInput, SearchManyReserveesInput,
    Reservee, UpdateReserveeInput,
};
pub use db_interface::reservee::ReserveeDB as ReserveeDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct ReserveeDB {
    connection: DatabaseConnection,
}

impl ReserveeDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ReserveeDBTrait for ReserveeDB {
    async fn create(&self, input: CreateReserveeInput) -> DBResult<Reservee> {
        let input_converter = CreateReserveeInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservee = Reservee::from(model);
        Ok(reservee)
    }

    async fn get_one(&self, input: SearchReserveeInput) -> DBResult<Option<Reservee>> {
        let converter = SearchReserveeInputConverter::from(input);
        let reservee = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(Reservee::from(model)));

        Ok(reservee)
    }

    async fn get_many(&self, input: SearchManyReserveesInput) -> DBResult<Vec<Reservee>> {
        let converter = SearchReserveeInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservees = models.into_iter().map(Reservee::from).collect();
        Ok(reservees)
    }

    async fn update(&self, input: UpdateReserveeInput) -> DBResult<Reservee> {
        let converter = UpdateReserveeInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservee = Reservee::from(model);
        Ok(reservee)
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
