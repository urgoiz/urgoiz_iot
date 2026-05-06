mod domain;
mod sensor_parser;
mod mqtt_handler;
mod mqtt_listener;
mod sqlite_repository;
mod config;

use crate::mqtt_handler::MqttHandler;
use std::error::Error;
use tracing_subscriber::{prelude::*, EnvFilter};
use config::Settings;


fn setup_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_client, eventloop) = mqtt_listener::setup_mqtt_client(
        "gateway_prod",
        "localhost",
        1883
    ).await;

    setup_tracing();
    let settings = Settings::new().expect("Failed to load configuration");
    tracing ::info!("Configuration loaded: {:?}", settings);

    let repo = sqlite_repository::SqliteRepository::new(&settings.database.url)
        .await
        .expect("Failed to initialize database");
    tracing::info!("Database initialized (SQLite).");

    let handler = MqttHandler::new(repo);
    
    tracing::info!("Gateway is running...");
    mqtt_listener::run_event_loop(eventloop, handler).await;

    Ok(())
}
