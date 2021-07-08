use actix_web::{self, web, HttpServer};
use clap::{self, Arg};
use ipfs_api::IpfsClient;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = clap::App::new("ipfs-server")
        .version("0.1.0")
        .author("Tomas Farias Santana <tomas@tomasfarias.dev")
        .about("A simple IPFS server")
        .arg(
            Arg::with_name("ADDRESS")
                .help("The address the IPFS server will be listening to")
                .takes_value(true)
                .index(1)
                .required(true),
        )
        .get_matches();

    let address = matches
        .value_of("ADDRESS")
        .expect("ADDRESS is a required argument");

    log::info!("Starting IPFS Server on: {}", address);

    HttpServer::new(|| {
        actix_web::App::new().data(IpfsClient::default()).service(
            web::scope("/")
                .service(routes::index)
                .service(routes::upload),
        )
    })
    .bind(address)?
    .run()
    .await
}
