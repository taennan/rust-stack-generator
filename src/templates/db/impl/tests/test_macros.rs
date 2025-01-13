macro_rules! create {
    ($db:ident, $entity_model:ident, $db_model:ident, $input:ident) => {
        mod create {
            use super::*;

            #[tokio::test]
            async fn it_returns_newly_created_model() {
                let model = $entity_model::default();
                let queried_results = vec![model.clone()];
                let connection = crate::utils::test_utils::mock_query_connection(queried_results);

                let actual = $db::new(connection)
                    .create($input::default())
                    .await
                    .unwrap();
                let expected = $db_model::from(model);

                assert_eq!(actual, expected);
            }

            #[tokio::test]
            async fn it_throws_not_found_on_unsuccessful_creation() {
                let queried_results: Vec<$entity_model> = vec![];
                let connection = crate::utils::test_utils::mock_query_connection(queried_results);

                let result = $db::new(connection).create($input::default()).await;

                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ))
            }
        }
    };
}

macro_rules! get_one {
    ($db:ident, $entity_model:ident, $db_model:ident, $input:ident) => {
        mod get_one {
            use super::*;

            #[tokio::test]
            async fn it_returns_some_model_when_found() {
                let model = $entity_model::default();
                let queried_results: Vec<$entity_model> = vec![model.clone()];
                let connection = crate::utils::test_utils::mock_query_connection(queried_results);

                let actual = $db::new(connection)
                    .get_one($input::default())
                    .await
                    .unwrap();
                let expected = Some($db_model::from(model));

                assert_eq!(actual, expected);
            }

            #[tokio::test]
            async fn it_returns_none_when_no_models_are_found() {
                let queried_results: Vec<$entity_model> = vec![];
                let connection = crate::utils::test_utils::mock_query_connection(queried_results);

                let actual = $db::new(connection)
                    .get_one($input::default())
                    .await
                    .unwrap();
                let expected = None;

                assert_eq!(actual, expected);
            }
        }
    };
}

macro_rules! update {
    ($db:ident, $entity_model:ident, $db_model:ident, $input:ident) => {
        mod update {
            use super::*;

            #[tokio::test]
            async fn it_returns_updated_model_on_successful_update() {
                let model = $entity_model::default();
                let queried_results: Vec<$entity_model> = vec![model.clone()];
                let connection = crate::utils::test_utils::mock_query_connection(queried_results);

                let actual = $db::new(connection)
                    .update($input::default())
                    .await
                    .unwrap();
                let expected = $db_model::from(model);

                assert_eq!(actual, expected);
            }

            #[tokio::test]
            async fn it_throws_not_found_if_model_is_not_found() {
                let queried_results: Vec<$entity_model> = vec![];
                let connection = crate::utils::test_utils::mock_query_connection(queried_results);

                let result = $db::new(connection).update($input::default()).await;

                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ))
            }
        }
    };
}

macro_rules! delete_by_id {
    ($db:ident) => {
        mod delete_by_id {
            use super::*;

            #[tokio::test]
            async fn it_returns_ok_on_successful_deletion() {
                let execution_result = sea_orm::MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                };
                let connection = crate::utils::test_utils::mock_exec_connection(execution_result);

                let result = $db::new(connection)
                    .delete_by_id(uuid::Uuid::new_v4())
                    .await;

                assert!(result.is_ok());
            }

            #[tokio::test]
            async fn it_throws_error_on_unsuccessful_deletion() {
                let execution_result = sea_orm::MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 0,
                };
                let connection = crate::utils::test_utils::mock_exec_connection(execution_result);

                let result = $db::new(connection)
                    .delete_by_id(uuid::Uuid::new_v4())
                    .await;

                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ))
            }
        }
    };
}

pub(crate) use create;
pub(crate) use delete_by_id;
pub(crate) use get_one;
pub(crate) use update;
