use crate::{
    error::SeaOrmErrorConverter,
    org_user::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateOrgUserInputConverter,
            SearchOrgUserInputConverter,
            UpdateOrgUserInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::org_user::{
    CreateOrgUserInput, SearchOrgUserInput, SearchManyOrgUsersInput,
    OrgUser, UpdateOrgUserInput,
};
pub use db_interface::org_user::OrgUserDB as OrgUserDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct OrgUserDB {
    connection: DatabaseConnection,
}

impl OrgUserDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl OrgUserDBTrait for OrgUserDB {
    async fn create(&self, input: CreateOrgUserInput) -> DBResult<OrgUser> {
        let input_converter = CreateOrgUserInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let org_user = OrgUser::from(model);
        Ok(org_user)
    }

    async fn get_one(&self, input: SearchOrgUserInput) -> DBResult<Option<OrgUser>> {
        let converter = SearchOrgUserInputConverter::from(input);
        let org_user = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(OrgUser::from(model)));

        Ok(org_user)
    }

    async fn get_many(&self, input: SearchManyOrgUsersInput) -> DBResult<Vec<OrgUser>> {
        let converter = SearchOrgUserInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let org_users = models.into_iter().map(OrgUser::from).collect();
        Ok(org_users)
    }

    async fn update(&self, input: UpdateOrgUserInput) -> DBResult<OrgUser> {
        let converter = UpdateOrgUserInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let org_user = OrgUser::from(model);
        Ok(org_user)
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
