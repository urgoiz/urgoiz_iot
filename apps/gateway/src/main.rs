mod domain;
mod sensor_parser;
mod mqtt_handler;
mod mqtt_listener;
mod sqlite_repository;

use crate::mqtt_handler::MqttHandler;
use crate::sqlite_repository::SqliteRepository;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_client, eventloop) = mqtt_listener::setup_mqtt_client(
        "gateway_prod",
        "localhost",
        1883
    ).await;

    let repo = SqliteRepository::new("sqlite:gateway.db").await?;
    println!("Database initilized (SQLite).");

    let handler = MqttHandler::new(repo);
    
    println!("Gateway is running...");
    mqtt_listener::run_event_loop(eventloop, handler).await;

    Ok(())
}
