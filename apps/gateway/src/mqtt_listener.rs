use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, Publish, QoS};
use std::time::Duration;

use crate::domain::SensorData;
use crate::mqtt_handler::handle_mqtt_message;
use crate::sensor_parser::parse_sensor_protobuf;

pub async fn setup_mqtt_client(client_id: &str, host: &str, port: u16) -> (AsyncClient, EventLoop) {
    let mut mqttoptions = MqttOptions::new(client_id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

    client.subscribe("garden/sensors/#", QoS::AtMostOnce).await.unwrap();
    println!("Subscribed to 'garden/sensors/#'. Waiting for data...");

    (client, eventloop)
}

pub async fn run_event_loop(mut event: EventLoop) {
    loop {
        match event.poll().await {
            Ok(Event::Incoming(Packet::Publish(packet))) => {
                match process_packet(packet) {
                    Ok(data) => println!("SUCCESS | Sensor: {:?} | Value: {}", data.sensor_type, data.value),
                    Err(e) => eprintln!("ERROR | {}", e),
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("MQTT connection error: {:?}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

pub fn process_packet(packet: Publish) -> Result<SensorData, String> {
    println!("Received MQTT message on topic: {}", packet.topic);

    handle_mqtt_message(&packet.payload, parse_sensor_protobuf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumqttc::QoS;
    use bytes::Bytes;
    use crate::domain::SensorType;
    use crate::sensor_parser::proto;
    use prost::Message;

    #[test]
    fn test_process_valid_packet() {
        let msg = proto::SensorReading {
            r#type: proto::SensorType::Temperature as i32,
            value: 25.5,
        };
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();

        let packet = Publish {
            dup: false,
            retain: false,
            qos: QoS::AtMostOnce,
            topic: "irrelevant".to_string(),
            pkid: 0,
            payload: Bytes::from(buf),
        };

        let result = process_packet(packet);

        assert!(result.is_ok());
        let expected = Ok(SensorData {
            sensor_type: SensorType::Temperature,
            value: 25.5,
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_packet_with_invalid_utf8_payload() {

        let packet = Publish {
            dup: false,
            retain: false,
            qos: QoS::AtMostOnce,
            topic: "irrelevant".to_string(),
            pkid: 0,
            payload: Bytes::from(vec![0xFF, 0xFE, 0xFD]), // Invalid UTF-8 sequence
        };

        let result = process_packet(packet);

        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Protobuf decode error"));
    }
}