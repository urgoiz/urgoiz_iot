use crate::sensor_parser::{parse_sensor_data, SensorData, SensorType};

fn handle_mqtt_message(topic: &str, payload: &str) -> Result<SensorData, String> {
    match parse_sensor_data(topic, payload) {
        Some(data) => Ok(data),
        None => Err(format!("Failed to parse sensor data from topic: {}", topic)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_valid_temperature_message() {
        let topic = "garden/sensors/temperature";
        let payload = "22.5";

        let result = handle_mqtt_message(topic, payload);

        let expected = Ok(SensorData {
            sensor_type: SensorType::Temperature,
            value: 22.5,
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_handle_invalid_message() {
        let topic = "garden/sensors/humidity";
        let payload = "error_reading";

        let result = handle_mqtt_message(topic, payload);

        let expected_error = format!("Failed to parse sensor data from topic: {}", topic);

        assert_eq!(result, Err(expected_error));
    }
}