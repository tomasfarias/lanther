use std::fmt::{Display, Formatter, Result as FmtResult};

use actix_web::{delete, get, http::StatusCode, post, put, web, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("MongoDB Operation failed: {source}")]
    MongoDBOperationError {
        #[from]
        source: mongodb::error::Error,
    },
    #[error("Attempted to access an invalid field in BSON document: {0}")]
    InvalidFieldError(#[from] bson::document::ValueAccessError),
    #[error("MongoDB query returned no results")]
    MongoDBEmptyResult,
}

#[derive(Debug, Serialize)]
pub struct JsonError {
    pub msg: String,
    pub status: u16,
    pub success: bool,
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let err_json = serde_json::to_string(self).unwrap();
        write!(f, "{}", err_json)
    }
}

impl ResponseError for JsonError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json2(self)
    }
}

impl From<ApiError> for JsonError {
    fn from(err: ApiError) -> Self {
        let status = match err {
            ApiError::MongoDBOperationError { source: _ } | ApiError::InvalidFieldError(_) => 500,
            ApiError::MongoDBEmptyResult => 404,
        };

        JsonError {
            msg: format!("{}", err.to_string()),
            status: status,
            success: false,
        }
    }
}
