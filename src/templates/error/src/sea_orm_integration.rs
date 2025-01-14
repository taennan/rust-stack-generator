#![cfg(feature = "sea_orm")]

use crate::{project_prefix}Error;
use sea_orm::DbErr;

impl From<DbErr> for {project_prefix}Error {
    fn from(error: DbErr) -> Self {
        // TODO: Old {project_prefix} code, find out what this is for
        let is_sql_error = error.sql_err().is_some();
        if is_sql_error {
            return Self::Conflict("Something conflicted in SQL query".to_string());
        }

        match error {
            DbErr::RecordNotFound(message) => Self::NotFound(message),
            DbErr::RecordNotInserted => {
                Self::NotFound("Failed to insert model in database".to_string())
            }
            DbErr::RecordNotUpdated => {
                Self::NotFound("Failed to update model in database".to_string())
            }
            _ => Self::Other(error.to_string()),
        }
    }
}
