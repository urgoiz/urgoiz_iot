mod domain;
mod sensor_parser;
mod mqtt_handler;

use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;

use crate::mqtt_handler::handle_mqtt_message;
use crate::sensor_parser::parse_sensor_data;

#[tokio::main]
async fn main() {
    println!("IoT Gateway is starting...");

    let mut mqttoptions = MqttOptions::new("gateway_client_1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client.subscribe("garden/sensors/#", QoS::AtMostOnce).await.unwrap();
    println!("Subscribed to 'garden/sensors/#'. Waiting for data...");

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(publish_packet))) => {
                let topic = publish_packet.topic;

                if let Ok(payload_str) = String::from_utf8(publish_packet.payload.to_vec()) {
                    match handle_mqtt_message(&topic, &payload_str, parse_sensor_data) {
                        Ok(sensor_data) => {
                            println!("Success | Sensor: {:?} | Value: {}", sensor_data, sensor_data.value);
                        },
                        Err(error_msg) => eprintln!("Error handling MQTT message: {}", error_msg),
                    }
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
