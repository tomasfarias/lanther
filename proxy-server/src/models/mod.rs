use actix_web::http::header;
use actix_web::HttpRequest;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::{Error, Uuid};

/// A simple model for an HTTP Request
/// Could be extended to support more information, like aditional headers
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    #[serde(rename = "_id")]
    id: String,
    method: String,
    path: String,
    authorization: Option<Uuid>,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRequest {
    #[serde(rename = "_id")]
    method: String,
    path: String,
    authorization: Option<Uuid>,
}

impl Request {
    pub fn from_http_request(req: &HttpRequest) -> Result<Self, Error> {
        let auth_header = req.headers().get(header::AUTHORIZATION);
        let authorization = match auth_header {
            Some(key) => Some(Uuid::parse_str(key)?),
            None => None,
        };
        Ok(Request {
            method: req.method().as_str().to_string(),
            path: req.path().to_string(),
            authorization: authorization,
        })
    }
}
