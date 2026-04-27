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