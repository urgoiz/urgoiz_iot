use prost::DecodeError;

#[derive(Debug, PartialEq)]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    Unknown,
}

#[derive(Debug, PartialEq)]
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