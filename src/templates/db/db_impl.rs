use crate::{
    error::SeaOrmErrorConverter,
    {entity_name_lowercase}::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            Create{entity_name}InputConverter,
            Search{entity_name}InputConverter,
            Update{entity_name}InputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::{entity_name_lowercase}::{
    Create{entity_name}Input, Search{entity_name}Input, SearchMany{entity_name}sInput,
    {entity_name}, Update{entity_name}Input,
};
pub use db_interface::{entity_name_lowercase}::{entity_name}DB as {entity_name}DBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct {entity_name}DB {
    connection: DatabaseConnection,
}

impl {entity_name}DB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl {entity_name}DBTrait for {entity_name}DB {
    async fn create(&self, input: Create{entity_name}Input) -> DBResult<{entity_name}> {
        let input_converter = Create{entity_name}InputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let {entity_name_lowercase} = {entity_name}::from(model);
        Ok({entity_name_lowercase})
    }

    async fn get_one(&self, input: Search{entity_name}Input) -> DBResult<Option<{entity_name}>> {
        let converter = Search{entity_name}InputConverter::from(input);
        let {entity_name_lowercase} = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some({entity_name}::from(model)));

        Ok({entity_name_lowercase})
    }

    async fn get_many(&self, input: SearchMany{entity_name}sInput) -> DBResult<Vec<{entity_name}>> {
        let converter = Search{entity_name}InputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let {entity_name_lowercase}s = models.into_iter().map({entity_name}::from).collect();
        Ok({entity_name_lowercase}s)
    }

    async fn update(&self, input: Update{entity_name}Input) -> DBResult<{entity_name}> {
        let converter = Update{entity_name}InputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let {entity_name_lowercase} = {entity_name}::from(model);
        Ok({entity_name_lowercase})
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
