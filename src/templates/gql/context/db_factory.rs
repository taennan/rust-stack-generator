use {project_lower}_db_postgres::{{imports}};
use sea_orm::DatabaseConnection;

pub(crate) struct DBFactory {
    db_connection: DatabaseConnection,
}

impl DBFactory {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }

{methods}
}
