use crate::domain::{SensorData, SensorType};

pub fn parse_sensor_data(topic_path: &str, payload: &str) -> Option<SensorData> {
    let sensor_str = topic_path.split('/').last().unwrap_or("");

    let sensor_type = match sensor_str {
        "temperature" => SensorType::Temperature,
        "humidity" => SensorType::Humidity,
        "pressure" => SensorType::Pressure,
        _ => SensorType::Unknown,
    };

    let value = payload.parse::<f64>().ok()?;

    Some(SensorData {
        sensor_type,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_humidity_data() {
        let raw_topic = "garden/sensors/humidity";
        let raw_payload = "65.5";

        let result = parse_sensor_data(raw_topic, raw_payload);

        let expected = Some(SensorData {
            sensor_type: SensorType::Humidity,
            value: 65.5,
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_invalid_payload() {
        let raw_topic = "garden/sensors/temperature";
        let raw_payload = "sensor_error";

        let result = parse_sensor_data(raw_topic, raw_payload);

        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_unknown_sensor_type() {
        let raw_topic = "garden/sensors/light";
        let raw_payload = "150.0";

        let result = parse_sensor_data(raw_topic, raw_payload);

        let expected = Some(SensorData {
            sensor_type: SensorType::Unknown,
            value: 150.0,
        });

        assert_eq!(result, expected);
    }
}