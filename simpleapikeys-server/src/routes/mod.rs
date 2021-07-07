use std::fmt::{Display, Formatter, Result as FmtResult};

use super::error::{ApiError, JsonError};
use super::models;
use actix_web::{delete, get, http::StatusCode, post, put, web, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;

#[get("")]
async fn get_apikeys(app_data: web::Data<crate::AppState>) -> Result<HttpResponse, JsonError> {
    let result = app_data.service.apikey.get_all().await;
    match result {
        Ok(keys) => Ok(HttpResponse::Ok().json(json!({
            "status": 200,
            "sucess": true,
            "payload": keys,
        }))),
        Err(e) => Err(e.into()),
    }
}

#[get("/{key}")]
async fn get_apikey(
    key: web::Path<String>,
    app_data: web::Data<crate::AppState>,
) -> Result<HttpResponse, JsonError> {
    let result = app_data.service.apikey.get_by_key(&key).await;
    match result {
        Ok(apikey) => Ok(HttpResponse::Ok().json(json!({
            "status": 200,
            "success": true,
            "payload": apikey,
        }))),
        Err(e) => Err(e.into()),
    }
}

#[post("")]
async fn create_apikey(app_data: web::Data<crate::AppState>) -> Result<HttpResponse, JsonError> {
    let apikey = models::NewApiKey::new();
    let result = app_data.service.apikey.create(apikey).await;
    match result {
        Ok(inserted) => {
            let id = inserted.inserted_id.as_object_id().ok_or(JsonError {
                msg: format!("Insert failed"),
                status: 500,
                success: false,
            })?;
            let apikey = app_data.service.apikey.get_by_id(&id).await;
            match apikey {
                Ok(key) => Ok(HttpResponse::Ok().json(json!({
                    "status": 200,
                    "success": true,
                    "payload": key,
                }))),
                Err(e) => Err(e.into()),
            }
        }
        Err(e) => Err(e.into()),
    }
}

#[put("")]
async fn update_apikey(
    apikey: web::Json<models::UpdateApiKey>,
    app_data: web::Data<crate::AppState>,
) -> Result<HttpResponse, JsonError> {
    let apikey = apikey.into_inner();
    let result = app_data.service.apikey.update(apikey).await;
    log::debug!("Result: {:?}", result);
    match result {
        Ok(res) => {
            // Result does not return an upserted_id, so we play nice by fetching by key
            // And returning the changed object
            let key = apikey.key.to_hyphenated().to_string();
            let apikey = app_data.service.apikey.get_by_key(&key).await;
            match apikey {
                Ok(key) => Ok(HttpResponse::Ok().json(json!({
                    "status": 200,
                    "success": true,
                    "payload": key,
                }))),
                Err(e) => Err(e.into()),
            }
        },
        Err(e) => Err(e.into()),
    }
}

#[delete("/{key}")]
async fn delete_apikey(
    key: web::Path<String>,
    app_data: web::Data<crate::AppState>,
) -> Result<HttpResponse, JsonError> {
    let result = app_data.service.apikey.delete_by_key(&key).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 0 {
                Err(JsonError {
                    msg: format!("Key not found: {}", key),
                    status: 404,
                    success: false,
                })
            } else {
                Ok(HttpResponse::Ok().json(json!({
                    "status": 200,
                    "success": true,
                    "payload": {
                        "count": res.deleted_count,
                    },
                })))
            }
        }
        Err(e) => Err(e.into()),
    }
}
