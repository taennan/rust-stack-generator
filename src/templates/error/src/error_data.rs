use crate::{project_prefix}Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct {project_prefix}ErrorData {
    message: String,
    status_code: u16,
    status_string: String,
}

impl {project_prefix}ErrorData {
    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn status_string(&self) -> String {
        self.status_string.clone()
    }
}

impl From<&{project_prefix}Error> for {project_prefix}ErrorData {
    fn from(error: &{project_prefix}Error) -> Self {
        Self {
            message: error.message(),
            status_code: error.status_code(),
            status_string: error.status_str().to_string(),
        }
    }
}
