use crate::{
    error::SeaOrmErrorConverter,
    org::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateOrgInputConverter,
            SearchOrgInputConverter,
            UpdateOrgInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::org::{
    CreateOrgInput, SearchOrgInput, SearchManyOrgsInput,
    Org, UpdateOrgInput,
};
pub use db_interface::org::OrgDB as OrgDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct OrgDB {
    connection: DatabaseConnection,
}

impl OrgDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl OrgDBTrait for OrgDB {
    async fn create(&self, input: CreateOrgInput) -> DBResult<Org> {
        let input_converter = CreateOrgInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let org = Org::from(model);
        Ok(org)
    }

    async fn get_one(&self, input: SearchOrgInput) -> DBResult<Option<Org>> {
        let converter = SearchOrgInputConverter::from(input);
        let org = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(Org::from(model)));

        Ok(org)
    }

    async fn get_many(&self, input: SearchManyOrgsInput) -> DBResult<Vec<Org>> {
        let converter = SearchOrgInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let orgs = models.into_iter().map(Org::from).collect();
        Ok(orgs)
    }

    async fn update(&self, input: UpdateOrgInput) -> DBResult<Org> {
        let converter = UpdateOrgInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let org = Org::from(model);
        Ok(org)
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
