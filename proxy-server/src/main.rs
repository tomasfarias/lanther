use std::net::ToSocketAddrs;

use actix_web::client::Client;
use actix_web::http::{header, StatusCode};
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use chrono::prelude::*;
use clap::{value_t, Arg};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize)]
struct ApiKey {
    #[serde(rename = "_id")]
    id: String,
    key: Uuid,
    disabled: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct ApiKeyResponse {
    status: u16,
    success: bool,
    payload: ApiKey,
}

struct Urls {
    auth_url: Url,
    forward_url: Url,
}

async fn forward(
    req: HttpRequest,
    body: web::Bytes,
    urls: web::Data<Urls>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let header = req.headers().get(header::AUTHORIZATION);

    if let Some(apikey) = header {
        let mut new_auth_url = urls.get_ref().auth_url.clone();
        new_auth_url.set_path(&format!("/apikeys/{}", apikey.to_str().unwrap()));

        let mut res = client.get(new_auth_url.as_str()).send().await?;

        if res.status() != StatusCode::OK {
            return Err(error::ErrorUnauthorized("APIKey could not be validated"));
        }

        let apikey_res: ApiKeyResponse = res.json().await?;
        if apikey_res.payload.disabled == true {
            return Err(error::ErrorUnauthorized("APIKey is disabled"));
        }
    } else {
        return Err(error::ErrorUnauthorized("APIKey is required"));
    }

    log::info!("Request authorized");
    let mut new_url = urls.get_ref().forward_url.clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = clap::App::new("HTTP Proxy")
        .arg(
            Arg::with_name("listen_addr")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("listen_port")
                .takes_value(true)
                .value_name("LISTEN PORT")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("forward_addr")
                .takes_value(true)
                .value_name("FWD ADDR")
                .index(3)
                .required(true),
        )
        .arg(
            Arg::with_name("forward_port")
                .takes_value(true)
                .value_name("FWD PORT")
                .index(4)
                .required(true),
        )
        .arg(
            Arg::with_name("auth_addr")
                .takes_value(true)
                .value_name("AUTH ADDR")
                .index(5)
                .required(true),
        )
        .arg(
            Arg::with_name("auth_port")
                .takes_value(true)
                .value_name("AUTH PORT")
                .index(6)
                .required(true),
        )
        .get_matches();

    let listen_addr = matches.value_of("listen_addr").unwrap();
    let listen_port = value_t!(matches, "listen_port", u16).unwrap_or_else(|e| e.exit());

    let forwarded_addr = matches.value_of("forward_addr").unwrap();
    let forwarded_port = value_t!(matches, "forward_port", u16).unwrap_or_else(|e| e.exit());

    let auth_addr = matches.value_of("auth_addr").unwrap();
    let auth_port = value_t!(matches, "auth_port", u16).unwrap_or_else(|e| e.exit());

    let forward_url = Url::parse(&format!(
        "http://{}",
        (forwarded_addr, forwarded_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap(),
    ))
    .unwrap();

    let auth_url = Url::parse(&format!(
        "http://{}",
        (auth_addr, auth_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap(),
    ))
    .unwrap();

    log::info!("Listening on: {}:{}", listen_addr, listen_port);
    log::info!("Forwarding to: {}", forward_url);
    log::info!("Authenticating on: {}", auth_url);

    HttpServer::new(move || {
        App::new()
            .data(Client::new())
            .data(Urls {
                auth_url: auth_url.clone(),
                forward_url: forward_url.clone(),
            })
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(forward))
    })
    .bind((listen_addr, listen_port))?
    .system_exit()
    .run()
    .await
}
