use std::fmt::{Display, Formatter, Result as FmtResult};

pub type {project_prefix}Result<T> = Result<T, {project_prefix}Error>;

impl<T> From<{project_prefix}Error> for {project_prefix}Result<T> {
    fn from(error: {project_prefix}Error) -> Self {
        Err(error)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum {project_prefix}Error {
    BadInput(String),
    Unauthenticated(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    Other(String),
}

impl {project_prefix}Error {
    pub const INTERNAL_ERROR_CODE: u16 = 500;

    pub fn message(&self) -> String {
        match self.clone() {
            Self::BadInput(m) => m,
            Self::Unauthenticated(m) => m,
            Self::Unauthorized(m) => m,
            Self::NotFound(m) => m,
            Self::Conflict(m) => m,
            Self::Other(m) => m,
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            Self::BadInput(_) => 400,
            Self::Unauthenticated(_) => 401,
            Self::Unauthorized(_) => 403,
            Self::NotFound(_) => 404,
            Self::Conflict(_) => 409,
            Self::Other(_) => Self::INTERNAL_ERROR_CODE,
        }
    }

    pub fn status_str(&self) -> &str {
        match self {
            Self::BadInput(_) => "BAD_INPUT",
            Self::Unauthenticated(_) => "UNAUTHORIZED",
            Self::Unauthorized(_) => "UNAUTHENTICATED",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Conflict(_) => "CONFLICT",
            Self::Other(_) => "INTERNAL",
        }
    }
}

impl Display for {project_prefix}Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BadInput(s) => write!(f, "{}", s),
            Self::Unauthenticated(s) => write!(f, "{}", s),
            Self::Unauthorized(s) => write!(f, "{}", s),
            Self::NotFound(s) => write!(f, "{}", s),
            Self::Conflict(s) => write!(f, "{}", s),
            Self::Other(s) => write!(f, "{}", s),
        }
    }
}
