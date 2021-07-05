use std::boxed::Box;
use std::io::Cursor;

use actix_web::{error, get, post, web, Error, HttpResponse};
use futures::StreamExt;
use ipfs_api::IpfsClient;
use serde::Serialize;

#[get("")]
async fn index(_client: web::Data<IpfsClient>) -> HttpResponse {
    HttpResponse::Ok().body("Listening")
}

// Maps an AddResponse from the IpfsClient to a response that implements Serialize
#[derive(Serialize)]
struct IpfsResponse {
    hash: String,
    name: String,
    size: String,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("")]
async fn upload(
    mut payload: web::Payload,
    client: web::Data<IpfsClient>,
) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let data = Cursor::new(body);
    // heap allocation required otherwise cursor does not live long enough
    let boxed = Box::new(data);
    match client.add(*boxed).await {
        Ok(res) => Ok(HttpResponse::Ok().json(IpfsResponse {
            hash: res.hash,
            name: res.name,
            size: res.size,
        })),
        Err(e) => Err(error::ErrorInternalServerError(format!(
            "Something went horribly wrong: {:?}",
            e
        ))),
    }
}
