use crate::domain::{SensorData, SensorError};

pub fn handle_mqtt_message(
    payload: &[u8],
    parser_fn: fn(&[u8]) -> Result<SensorData, SensorError>,
) -> Result<SensorData, String> {
    
    match parser_fn(payload) {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to parse sensor data | Reason: {:?}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SensorType;

    fn mock_success_parser(_payload: &[u8]) -> Result<SensorData, SensorError> {
        Ok(SensorData {
            sensor_type: SensorType::Temperature,
            value: 22.5,
        })
    }

    fn mock_fail_parser(_payload: &[u8]) -> Result<SensorData, SensorError> {
        Err(SensorError::InvalidPayload("Cannot parse".to_string()))
    }

    #[test]
    fn test_handle_valid_message_with_injected_parser() {

        let result = handle_mqtt_message(b"any_payload", mock_success_parser);

        let expected = Ok(SensorData {
            sensor_type: SensorType::Temperature,
            value: 22.5,
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_handle_invalid_message_with_injected_parser() {
        let result = handle_mqtt_message(b"error_reading", mock_fail_parser);
        let expected_error = format!("Failed to parse sensor data | Reason: {:?}", SensorError::InvalidPayload("Cannot parse".to_string()));

        assert_eq!(result, Err(expected_error));
    }
}