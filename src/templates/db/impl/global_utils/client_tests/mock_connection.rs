#[cfg(feature = "mock")]
use sea_orm::{DatabaseBackend, DatabaseConnection, IntoMockRow, MockDatabase, MockExecResult};

#[cfg(feature = "mock")]
pub(crate) fn mock_query_connection<T, I>(queried_results: I) -> DatabaseConnection
where
    T: IntoMockRow,
    I: IntoIterator<Item = T>,
{
    mock_multi_query_connection(vec![queried_results])
}

#[cfg(feature = "mock")]
fn mock_multi_query_connection<T, I, II>(queried_results: II) -> DatabaseConnection
where
    T: IntoMockRow,
    I: IntoIterator<Item = T>,
    II: IntoIterator<Item = I>,
{
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(queried_results)
        .into_connection()
}

#[cfg(feature = "mock")]
pub(crate) fn mock_exec_connection(execution_result: MockExecResult) -> DatabaseConnection {
    mock_multi_exec_connection(vec![execution_result])
}

#[cfg(feature = "mock")]
fn mock_multi_exec_connection<T>(execution_results: T) -> DatabaseConnection
where
    T: IntoIterator<Item = MockExecResult>,
{
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(execution_results)
        .into_connection()
}
