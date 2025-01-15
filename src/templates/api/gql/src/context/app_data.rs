use {project_lower}_db_postgres::DatabaseConnector;
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct {project_prefix}AppData {
    db_connection: DatabaseConnection,
}

impl {project_prefix}AppData {
    pub async fn new(db_uri: &str) -> Self {
        Self {
            db_connection: DatabaseConnector::new(db_uri).await.connection(),
        }
    }

    pub fn db_connection(&self) -> DatabaseConnection {
        self.db_connection.clone()
    }
}
