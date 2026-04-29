mod domain;
mod sensor_parser;
mod mqtt_handler;
mod mqtt_listener;

use crate::mqtt_handler::MqttHandler;
use crate::domain::{SensorRepository, SensorData, SensorError};
use async_trait::async_trait;

struct TempRepo;

#[async_trait]
impl SensorRepository for TempRepo {
    async fn save_reading(&self, _data: SensorData) -> Result<(), SensorError> {
        // Implementation for saving sensor reading
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("IoT Gateway is starting...");

    let (_client, eventloop) = mqtt_listener::setup_mqtt_client("gateway", "localhost", 1883).await;

    let repo = TempRepo;
    let handler = MqttHandler::new(repo);
    
    mqtt_listener::run_event_loop(eventloop, handler).await;
}
