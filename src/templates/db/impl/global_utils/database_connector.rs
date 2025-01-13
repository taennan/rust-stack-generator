#![cfg(not(feature = "mock"))]

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct DatabaseConnector {
    database_connection: DatabaseConnection,
}

impl DatabaseConnector {
    pub async fn new(uri: &str) -> Self {
        let connection_options = ConnectOptions::new(uri);
        let database_connection = Database::connect(connection_options)
            .await
            .expect("Could not connect to database :(");

        Self {
            database_connection,
        }
    }

    pub fn connection(&self) -> DatabaseConnection {
        self.database_connection.clone()
    }
}
