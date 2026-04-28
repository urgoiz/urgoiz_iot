use crate::domain::{SensorData, SensorError};

pub fn handle_mqtt_message(
    topic: &str,
    payload: &str,
    parser_fn: fn(&str, &str) -> Result<SensorData, SensorError>,
) -> Result<SensorData, String> {
    
    match parser_fn(topic, payload) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to parse sensor data from topic: {} | Reason: {:?}", topic, e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SensorType;

    fn mock_success_parser(_topic: &str, _payload: &str) -> Result<SensorData, SensorError> {
        Ok(SensorData {
            sensor_type: SensorType::Temperature,
            value: 22.5,
        })
    }

    fn mock_fail_parser(_topic: &str, _payload: &str) -> Result<SensorData, SensorError> {
        Err(SensorError::InvalidPayload("Cannot parse".to_string()))
    }

    #[test]
    fn test_handle_valid_message_with_injected_parser() {

        let result = handle_mqtt_message("any/topic", "any_payload", mock_success_parser);

        let expected = Ok(SensorData {
            sensor_type: SensorType::Temperature,
            value: 22.5,
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_handle_invalid_message_with_injected_parser() {
        let topic = "garden/sensors/humidity";
        let payload = "error_reading";

        let result = handle_mqtt_message(topic, payload, mock_fail_parser);

        let expected_error = format!("Failed to parse sensor data from topic: {} | Reason: {:?}", topic, SensorError::InvalidPayload("Cannot parse".to_string()));

        assert_eq!(result, Err(expected_error));
    }
}