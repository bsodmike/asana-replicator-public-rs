use crate::{config::AppConfig, mpsc::ChannelReceiver};
use axum::{body::Body, response::Response};
use error::Error;
use hyper::StatusCode;
use mpsc::TxMessage;
use serde_json::Value;
use std::sync::LazyLock;
use std::{env, sync::Arc};
use tokio::sync::{mpsc as tokio_mpsc, Mutex};

use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

pub mod asana;
pub mod config;
pub mod error;
pub mod mpsc;
pub mod server;
pub mod utils;

#[allow(clippy::type_complexity)]
static APP_CONFIG: LazyLock<Mutex<Option<Box<AppConfig>>>> =
    LazyLock::new(|| Mutex::new(Some(Box::default())));

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenv::from_filename(".env.development").ok();

    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "asana_replicator_public=info,tower_http=trace,tokio=trace,runtime=trace",
        )
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .init();

    let new_config = config::config().await.expect("Loads config");
    tracing::info!("Config: {:#?}", new_config);
    let arc_config = Arc::new(new_config.clone());

    {
        let config_lock = &mut *APP_CONFIG.lock().await;
        if let Some(config) = config_lock {
            *config = Box::new(new_config);
        }
    } // This block ensures we drop the lock here.

    // Spin up our API
    let addr: &str = &env::var("SERVER_BIND_HOST_PORT").unwrap_or("0.0.0.0:3001".to_string());
    tracing::info!("[ OK ]: Listening on {}", addr);

    // Setup mpsc
    let (tx, receiver) = tokio_mpsc::channel::<TxMessage>(32);
    let mut rx = ChannelReceiver::new(receiver);

    // let config = config::config().await.expect("Loads config");
    let backend = async move { server::serve(&arc_config, &addr, tx).await };

    // single consumer
    tokio::spawn(async move {
        if let Err(err) = rx.run().await {
            let err_message = format!("Error when spawning single consumer! {}", err);
            tracing::error!("{}", err_message);

            return Err(Error::new(err_message));
        }

        Ok(())
    });

    tokio::join!(backend);

    Ok(())
}

fn return_json(json: Value, status: Option<StatusCode>) -> Result<Response<Body>, Error> {
    let status = status.unwrap_or(StatusCode::OK);

    let resp = Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(json.to_string().into())
        .map_err(Box::new)?;

    Ok(resp)
}
