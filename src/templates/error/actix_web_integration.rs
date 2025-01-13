#![cfg(feature = "actix_web")]

use crate::{error::{project_prefix}Error, error_data::{project_prefix}ErrorData};
use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

impl ResponseError for {project_prefix}Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadInput(_) => StatusCode::BAD_REQUEST,
            Self::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
            Self::Unauthorized(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = ResponseError::status_code(self);
        HttpResponse::build(status_code)
            .insert_header(ContentType::json())
            .json({project_prefix}ErrorData::from(self))
    }
}
