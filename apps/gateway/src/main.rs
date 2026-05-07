mod domain;
mod sensor_parser;
mod mqtt_handler;
mod mqtt_listener;
mod sqlite_repository;
mod config;
mod error;
mod app;

use crate::app::GatewayApp;
use crate::mqtt_handler::MqttHandler;
use config::Settings;
use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    setup_tracing();
    let settings = Settings::new()?;

    let repo = Arc::new(sqlite_repository::SqliteRepository::new(&settings.database.url).await?);

    let handler = MqttHandler::new(repo);
    
    let app = GatewayApp::new(settings, handler);
    app.run().await
}

fn setup_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}