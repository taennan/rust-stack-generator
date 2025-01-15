macro_rules! create_impl {
    ($this:ident, $input:ident, $model:ident, $input_converter:ident) => {{
        let converter = $input_converter::from($input);
        let db_model = ActiveModel::from(converter)
            .insert(&$this.connection)
            .await
            .map_err({project_lower}_error::{project_prefix}Error::from)?;

        let model = $model::from(db_model);
        Ok(model)
    }};
}

macro_rules! get_one_impl {
    ($this:ident, $input:ident, $model:ident, $input_converter:ident) => {{
        let converter = $input_converter::from($input);
        let model = Select::<Entity>::from(converter)
            .one(&$this.connection)
            .await
            .map_err({project_lower}_error::{project_prefix}Error::from)?
            .and_then(|model| Some($model::from(model)));

        Ok(model)
    }};
}

macro_rules! get_many_impl {
    ($this:ident, $input:ident, $model:ident, $input_converter:ident) => {{
        let converter = $input_converter::from($input.conditions);
        let select = Select::<Entity>::from(converter);

        let db_models = if let Some(pagination) = $input.pagination {
            if pagination.take == 0 {
                return Err({project_lower}_error::{project_prefix}Error::BadInput(
                    "Must have a page size of at least 1".to_string(),
                ));
            }
            select
                .order_by_desc(Column::Created)
                .paginate(&$this.connection, pagination.take)
                .fetch_page(pagination.page)
                .await
                .map_err({project_lower}_error::{project_prefix}Error::from)?
        } else {
            select
                .all(&$this.connection)
                .await
                .map_err({project_lower}_error::{project_prefix}Error::from)?
        };

        let models = db_models.into_iter().map($model::from).collect();
        Ok(models)
    }};
}

macro_rules! count_impl {
    ($this:ident, $input:ident, $model:ident, $input_converter:ident) => {{
        let converter = $input_converter::from($input);
        let count = Select::<Entity>::from(converter)
            .count(&$this.connection)
            .await
            .map_err(dockit_error::DockitError::from)?;

        Ok(dockit_common_models::search::CountOutput::from(count))
    }};
}

macro_rules! update_impl {
    ($this:ident, $input:ident, $model:ident, $input_converter:ident) => {{
        let converter = $input_converter::from($input);
        let db_model = ActiveModel::from(converter)
            .update(&$this.connection)
            .await
            .map_err({project_lower}_error::{project_prefix}Error::from)?;

        let model = $model::from(db_model);
        Ok(model)
    }};
}

macro_rules! delete_by_id_impl {
    ($this:ident, $id:ident) => {{
        let result = Entity::delete_many()
            .filter(Column::Id.eq($id))
            .exec(&$this.connection)
            .await
            .map_err({project_lower}_error::{project_prefix}Error::from)?;

        if result.rows_affected == 0 {
            Err({project_lower}_error::{project_prefix}Error::NotFound(format!(
                "No model with id '{}' found",
                $id
            )))
        } else {
            Ok({project_lower}_db_models::DeleteOutput::from(1))
        }
    }};
}

macro_rules! simple_client {
    (
        $client_name:ident,
        $trait_name:ty,
        $model_name:ty,
        { $create_input:ty, $create_input_converter:ty },
        { $get_one_input:ty, $get_one_input_converter:ty },
        { $get_many_input:ty },
        { $update_input:ty, $update_input_converter:ty },
    ) => {
        pub struct $client_name {
            connection: sea_orm::DatabaseConnection,
        }

        impl $client_name {
            pub fn new(connection: sea_orm::DatabaseConnection) -> Self {
                Self { connection }
            }
        }

        #[async_trait::async_trait]
        impl $trait_name for $client_name {
            async fn create(
                &self,
                input: $create_input,
            ) -> {project_lower}_error::{project_prefix}Result<$model_name> {
                crate::utils::client_impl::create_impl!(
                    self,
                    input,
                    $model_name,
                    $create_input_converter
                )
            }

            async fn get_one(
                &self,
                input: $get_one_input,
            ) -> {project_lower}_error::{project_prefix}Result<Option<$model_name>> {
                crate::utils::client_impl::get_one_impl!(
                    self,
                    input,
                    $model_name,
                    $get_one_input_converter
                )
            }

            async fn get_many(
                &self,
                input: $get_many_input,
            ) -> {project_lower}_error::{project_prefix}Result<Vec<$model_name>> {
                crate::utils::client_impl::get_many_impl!(
                    self,
                    input,
                    $model_name,
                    $get_one_input_converter
                )
            }

            async fn count(
                &self,
                input: $get_one_input,
            ) -> {project_lower}_error::{project_prefix}Result<{project_lower}_common_models::search::CountOutput> {
                crate::utils::client_impl::count_impl!(
                    self,
                    input,
                    $model_name,
                    $get_one_input_converter
                )
            }

            async fn update(
                &self,
                input: $update_input,
            ) -> {project_lower}_error::{project_prefix}Result<$model_name> {
                crate::utils::client_impl::update_impl!(
                    self,
                    input,
                    $model_name,
                    $update_input_converter
                )
            }

            async fn delete_by_id(
                &self, 
                id: uuid::Uuid
            ) -> {project_lower}_error::{project_prefix}Result<{project_lower}_common_models::delete::DeleteOutput> {
                crate::utils::client_impl::delete_by_id_impl!(self, id)
            }
        }
    };
}

pub(crate) use count_impl;
pub(crate) use create_impl;
pub(crate) use delete_by_id_impl;
pub(crate) use get_many_impl;
pub(crate) use get_one_impl;
pub(crate) use simple_client;
pub(crate) use update_impl;
