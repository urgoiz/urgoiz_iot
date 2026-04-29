use prost::DecodeError;
use async_trait::async_trait;

#[derive(Debug, PartialEq, Clone)]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SensorData {
    pub sensor_type: SensorType,
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub enum SensorError {
    InvalidPayload(String),
}

impl From<DecodeError> for SensorError {
    fn from(err: DecodeError) -> Self {
        SensorError::InvalidPayload(format!("Protobuf decode error: {}", err))
    }
}

#[async_trait]
pub trait SensorRepository: Send + Sync {
    async fn save_reading(&self, data: SensorData) -> Result<(), SensorError>;
}