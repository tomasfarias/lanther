use bson::oid::ObjectId;
use chrono::Utc;
use futures::StreamExt;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{bson::doc, Collection};

use super::error::ApiError;
use super::models;

/// Service to operate on API keys in MongoDB
#[derive(Clone)]
pub struct ApiKeyService {
    collection: Collection,
}

impl ApiKeyService {
    pub fn new(collection: Collection) -> Self {
        ApiKeyService { collection }
    }

    /// Create an API key
    pub async fn create(&self, apikey: models::NewApiKey) -> Result<InsertOneResult, ApiError> {
        let document = doc! {
            "key": apikey.key.to_hyphenated().to_string(),
            "updated_at": Utc::now(),
            "created_at": Utc::now(),
            "disabled": false,
        };
        let result = self.collection.insert_one(document, None).await?;
        Ok(result)
    }

    /// Update an API key given the key itself
    pub async fn update(&self, apikey: models::UpdateApiKey) -> Result<UpdateResult, ApiError> {
        let filter = doc! {
            "key": apikey.key.to_hyphenated().to_string(),
        };
        let document = doc! {
            "$set": {
                "updated_at": Utc::now(),
                "disabled": apikey.disabled,
            }
        };
        let result = self.collection.update_one(filter, document, None).await?;
        Ok(result)
    }

    /// Delete an API key given the key itself
    pub async fn delete_by_key(&self, key: &str) -> Result<DeleteResult, ApiError> {
        let filter = doc! {
            "key": key.to_string(),
        };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result)
    }

    /// Get all existing API keys
    pub async fn get_all(&self) -> Result<Vec<models::ApiKey>, ApiError> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut result: Vec<models::ApiKey> = Vec::new();
        while let Some(doc) = cursor.next().await {
            log::debug!("doc: {:?}", doc);
            result.push(models::ApiKey::from_bson_document(&doc?)?);
        }
        Ok(result)
    }

    /// Find an existing API key by the key itself
    pub async fn get_by_key(&self, key: &str) -> Result<models::ApiKey, ApiError> {
        let filter = doc! {
            "key": key.to_string(),
        };
        let doc = self.collection.find_one(filter, None).await?;
        let result = doc.ok_or(ApiError::MongoDBEmptyResult)?;
        let apikey = models::ApiKey::from_bson_document(&result)?;

        Ok(apikey)
    }

    /// Find an existing API key by the ObjectID
    pub async fn get_by_id(&self, id: &ObjectId) -> Result<models::ApiKey, ApiError> {
        log::debug!("id:{}", id);

        let filter = doc! {
            "_id": id,
        };
        let doc = self.collection.find_one(filter, None).await?;
        let result = doc.ok_or(ApiError::MongoDBEmptyResult)?;
        let apikey = models::ApiKey::from_bson_document(&result)?;

        Ok(apikey)
    }
}
