use actix_web::client::Client;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use serde_json::json;
use url::Url;

use super::error::JsonError;
use super::models;

pub async fn forward(
    req: HttpRequest,
    body: web::Bytes,
    forward_url: web::Data<Url>,
    app_data: web::Data<crate::AppState>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let request = models::NewRequest::from_http_request(&req).unwrap();
    let result = app_data.service.request.create(request).await;

    // TODO: Figure out a better way to log requests, perhaps with a middelware?
    match result {
        Ok(_) => log::info!("Request logged"),
        Err(e) => log::error!("Error logging request: {}", e),
    };

    log::info!("Processing request");
    let mut new_url = forward_url.as_ref().clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    log::debug!("Forwarded request URL: {:?}", new_url);
    // TODO: This forwarded implementation is incomplete as it only handles the inofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(res.body().await?))
}

#[get("/requests")]
pub async fn get_all_requests(
    app_data: web::Data<crate::AppState>,
) -> Result<HttpResponse, JsonError> {
    let result = app_data.service.request.get_all().await;
    match result {
        Ok(requests) => Ok(HttpResponse::Ok().json(json!({
            "status": 200,
            "sucess": true,
            "payload": requests,
        }))),
        Err(e) => Err(e.into()),
    }
}

#[get("/requests/{key}")]
pub async fn get_requests_by_key(
    key: web::Path<String>,
    app_data: web::Data<crate::AppState>,
) -> Result<HttpResponse, JsonError> {
    let result = app_data.service.request.get_by_key(&key).await;
    match result {
        Ok(requests) => Ok(HttpResponse::Ok().json(json!({
            "status": 200,
            "success": true,
            "payload": requests,
        }))),
        Err(e) => Err(e.into()),
    }
}
