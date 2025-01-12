use crate::{
    error::SeaOrmErrorConverter,
    resource::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateResourceInputConverter,
            SearchResourceInputConverter,
            UpdateResourceInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::resource::{
    CreateResourceInput, SearchResourceInput, SearchManyResourcesInput,
    Resource, UpdateResourceInput,
};
pub use db_interface::resource::ResourceDB as ResourceDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct ResourceDB {
    connection: DatabaseConnection,
}

impl ResourceDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ResourceDBTrait for ResourceDB {
    async fn create(&self, input: CreateResourceInput) -> DBResult<Resource> {
        let input_converter = CreateResourceInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resource = Resource::from(model);
        Ok(resource)
    }

    async fn get_one(&self, input: SearchResourceInput) -> DBResult<Option<Resource>> {
        let converter = SearchResourceInputConverter::from(input);
        let resource = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(Resource::from(model)));

        Ok(resource)
    }

    async fn get_many(&self, input: SearchManyResourcesInput) -> DBResult<Vec<Resource>> {
        let converter = SearchResourceInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resources = models.into_iter().map(Resource::from).collect();
        Ok(resources)
    }

    async fn update(&self, input: UpdateResourceInput) -> DBResult<Resource> {
        let converter = UpdateResourceInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let resource = Resource::from(model);
        Ok(resource)
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
