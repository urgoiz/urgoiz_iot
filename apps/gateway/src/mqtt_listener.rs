use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, QoS};
use std::time::Duration;

use crate::domain::SensorRepository;
use crate::mqtt_handler::MqttHandler;
use crate::sensor_parser::parse_sensor_protobuf;

pub async fn setup_mqtt_client(client_id: &str, host: &str, port: u16) -> (AsyncClient, EventLoop) {
    let mut mqttoptions = MqttOptions::new(client_id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

    client.subscribe("garden/sensors/#", QoS::AtMostOnce).await.unwrap();
    println!("Subscribed to 'garden/sensors/#'. Waiting for data...");

    (client, eventloop)
}

pub async fn run_event_loop<R: SensorRepository>(
    mut event: EventLoop,
    handler: MqttHandler<R>,) {
    loop {
        match event.poll().await {
            Ok(Event::Incoming(Packet::Publish(packet))) => {
                if let Err(e) = handler.handle_message(&packet.payload, parse_sensor_protobuf).await {
                    eprintln!("ERROR | {}", e);
                } else {
                    println!("SUCCESS | Message processed and stored.");
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use async_trait::async_trait;
    use crate::domain::{SensorData, SensorError, SensorType};
    use crate::sensor_parser::proto;
    use prost::Message;

    struct MockRepo{
        data: Arc<Mutex<Vec<SensorData>>>,
    }

    #[async_trait]
    impl SensorRepository for MockRepo {
        async fn save_reading(&self, data: SensorData) -> Result<(), SensorError> {
            self.data.lock().unwrap().push(data);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_event_loop_processing_flow() {

        let save_data = Arc::new(Mutex::new(vec![]));
        let repo = MockRepo { data: save_data.clone() };
        let handler = MqttHandler::new(repo);

        let msg = proto::SensorReading {
            id: "sensor_01".to_string(),
            r#type: proto::SensorType::Temperature as i32,
            value: 25.5,
        };
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();

        let result = handler.handle_message(&buf, parse_sensor_protobuf).await;

        assert!(result.is_ok());
        let _final_data = save_data.lock().unwrap();

        assert_eq!(_final_data.len(), 1);
        assert_eq!(_final_data[0].sensor_type, SensorType::Temperature);
        assert_eq!(_final_data[0].value, 25.5);
    }

    #[tokio::test]
    async fn test_hander_integration_in_listener_scope() {

        let save_data = Arc::new(Mutex::new(vec![]));
        let repo = MockRepo { data: save_data.clone() };
        let handler = MqttHandler::new(repo);

        let payload = vec![0, 8, 204, 204, 204, 61]; // Invalid protobuf payload

        let result = handler.handle_message(&payload, parse_sensor_protobuf).await;

        assert!(result.is_err());
        let _final_data = save_data.lock().unwrap();
        assert_eq!(_final_data.len(), 0); // No data should be saved due
    }
}