use async_trait::async_trait;
use strum::Display;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Clone, Copy, Display, Hash, Eq)]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SensorData {
    pub sensor_id: SensorId,
    pub sensor_type: SensorType,
    pub value: f64,
}

#[async_trait]
pub trait SensorRepository: Send + Sync {
    async fn save_reading(&self, data: SensorData) -> Result<(), crate::error::GatewayError>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct SensorId(String);

impl SensorId {
    pub fn new(id: impl Into<String>) -> Self {
        SensorId(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}