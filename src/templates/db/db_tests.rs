#![cfg(test)]
#![cfg(feature = "mock")]

use crate::{
    common::test_utils,
    {entity_name_lowercase}::{
        database::{{entity_name}DB, {entity_name}DBTrait},
        entity::Model,
    },
};
use db_interface::{
    {entity_name_lowercase}::{{entity_name}, Create{entity_name}Input, Search{entity_name}Input, Update{entity_name}Input},
    result::DBError,
};
use sea_orm::MockExecResult;
use tokio;
use uuid::Uuid;

mod create {
    use super::*;

    #[tokio::test]
    async fn it_returns_newly_created_model() {
        let model = Model::default();
        let queried_results = vec![model.clone()];
        let connection = test_utils::mock_query_connection(queried_results);

        let actual = {entity_name}DB::new(connection)
            .create(Create{entity_name}Input::default())
            .await
            .unwrap();
        let expected = {entity_name}::from(model);

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn it_throws_not_found_on_unsuccessful_creation() {
        let queried_results: Vec<Model> = vec![];
        let connection = test_utils::mock_query_connection(queried_results);

        let result = {entity_name}DB::new(connection)
            .create(Create{entity_name}Input::default())
            .await;

        assert!(result.is_err());
        assert_eq!(result, Err(DBError::NotFound))
    }
}

mod get_one {
    use super::*;

    #[tokio::test]
    async fn it_returns_some_model_when_found() {
        let model = Model::default();
        let queried_results: Vec<Model> = vec![model.clone()];
        let connection = test_utils::mock_query_connection(queried_results);

        let actual = {entity_name}DB::new(connection)
            .get_one(Search{entity_name}Input::default())
            .await
            .unwrap();
        let expected = Some({entity_name}::from(model));

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn it_returns_none_when_no_models_are_found() {
        let queried_results: Vec<Model> = vec![];
        let connection = test_utils::mock_query_connection(queried_results);

        let actual = {entity_name}DB::new(connection)
            .get_one(Search{entity_name}Input::default())
            .await
            .unwrap();
        let expected = None;

        assert_eq!(actual, expected);
    }
}

mod get_many {
    use super::*;

    #[tokio::test]
    async fn it_returns_models_when_found() {
        let model = Model::default();
        let queried_results: Vec<Model> = vec![model.clone(), model.clone()];
        let connection = test_utils::mock_query_connection(queried_results);

        let actual = {entity_name}DB::new(connection)
            .get_many(Search{entity_name}Input::default())
            .await
            .unwrap();
        let expected = vec![{entity_name}::from(model.clone()), {entity_name}::from(model.clone())];

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn it_returns_empty_vec_when_no_models_are_found() {
        let queried_results: Vec<Model> = vec![];
        let connection = test_utils::mock_query_connection(queried_results);

        let actual = {entity_name}DB::new(connection)
            .get_many(Search{entity_name}Input::default())
            .await
            .unwrap();
        let expected = vec![];

        assert_eq!(actual, expected);
    }
}

mod update {
    use super::*;

    #[tokio::test]
    async fn it_returns_updated_model_on_successful_update() {
        let model = Model::default();
        let queried_results: Vec<Model> = vec![model.clone()];
        let connection = test_utils::mock_query_connection(queried_results);

        let actual = {entity_name}DB::new(connection)
            .update(Update{entity_name}Input::default())
            .await
            .unwrap();
        let expected = {entity_name}::from(model);

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn it_throws_not_updated_if_no_matching_model_is_found() {
        let queried_results: Vec<Model> = vec![];
        let connection = test_utils::mock_query_connection(queried_results);

        let result = {entity_name}DB::new(connection)
            .update(Update{entity_name}Input::default())
            .await;

        assert!(result.is_err());
        assert_eq!(result, Err(DBError::NotUpdated))
    }
}

mod delete_by_id {
    use super::*;

    #[tokio::test]
    async fn it_returns_ok_on_successful_deletion() {
        let execution_result = MockExecResult {
            last_insert_id: 0,
            rows_affected: 1,
        };
        let connection = test_utils::mock_exec_connection(execution_result);

        let result = {entity_name}DB::new(connection)
            .delete_by_id(Uuid::new_v4())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn it_throws_error_on_unsuccessful_deletion() {
        let execution_result = MockExecResult {
            last_insert_id: 0,
            rows_affected: 0,
        };
        let connection = test_utils::mock_exec_connection(execution_result);

        let result = {entity_name}DB::new(connection)
            .delete_by_id(Uuid::new_v4())
            .await;

        assert!(result.is_err());
        assert_eq!(result, Err(DBError::NotFound))
    }
}
