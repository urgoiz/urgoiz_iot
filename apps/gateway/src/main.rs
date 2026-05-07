mod domain;
mod sensor_parser;
mod mqtt_handler;
mod mqtt_listener;
mod sqlite_repository;
mod config;
mod error;

use crate::mqtt_handler::MqttHandler;
use config::Settings;
use std::error::Error;
use tracing_subscriber::{prelude::*, EnvFilter};


fn setup_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    setup_tracing();
    tracing::info!("Starting IoT Gateway...");

    let settings = Settings::new()?;

    tracing ::info!("Configuration loaded: {:?}", settings);

    let repo = sqlite_repository::SqliteRepository::new(&settings.database.url)
        .await
        .map_err(|e| {
            tracing::error!("Database initialization failed: {}", e);
            e
        })?;
    tracing::info!("Database initialized at {}:", settings.database.url);

    let (client, eventloop) = mqtt_listener::setup_mqtt_client(
        "gateway_prod",
        &settings.mqtt.host,
        settings.mqtt.port
    ).await;

    let handler = MqttHandler::new(repo);
    
    tracing::info!("Gateway is ready. Press Ctrl+C to exit.");

    tokio::select! {
        result = mqtt_listener::run_event_loop(eventloop, handler) => {
            if let Err(e) = result {
                tracing::error!("Event loop stopped due to fatal error: {}", e);
                return Err(e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutdown signal received, exiting...");
            client.disconnect().await?;
        }
    }
    tracing::info!("IoT Gateway has shut down gracefully.");

    Ok(())
}
