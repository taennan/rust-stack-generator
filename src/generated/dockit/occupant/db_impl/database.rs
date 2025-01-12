use crate::{
    error::SeaOrmErrorConverter,
    occupant::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateOccupantInputConverter,
            SearchOccupantInputConverter,
            UpdateOccupantInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::occupant::{
    CreateOccupantInput, SearchOccupantInput, SearchManyOccupantsInput,
    Occupant, UpdateOccupantInput,
};
pub use db_interface::occupant::OccupantDB as OccupantDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct OccupantDB {
    connection: DatabaseConnection,
}

impl OccupantDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl OccupantDBTrait for OccupantDB {
    async fn create(&self, input: CreateOccupantInput) -> DBResult<Occupant> {
        let input_converter = CreateOccupantInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let occupant = Occupant::from(model);
        Ok(occupant)
    }

    async fn get_one(&self, input: SearchOccupantInput) -> DBResult<Option<Occupant>> {
        let converter = SearchOccupantInputConverter::from(input);
        let occupant = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(Occupant::from(model)));

        Ok(occupant)
    }

    async fn get_many(&self, input: SearchManyOccupantsInput) -> DBResult<Vec<Occupant>> {
        let converter = SearchOccupantInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let occupants = models.into_iter().map(Occupant::from).collect();
        Ok(occupants)
    }

    async fn update(&self, input: UpdateOccupantInput) -> DBResult<Occupant> {
        let converter = UpdateOccupantInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let occupant = Occupant::from(model);
        Ok(occupant)
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
