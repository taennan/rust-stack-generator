use crate::{
    error::SeaOrmErrorConverter,
    admin_api_key::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateAdminApiKeyInputConverter,
            SearchAdminApiKeyInputConverter,
            UpdateAdminApiKeyInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::admin_api_key::{
    CreateAdminApiKeyInput, SearchAdminApiKeyInput, SearchManyAdminApiKeysInput,
    AdminApiKey, UpdateAdminApiKeyInput,
};
pub use db_interface::admin_api_key::AdminApiKeyDB as AdminApiKeyDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct AdminApiKeyDB {
    connection: DatabaseConnection,
}

impl AdminApiKeyDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl AdminApiKeyDBTrait for AdminApiKeyDB {
    async fn create(&self, input: CreateAdminApiKeyInput) -> DBResult<AdminApiKey> {
        let input_converter = CreateAdminApiKeyInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let admin_api_key = AdminApiKey::from(model);
        Ok(admin_api_key)
    }

    async fn get_one(&self, input: SearchAdminApiKeyInput) -> DBResult<Option<AdminApiKey>> {
        let converter = SearchAdminApiKeyInputConverter::from(input);
        let admin_api_key = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(AdminApiKey::from(model)));

        Ok(admin_api_key)
    }

    async fn get_many(&self, input: SearchManyAdminApiKeysInput) -> DBResult<Vec<AdminApiKey>> {
        let converter = SearchAdminApiKeyInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let admin_api_keys = models.into_iter().map(AdminApiKey::from).collect();
        Ok(admin_api_keys)
    }

    async fn update(&self, input: UpdateAdminApiKeyInput) -> DBResult<AdminApiKey> {
        let converter = UpdateAdminApiKeyInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let admin_api_key = AdminApiKey::from(model);
        Ok(admin_api_key)
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
