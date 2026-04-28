use crate::domain::{SensorData, SensorError, SensorType};
use serde::Deserialize;

#[derive(Deserialize)]
struct SensorPayload {
    value: f64,
}

pub fn parse_sensor_data(topic_path: &str, payload: &str) -> Result<SensorData, SensorError> {
    let sensor_type = extract_sesor_type(topic_path)?;
    let value = extract_value(payload)?;

    Ok(SensorData {
        sensor_type,
        value,
    })
}

fn extract_sesor_type(topic_path: &str) -> Result<SensorType, SensorError> {
    match topic_path.split('/').last() {
        Some("temperature") => Ok(SensorType::Temperature),
        Some("humidity") => Ok(SensorType::Humidity),
        Some("pressure") => Ok(SensorType::Pressure),
        Some(_) => Ok(SensorType::Unknown),
        None => Err(SensorError::InvalidTopic("Invalid topic path".to_string())),
    }
}

fn extract_value(payload: &str) -> Result<f64, SensorError> {
    if let Ok(sensor_payload) = serde_json::from_str::<SensorPayload>(payload) {
        Ok(sensor_payload.value)
    } else if let Ok(float_val) = payload.parse::<f64>() {
        Ok(float_val)
    } else {
        Err(SensorError::InvalidPayload(format!("Cannot parse: '{}'", payload)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_humidity_data() {
        let raw_topic = "garden/sensors/humidity";
        let raw_payload = "65.5";

        let result = parse_sensor_data(raw_topic, raw_payload);

        let expected = SensorData {
            sensor_type: SensorType::Humidity,
            value: 65.5,
        };

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_invalid_payload() {
        let raw_topic = "garden/sensors/temperature";
        let raw_payload = "sensor_error";

        let result = parse_sensor_data(raw_topic, raw_payload);

        assert_eq!(result, Err(SensorError::InvalidPayload("Cannot parse: 'sensor_error'".to_string())));
    }

    #[test]
    fn test_parse_unknown_sensor_type() {
        let raw_topic = "garden/sensors/light";
        let raw_payload = "150.0";

        let result = parse_sensor_data(raw_topic, raw_payload);

        let expected = SensorData {
            sensor_type: SensorType::Unknown,
            value: 150.0,
        };

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_json_payload() {
        let raw_topic = "garden/sensors/temperature";
        let raw_payload = r#"{"value": 22.5}"#;

        let result = parse_sensor_data(raw_topic, raw_payload);

        let expected = SensorData {
            sensor_type: SensorType::Temperature,
            value: 22.5,
        };

        assert_eq!(result, Ok(expected));
    }
}