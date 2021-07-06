use actix_web::{self, web, HttpServer};
use clap::{self, Arg};
use mongodb::{options::ClientOptions, Client};
use services::ApiKeyService;

mod models;
mod routes;
mod services;
mod error;

struct ServiceContainer {
    apikey: ApiKeyService,
}

impl ServiceContainer {
    fn new(apikey: ApiKeyService) -> Self {
        ServiceContainer { apikey }
    }
}

// Application state to be shared
struct AppState {
    service: ServiceContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = clap::App::new("simpleapikeys-server")
        .version("0.1.0")
        .author("Tomas Farias Santana <tomas@tomasfarias.dev")
        .about("A SimpleAPI key server")
        .arg(
            Arg::with_name("ADDRESS")
                .help("The address the SimpleAPI key server will be listening to")
                .takes_value(true)
                .index(1)
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

    let db_address = matches
        .value_of("mongo-db-address")
        .expect("mongo-db-address is a required argument");
    let db_name = matches
        .value_of("mongo-db")
        .expect("mongo-db is a required argument");

    let client_options = ClientOptions::parse(db_address).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(db_name);
    let apikeys = db.collection("apikeys");

    let address = matches
        .value_of("ADDRESS")
        .expect("ADDRESS is a required argument");

    log::info!("Starting SimpleAPI Keys Server on: {}", address);

    HttpServer::new(move || {
        let service = ServiceContainer::new(ApiKeyService::new(apikeys.clone()));
        actix_web::App::new().service(
            web::scope("/apikeys")
                .data(AppState { service })
                .service(routes::get_apikeys)
                .service(routes::get_apikey)
                .service(routes::create_apikey)
                .service(routes::delete_apikey)
                .service(routes::update_apikey),
        )
    })
    .bind(address)?
    .run()
    .await
}
