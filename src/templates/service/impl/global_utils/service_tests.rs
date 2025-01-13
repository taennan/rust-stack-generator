macro_rules! simple_create_tests {
    ($service:ident, [($($mock_dep:ident),*), $mock_db:ident], $model:ident, $input:ident) => {
        mod create {
            use super::*;

            #[tokio::test]
            async fn it_returns_created_model() {
                let model = $model::default();
                let expected = model.clone();

                let mut db = $mock_db::new();
                db.expect_create().return_once(|_| Ok(model));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );

                let actual = service.create($input::default()).await.unwrap();
                assert_eq!(actual, expected);
            }

            #[tokio::test]
            async fn it_throws_if_not_found() {
                let mut db = $mock_db::new();
                db.expect_create()
                    .return_once(|_| Err({project_lower}_error::{project_prefix}Error::NotFound("".to_string())));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );
                let result = service.create($input::default()).await;

                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ));
            }
        }
    };
}

macro_rules! simple_get_by_id_tests {
    ($service:ident, [($($mock_dep:ident),*), $mock_db:ident], $model:ident) => {
        mod get_by_id_or_throw {
            use super::*;

            #[tokio::test]
            async fn it_returns_model_if_found() {
                let db_model = $model::default();
                let expected = db_model.clone();

                let mut db = $mock_db::new();
                db.expect_get_one().return_once(|_| Ok(Some(db_model)));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );

                let actual = service
                    .get_by_id_or_throw(uuid::Uuid::new_v4())
                    .await
                    .unwrap();
                assert_eq!(actual, expected);
            }

            #[tokio::test]
            async fn it_throws_if_not_found() {
                let mut db = $mock_db::new();
                db.expect_get_one().return_once(|_| Ok(None));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );
                let result = service.get_by_id_or_throw(uuid::Uuid::new_v4()).await;

                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ));
            }
        }
    };
}

macro_rules! simple_get_one_tests {
    ($service:ident, [($($mock_dep:ident),*), $mock_db:ident], $model:ident, $input:ident) => {
        mod get_one {
            use super::*;

            #[tokio::test]
            async fn it_returns_some_model_if_found() {
                let db_model = $model::default();
                let expected = Some(db_model.clone());

                let mut db = $mock_db::new();
                db.expect_get_one().return_once(|_| Ok(Some(db_model)));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );

                let actual = service.get_one($input::default()).await.unwrap();
                assert_eq!(actual, expected);
            }

            #[tokio::test]
            async fn it_returns_none_if_not_found() {
                let mut db = $mock_db::new();
                db.expect_get_one().return_once(|_| Ok(None));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );

                let actual = service.get_one($input::default()).await.unwrap();
                let expected = None;
                assert_eq!(actual, expected);
            }
        }
    };
}

macro_rules! simple_update_tests {
    ($service:ident, [($($mock_dep:ident),*), $mock_db:ident], $model:ident, $input:ident) => {
        mod update {
            use super::*;

            #[tokio::test]
            async fn it_returns_model_on_successful_update() {
                let db_model = $model::default();
                let expected = db_model.clone();

                let mut db = $mock_db::new();
                db.expect_update().return_once(|_| Ok(db_model));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );

                let actual = service.update($input::default()).await.unwrap();
                assert_eq!(actual, expected);
            }

            #[tokio::test]
            async fn it_throws_if_not_found() {
                let mut db = $mock_db::new();
                db.expect_update()
                    .return_once(|_| Err({project_lower}_error::{project_prefix}Error::NotFound("".to_string())));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );
                let result = service.update($input::default()).await;

                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ));
            }

            #[tokio::test]
            async fn it_throws_if_not_updated() {
                let mut db = $mock_db::new();
                db.expect_update()
                    .return_once(|_| Err({project_lower}_error::{project_prefix}Error::NotFound("".to_string())));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );
                let result = service.update($input::default()).await;

                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ));
            }
        }
    };
}

macro_rules! simple_delete_by_id_tests {
    ($service:ident, [($($mock_dep:ident),*), $mock_db:ident]$(,)?) => {
        mod delete_by_id {
            use super::*;

            #[tokio::test]
            async fn it_returns_ok_on_successful_deletion() {
                let mut db = $mock_db::new();
                db.expect_delete_by_id().return_once(|_| Ok(1));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );

                let output = service.delete_by_id(uuid::Uuid::new_v4()).await.unwrap();
                assert_eq!(output.amount_deleted, 1);
            }

            #[tokio::test]
            async fn it_throws_if_nothing_was_deleted() {
                let mut db = $mock_db::new();
                db.expect_delete_by_id().return_once(|_| Ok(0));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );

                let result = service.delete_by_id(uuid::Uuid::new_v4()).await;
                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ));
            }

            #[tokio::test]
            async fn it_throws_on_deletion_error() {
                let mut db = $mock_db::new();
                db.expect_delete_by_id()
                    .return_once(|_| Err({project_lower}_error::{project_prefix}Error::NotFound("".to_string())));

                let service = $service::new(
                    $(
                        $mock_dep(),
                    )*
                    db,
                    uuid::Uuid::new_v4()
                );

                let result = service.delete_by_id(uuid::Uuid::new_v4()).await;
                assert!(result.is_err());
                assert!(matches!(
                    result.unwrap_err(),
                    {project_lower}_error::{project_prefix}Error::NotFound(_)
                ));
            }
        }
    };
}

pub(crate) use simple_create_tests;
pub(crate) use simple_delete_by_id_tests;
pub(crate) use simple_get_by_id_tests;
pub(crate) use simple_get_one_tests;
pub(crate) use simple_update_tests;
