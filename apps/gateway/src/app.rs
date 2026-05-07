use crate::config::Settings;
use crate::mqtt_handler::MqttHandler;
use crate::mqtt_listener;
use crate::domain::SensorRepository;
use std::time::Duration;
use tokio::time::sleep;


pub struct GatewayApp<R: SensorRepository + Clone> {
    settings: Settings,
    handler: MqttHandler<R>,
}

impl<R: SensorRepository + Clone> GatewayApp<R> {
    pub fn new(settings: Settings, handler: MqttHandler<R>) -> Self {
        Self { settings, handler }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut retry_delay = Duration::from_secs(1);
        let max_retry_delay = Duration::from_secs(60);

        loop {
            let (client, eventloop) = mqtt_listener::setup_mqtt_client(
                "gateway_prod",
                &self.settings.mqtt.host,
                self.settings.mqtt.port
            ).await;
            tracing::info!("Connecting to MQTT broker...");

            tokio::select! {
                result = mqtt_listener::run_event_loop(eventloop, self.handler.clone()) => {
                    if let Err(e) = result {
                        tracing::error!("Event loop stopped due to error: {}", e);
                        sleep(retry_delay).await;
                        retry_delay = std::cmp::min(retry_delay * 2, max_retry_delay);
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Shutdown signal received, exiting...");
                    let _ = client.disconnect().await;
                    return Ok(());
                }
            }
        }
    }
}