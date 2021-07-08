use std::net::ToSocketAddrs;

use actix_web::client::Client;
use actix_web::{middleware, web, App, HttpServer};
use clap::{value_t, Arg};
use mongodb::{options::ClientOptions, self};
use url::Url;
use services::RequestService;

mod middlewares;
mod models;
mod services;
mod routes;
mod error;

struct ServiceContainer {
    request: RequestService,
}

impl ServiceContainer {
    fn new(request: RequestService) -> Self {
        ServiceContainer { request }
    }
}

// Application state to be shared
pub struct AppState {
    service: ServiceContainer,
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
        .arg(
            Arg::with_name("mongo-db-address")
                .long("mongo-db-address")
                .help("The address of a Mongo DB instance")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("mongo-db")
                .long("mongo-db")
                .help("The name of a Mongo DB")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let listen_addr = matches.value_of("listen_addr").unwrap();
    let listen_port = value_t!(matches, "listen_port", u16).unwrap_or_else(|e| e.exit());

    let forwarded_addr = matches.value_of("forward_addr").unwrap();
    let forwarded_port = value_t!(matches, "forward_port", u16).unwrap_or_else(|e| e.exit());

    let auth_addr = matches.value_of("auth_addr").unwrap();
    let auth_port = value_t!(matches, "auth_port", u16).unwrap_or_else(|e| e.exit());

    let db_address = matches
        .value_of("mongo-db-address")
        .expect("mongo-db-address is a required argument");
    let db_name = matches
        .value_of("mongo-db")
        .expect("mongo-db is a required argument");

    let client_options = ClientOptions::parse(db_address).await.unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database(db_name);
    let requests = db.collection("requests");
    
    let auth_url = Url::parse(&format!(
        "http://{}",
        (auth_addr.to_owned().as_str(), auth_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap(),
    ))
        .unwrap();

    let forward_url = Url::parse(&format!(
        "http://{}",
        (forwarded_addr, forwarded_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap(),
    ))
    .unwrap();

    log::info!("Listening on: {}:{}", listen_addr, listen_port);
    log::info!("Forwarding to: {}", forward_url);
    log::info!("Authenticating on: {}:{}", auth_addr, auth_port);

    HttpServer::new(move || {
        let service = ServiceContainer::new(RequestService::new(requests.clone()));

        App::new()
            .wrap(middleware::Logger::default())
            .data(AppState { service })
            .service(routes::get_all_requests)
            .service(routes::get_requests_by_key)
            .service(
                web::scope("/")
                    .data(Client::new())
                    .data(forward_url.clone())
                    .wrap(middlewares::Authorized::new(&auth_url))
                    .default_service(web::route().to(routes::forward))
            )
    })
    .bind((listen_addr, listen_port))?
    .system_exit()
    .run()
    .await
}
