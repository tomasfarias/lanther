use chrono::Utc;
use futures::StreamExt;
use mongodb::results::InsertOneResult;
use mongodb::{bson::doc, bson::Bson, Collection};

use super::error::ApiError;
use super::models;

/// Service to operate on API keys in MongoDB
#[derive(Clone)]
pub struct RequestService {
    collection: Collection,
}

impl RequestService {
    pub fn new(collection: Collection) -> Self {
        RequestService { collection }
    }

    /// Create a new entry for a Request
    pub async fn create(&self, req: models::NewRequest) -> Result<InsertOneResult, ApiError> {
        let document = match req.authorization {
            Some(a) => {
                doc! {
                    "method": req.method.clone(),
                    "path": req.path.clone(),
                    "authorization": a.to_hyphenated().to_string(),
                    "created_at": Utc::now(),
                }
            }
            None => {
                doc! {
                    "method": req.method.clone(),
                    "path": req.path.clone(),
                    "authorization": Bson::Null,
                    "created_at": Utc::now(),
                }
            }
        };
        let result = self.collection.insert_one(document, None).await?;
        Ok(result)
    }

    /// Get all existing Requests
    pub async fn get_all(&self) -> Result<Vec<models::Request>, ApiError> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut result: Vec<models::Request> = Vec::new();
        while let Some(doc) = cursor.next().await {
            log::debug!("doc: {:?}", doc);
            result.push(models::Request::from_bson_document(&doc?)?);
        }
        Ok(result)
    }

    /// Find all existing Requests by API key
    pub async fn get_by_key(&self, key: &str) -> Result<Vec<models::Request>, ApiError> {
        let filter = doc! {
            "authorization": key.to_string(),
        };
        let mut cursor = self.collection.find(filter, None).await?;
        let mut result: Vec<models::Request> = Vec::new();
        while let Some(doc) = cursor.next().await {
            log::debug!("doc: {:?}", doc);
            result.push(models::Request::from_bson_document(&doc?)?);
        }
        Ok(result)
    }
}
