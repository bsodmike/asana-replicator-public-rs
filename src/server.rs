use crate::{error::Error, mpsc::TxMessage, return_json, AppConfig, APP_CONFIG};
use axum::{
    extract::{self, DefaultBodyLimit, Extension, Path},
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::{get, post, Router},
};
use tokio::sync::mpsc;

use hyper::StatusCode;
use serde_json::json;
use std::fmt;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{AllowOrigin, Any, CorsLayer},
    trace,
    trace::TraceLayer,
};
use tracing::Level;

pub async fn serve(config: &AppConfig, addr: &str, handle: mpsc::Sender<TxMessage>) {
    let mut app = api_router();

    app = allow_cors(app);
    app = add_middleware(config, app, handle);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn get_middleware(config: &AppConfig, handle: mpsc::Sender<TxMessage>) -> Router {
    let mut app = api_router();
    app = add_middleware(config, app, handle);

    app
}

struct CorsOrigins<'a>(pub(crate) &'a Vec<HeaderValue>);

impl From<CorsOrigins<'_>> for AllowOrigin {
    fn from(value: CorsOrigins<'_>) -> Self {
        AllowOrigin::list(value.0.to_owned())
    }
}

impl<'a> fmt::Display for CorsOrigins<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, origin| {
            result.and_then(|_| writeln!(f, "{:?}", origin))
        })
    }
}

fn allow_cors(router: Router) -> Router {
    let origins = ["http://localhost:9001".parse().unwrap()];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_headers(Any)
        .allow_methods(Any);

    router.layer(cors)
}

fn add_middleware(config: &AppConfig, router: Router, handle: mpsc::Sender<TxMessage>) -> Router {
    router.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            .layer(CatchPanicLayer::new())
            .layer(Extension(config.clone()))
            .layer(Extension(handle))
            .layer(DefaultBodyLimit::max(20971520)),
    )
}

fn api_router() -> Router {
    Router::new()
        .route("/receive-webhook/:gid", post(handle_receive_webhook))
        .route("/health", get(handle_health_get))
}

pub async fn handle_health_get() -> Result<Response, Error> {
    Ok(return_json(json!({ "status": "success" }), None)?.into_response())
}

fn return_webhook(secret: &str) -> Result<Response<axum::body::Body>, Error> {
    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("X-Hook-Secret", secret)
        .body(json!({}).to_string().into())
        .map_err(Box::new)?;

    Ok(resp)
}

// During the handshake, the response does not contain a JSON payload.
pub async fn handle_receive_webhook(
    headers: axum::http::header::HeaderMap,
    Path(_gid): Path<u64>,
    Extension(_actor_handle): Extension<mpsc::Sender<TxMessage>>,
    payload: Option<extract::Json<serde_json::Value>>,
) -> Result<Response, Error> {
    {
        let config_lock = &mut *APP_CONFIG.lock().await;
        if let Some(_config) = config_lock {
            todo!()
        }
    }

    // TODO: Received events from Asana should be persisted to the DB.
    if let Some(payload) = payload {
        tracing::info!("Payload: {:?}", payload);
    } else {
        tracing::info!("Payload: None");
    }

    tracing::info!("Headers: {:?}", headers);
    if let Some(h) = headers.get("X-Hook-Secret") {
        if let Ok(secret) = h.to_str() {
            // TODO: the secret should be stored in a database, as this is used to verify future
            // webhook requests

            return Ok(return_webhook(secret)?.into_response());
        } else {
            return Ok(
                return_json(json!({ "status": "error: Asana secret is missing?" }), None)?
                    .into_response(),
            );
        }
    } else {
        Ok(return_json(json!({ "status": "success" }), None)?.into_response())
    }
}
