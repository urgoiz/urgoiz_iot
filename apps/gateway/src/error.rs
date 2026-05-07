use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GatewayError {
    #[error("Invalid configuration: {0}")]
    _ConfigError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("MQTT network error: {0}")]
    _NetworkError(String),
    #[error("Protobuf decoding error: {0}")]
    DecodeError(#[from] prost::DecodeError),
    #[error("Invalid or corrupt sensor data: {0}")]
    InvalidData(String),
}

impl From<sqlx::Error> for GatewayError {
    fn from(err: sqlx::Error) -> Self {
        GatewayError::DatabaseError(err.to_string())
    }
}

pub type GatewayResult<T> = Result<T, GatewayError>;