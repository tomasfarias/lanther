use actix_web::http::header;
use actix_web::HttpRequest;
use bson::{document::ValueAccessError, Document};
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

impl Request {
    pub fn from_bson_document(doc: &Document) -> Result<Self, ValueAccessError> {
        let authorization = match doc.is_null("authorization") {
            true => None,
            false => {
                let key = doc.get_str("authorization")?;
                Some(Uuid::parse_str(key).expect("Uuid parse failed"))
            },   
        };
        Ok(Request {
            id: doc.get_object_id("_id")?.to_hex(),
            method: doc.get_str("method")?.to_string(),
            path: doc.get_str("path")?.to_string(),
            authorization: authorization,
            created_at: *doc.get_datetime("created_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRequest {
    pub method: String,
    pub path: String,
    pub authorization: Option<Uuid>,
}

impl NewRequest {
    pub fn from_http_request(req: &HttpRequest) -> Result<Self, Error> {
        let auth_header = req.headers()
            .get(header::AUTHORIZATION)
            .map(|h| h.to_str().unwrap());
        let authorization = match auth_header {
            Some(key) => Some(Uuid::parse_str(key)?),
            None => None,
        };
        Ok(NewRequest {
            method: req.method().as_str().to_string(),
            path: req.path().to_string(),
            authorization: authorization,
        })
    }
}


#[derive(Deserialize, Debug)]
pub struct ApiKey {
    #[serde(rename = "_id")]
    id: String,
    pub key: Uuid,
    pub disabled: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct ApiKeyResponse {
    pub status: u16,
    pub success: bool,
    pub payload: ApiKey,
}
