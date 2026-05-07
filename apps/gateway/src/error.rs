use thiserror::Error;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Invalid configuration: {0}")]
    Config(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("MQTT network error: {0}")]
    Network(#[from] rumqttc::ClientError),
    #[error("Protobuf decoding error: {0}")]
    Decod(#[from] prost::DecodeError),
    #[error("Invalid sensor data: {0}")]
    InvalidData(String),
}