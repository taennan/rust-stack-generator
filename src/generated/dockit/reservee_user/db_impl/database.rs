use crate::{
    error::SeaOrmErrorConverter,
    reservee_user::{
        entity::{ActiveModel, Entity, Column},
        utils::{
            CreateReserveeUserInputConverter,
            SearchReserveeUserInputConverter,
            UpdateReserveeUserInputConverter,
        },
    },
};
use async_trait::async_trait;
use db_models::reservee_user::{
    CreateReserveeUserInput, SearchReserveeUserInput, SearchManyReserveeUsersInput,
    ReserveeUser, UpdateReserveeUserInput,
};
pub use db_interface::reservee_user::ReserveeUserDB as ReserveeUserDBTrait;
use db_interface::error::{DBError, DBResult};
use sea_orm::{entity::prelude::*, DatabaseConnection};
use uuid::Uuid;

pub struct ReserveeUserDB {
    connection: DatabaseConnection,
}

impl ReserveeUserDB {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ReserveeUserDBTrait for ReserveeUserDB {
    async fn create(&self, input: CreateReserveeUserInput) -> DBResult<ReserveeUser> {
        let input_converter = CreateReserveeUserInputConverter::from(input);
        let model = ActiveModel::from(input_converter)
            .insert(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservee_user = ReserveeUser::from(model);
        Ok(reservee_user)
    }

    async fn get_one(&self, input: SearchReserveeUserInput) -> DBResult<Option<ReserveeUser>> {
        let converter = SearchReserveeUserInputConverter::from(input);
        let reservee_user = Select::<Entity>::from(converter)
            .one(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?
            .and_then(|model| Some(ReserveeUser::from(model)));

        Ok(reservee_user)
    }

    async fn get_many(&self, input: SearchManyReserveeUsersInput) -> DBResult<Vec<ReserveeUser>> {
        let converter = SearchReserveeUserInputConverter::from(input);
        let models = Select::<Entity>::from(converter)
            .all(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservee_users = models.into_iter().map(ReserveeUser::from).collect();
        Ok(reservee_users)
    }

    async fn update(&self, input: UpdateReserveeUserInput) -> DBResult<ReserveeUser> {
        let converter = UpdateReserveeUserInputConverter::from(input);
        let model = ActiveModel::from(converter)
            .update(&self.connection)
            .await
            .map_err(SeaOrmErrorConverter::from)?;

        let reservee_user = ReserveeUser::from(model);
        Ok(reservee_user)
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
