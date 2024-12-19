//! HTTP Client

use crate::error::Error;
use async_trait::async_trait;
use dotenv::dotenv;
use exponential_backoff::Backoff;
use hyper::{
    client::connect::HttpConnector, header::HeaderValue, Body, Client as HyperClient, Method,
    Response, StatusCode,
};
use hyper_rustls::HttpsConnector;
use serde_json::Value;
use std::{error::Error as StdError, fmt, ops, thread, time::Duration};

pub mod methods;
pub mod types;

/// GET method
pub const GET: Method = Method::GET;
/// POST method
pub const POST: Method = Method::POST;
/// PUT method
pub const PUT: Method = Method::PUT;
/// DELETE method
pub const DELETE: Method = Method::DELETE;

const RETRY_ATTEMPTS: u32 = 5;

/// Represents a (Hyper) HTTP client.
#[derive(Debug)]
pub struct Client {
    asana_personal_access_token: String,
    server_url: String,
    https_client: HyperClient<HttpsConnector<HttpConnector>>,
}

/// Interface for any compatible HTTP client
#[async_trait]
pub trait HTTPClient {
    /// Send a request using the underlying HTTP client
    async fn send_request<T>(
        &self,
        method: &str,
        endpoint: &str,
        params: &[(&str, &str)],
        body: Option<String>,
    ) -> Result<(T, Value), Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug;
}

#[async_trait]
impl HTTPClient for Client {
    async fn send_request<T>(
        &self,
        method: &str,
        endpoint: &str,
        params: &[(&str, &str)],
        body: Option<String>,
    ) -> Result<(T, Value), Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let asana_personal_access_token = &self.asana_personal_access_token;
        let method = match method {
            "GET" => GET,
            "POST" => POST,
            "PUT" => PUT,
            "DELETE" => DELETE,
            &_ => GET,
        };

        match retry_with_backoff(
            self,
            &method,
            asana_personal_access_token.as_str(),
            endpoint,
            params,
            body,
        )
        .await
        {
            Ok(resp_body) => {
                let status = resp_body.status();
                let caller_type = std::any::type_name::<T>();

                let data: (Result<T, Error>, Value) = hyper::body::to_bytes(resp_body.into_body())
                    .await
                    .map_err(Error::new)
                    .map(|mut bytes| {
                        if bytes.is_empty() {
                            bytes = hyper::body::Bytes::from("{}");
                        }
                        let json = match serde_json::from_slice(&bytes) {
                            Ok(value) => Ok(value),
                            Err(err) => Err(Error::new(format!(
                                "Unable to deserialize bytes `{:?}` into `{}` / Error: {}",
                                &bytes, caller_type, err
                            ))),
                        };

                        (json, bytes)
                    })
                    .and_then(|data| {
                        let json = data.0;
                        let bytes = std::str::from_utf8(&data.1)?;
                        let json_raw: Value = serde_json::from_str(bytes)?;

                        match status {
                            StatusCode::OK => {}
                            StatusCode::CREATED => {}
                            StatusCode::NO_CONTENT => {}
                            _ => {
                                let text = format!(
                                    "Request Failure: Status: {} / Bytes: {}",
                                    status,
                                    bytes.to_string()
                                );
                                tracing::debug!(text);

                                return Err(Error::new(text));
                            }
                        };

                        Ok((json, json_raw))
                    })?;
                let decoded = data.0?;
                let raw_json = data.1;

                Ok((decoded, raw_json))
            }
            Err(err) => Err(Error::new(err)),
        }
    }
}

/// Create an instance by fetching defaults from the host ENV.
impl Default for Client {
    fn default() -> Self {
        dotenv().ok();

        let token = std::env::var("ASANA_PERSONAL_ACCESS_TOKEN")
            .expect("Asana Personal Access token expected!");
        let host = std::env::var("ASANA_BASE_URL").expect("ASANA_BASE_URL is missing!");

        Self::new(&token, &host)
    }
}

impl Client {
    /// Creates a new [`Client`] instance
    pub fn new(token: &str, server_url: &str) -> Self {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        Self {
            asana_personal_access_token: token.to_string(),
            server_url: server_url.to_string(),
            https_client: hyper::Client::builder().build::<_, hyper::Body>(https),
        }
    }

    /// Returns the 1Password Connect API token.
    pub fn token(&self) -> String {
        self.asana_personal_access_token.clone()
    }
}

#[allow(dead_code)]
struct RetryErrors<'a>(pub(crate) &'a mut Vec<String>);

#[allow(clippy::manual_try_fold)]
impl<'a> fmt::Display for RetryErrors<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().fold(Ok(()), |result, error_msg| {
            result.and_then(|_| writeln!(f, "{}", error_msg))
        })
    }
}

impl ops::Deref for RetryErrors<'_> {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Attempt exponential backoff when re-attempting requests.
async fn retry_with_backoff<'a>(
    client: &Client,
    method: &hyper::Method,
    asana_personal_access_token: &str,
    endpoint: &str,
    params: &[(&str, &str)],
    body: Option<String>,
) -> Result<Response<Body>, impl StdError> {
    let retries = RETRY_ATTEMPTS;
    let min = Duration::from_millis(100);
    let max = Duration::from_secs(20);
    let backoff = Backoff::new(retries, min, max);
    let mut retry_error_messages: Vec<String> = vec![];
    let mut retry_errors = vec![];

    for duration in &backoff {
        let url = format!("{}/{}?{}", client.server_url, endpoint, url_encode(params));
        dbg!(&url);

        let body_data = match body {
            Some(ref value) => Body::from(value.clone()),
            None => Body::empty(),
        };
        let mut req = hyper::Request::builder()
            .method(method)
            .uri(&*url)
            .body(body_data)?;

        let auth = String::from("Bearer ") + asana_personal_access_token;
        req.headers_mut()
            .insert("Accept", HeaderValue::from_str("application/json")?);
        req.headers_mut()
            .insert("Content-Type", HeaderValue::from_str("application/json")?);
        req.headers_mut()
            .insert("Authorization", HeaderValue::from_str(&auth)?);

        match client.https_client.request(req).await {
            Ok(value) => return Ok(value),
            Err(err) => {
                let error_message = format!("[ Retrying ]: Client error: {}", err);
                retry_error_messages.push(error_message);
                retry_errors.push(err);

                thread::sleep(duration)
            }
        }
    }

    let errors: Vec<&hyper::Error> = retry_errors.iter().collect();
    let mut err_vec: Vec<String> = vec![];
    for (index, item) in errors.iter().enumerate() {
        err_vec.push(format!("Error {}: {}", index, item))
    }
    if !err_vec.is_empty() {
        let error_text = err_vec.join(", ");
        return Err::<Response<Body>, Error>(Error::new(error_text));
    };

    Err::<Response<Body>, Error>(Error::new("Failed to retry request"))
    // Err(Error::new_internal_error())
}

fn url_encode(params: &[(&str, &str)]) -> String {
    params
        .iter()
        .map(|&t| {
            let (k, v) = t;
            format!("{}={}", k, v)
        })
        .fold("".to_string(), |mut acc, item| {
            acc.push_str(&item);
            acc.push('&');
            acc.replace('+', "%2B")
        })
}
