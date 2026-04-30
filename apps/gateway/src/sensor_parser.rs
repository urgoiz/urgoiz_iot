use crate::domain::{SensorData, SensorError, SensorType as DomainSensorType, SensorId};
use prost::Message;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/iot.rs"));
}

impl From<proto::SensorType> for DomainSensorType {
    fn from(proto_type: proto::SensorType) -> Self {
        match proto_type {
            proto::SensorType::Temperature => DomainSensorType::Temperature,
            proto::SensorType::Humidity => DomainSensorType::Humidity,
            proto::SensorType::Pressure => DomainSensorType::Pressure,
            _ => DomainSensorType::Unknown,
        }
    }
}

pub fn parse_sensor_protobuf(_payload: &[u8]) -> Result<SensorData, SensorError> {
    let proto_reading = proto::SensorReading::decode(_payload)?;

    let sensor_type : DomainSensorType = proto::SensorType::try_from(proto_reading.r#type)
        .map(DomainSensorType::from)
        .unwrap_or(DomainSensorType::Unknown);

    if sensor_type == DomainSensorType::Unknown {
        return Err(SensorError::InvalidPayload("Invalid sensor type value in protobuf".to_string()));
    }

    Ok(SensorData {
        sensor_id: SensorId::new(proto_reading.id.clone()),
        sensor_type,
        value: proto_reading.value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn test_parse_valid_protobuf() {
        let sensor_id = "sensor_01".to_string();
        let msg = proto::SensorReading {
            id: sensor_id.to_string(),
            r#type: proto::SensorType::Temperature as i32,
            value: 22.5,
        };

        let mut payload = Vec::new();
        msg.encode(&mut payload).unwrap();

        let result = parse_sensor_protobuf(&payload);

        let expected = SensorData {
            sensor_id: SensorId::new(sensor_id),
            sensor_type: DomainSensorType::Temperature,
            value: 22.5,
        };

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_invalid_protobuf() {
        let payload = vec![0xFF, 0x00, 0xBA, 0xDC];

        let result = parse_sensor_protobuf(&payload);

        assert!(matches!(result, Err(SensorError::InvalidPayload(_))));
    }
}