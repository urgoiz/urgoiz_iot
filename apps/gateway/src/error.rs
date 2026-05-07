use thiserror::Error;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("MQTT connection error: {0}")]
    MqttConnection(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),
}