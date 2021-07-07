use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::client::Client;
use actix_web::http::{header, StatusCode};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, error, Error};
use chrono::prelude::*;
use futures::future::{ok, Ready};
use futures::Future;
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

pub struct Authorized(Rc<Inner>);

struct Inner {
    client: Client,
    auth_url: Url,
}

impl Authorized {
    /// Initialize the Authorized service with an authorization URL and a HTTP client
    /// to communicate with the authorization server
    pub fn new(auth_url: &Url) -> Authorized {
        let client = Client::new();
        let new_url = auth_url.clone();

        Authorized(Rc::new(Inner {
            client: client,
            auth_url: new_url,
        }))
    }
}

impl<S, B> Transform<S> for Authorized
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizedMiddleware {
            service,
            inner: self.0.clone(),
        })
    }
}

#[derive(Deserialize, Debug)]
struct ApiKey {
    #[serde(rename = "_id")]
    id: String,
    key: Uuid,
    disabled: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
struct ApiKeyResponse {
    status: u16,
    success: bool,
    payload: ApiKey,
}

pub struct AuthorizedMiddleware<S> {
    inner: Rc<Inner>,
    service: S,
}

impl<S, B> Service for AuthorizedMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        log::debug!("Checking request authorization");
        let headers = req.headers().clone();
        let fut = self.service.call(req);
        let mut full_auth_url = self.inner.auth_url.clone();
        let client = self.inner.client.clone();

        Box::pin(async move {
            let header = headers
                .get(header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap());
            if let Some(apikey) = header {
                // TODO: Better error handling
                full_auth_url.set_path(&format!("/apikeys/{}", apikey));

                log::debug!("Auth url: {}", full_auth_url.as_str());
                let auth_fut = client.get(full_auth_url.as_str()).send();

                let mut res = auth_fut.await?;
                if res.status() != StatusCode::OK {
                    return Err(error::ErrorUnauthorized("APIKey could not be validated"));
                }

                let apikey_res: ApiKeyResponse = res.json().await?;
                if apikey_res.payload.disabled == true {
                    return Err(error::ErrorUnauthorized("APIKey is disabled"));
                }

                log::debug!("Request is valid!");
                Ok(fut.await?)
            } else {
                Err(error::ErrorUnauthorized("APIKey is required"))
            }
        })
    }
}
