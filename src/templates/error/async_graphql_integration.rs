#![cfg(feature = "async_graphql")]

use crate::{error::{project_prefix}Error, error_data::{project_prefix}ErrorData};
use async_graphql::{Error as GraphqlError, ErrorExtensionValues, ErrorExtensions, FieldError};

impl {project_prefix}Error {
    pub fn extended(other: Self) -> FieldError {
        other.extend()
    }

    fn set_error_extension(&self, ext: &mut ErrorExtensionValues) {
        let error_data = {project_prefix}ErrorData::from(self);

        eprintln!("[ERROR] {:?}", error_data.message());

        let message = if self.status_code() == {project_prefix}Error::INTERNAL_ERROR_CODE {
            "An internal error occurred".to_string()
        } else {
            error_data.message()
        };

        ext.set("message", message);
        ext.set("status_string", error_data.status_string());
        ext.set("status_code", error_data.status_code());
    }
}

impl ErrorExtensions for {project_prefix}Error {
    fn extend(&self) -> GraphqlError {
        GraphqlError::new(self.to_string()).extend_with(|_err, ext| match self {
            Self::BadInput(_) => self.set_error_extension(ext),
            Self::Unauthenticated(_) => self.set_error_extension(ext),
            Self::Unauthorized(_) => self.set_error_extension(ext),
            Self::NotFound(_) => self.set_error_extension(ext),
            Self::Conflict(_) => self.set_error_extension(ext),
            Self::Other(_) => self.set_error_extension(ext),
        })
    }
}
