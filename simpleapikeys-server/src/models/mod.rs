use bson::{document::ValueAccessError, Document};
use chrono::prelude::*;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an API key as defined by a UUID
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey {
    #[serde(rename = "_id")]
    id: String,
    key: Uuid,
    disabled: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ApiKey {
    pub fn from_bson_document(doc: &Document) -> Result<Self, ValueAccessError> {
        Ok(ApiKey {
            id: doc.get_object_id("_id")?.to_hex(),
            key: Uuid::parse_str(doc.get_str("key")?).expect("Uuid parse failed"),
            disabled: doc.get_bool("disabled")?,
            created_at: *doc.get_datetime("created_at")?,
            updated_at: *doc.get_datetime("updated_at")?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewApiKey {
    pub key: Uuid,
}

impl NewApiKey {
    pub fn new() -> Self {
        NewApiKey {
            key: Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct UpdateApiKey {
    pub key: Uuid,
    pub disabled: bool,
}
