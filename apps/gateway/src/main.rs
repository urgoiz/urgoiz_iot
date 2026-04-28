mod domain;
mod sensor_parser;
mod mqtt_handler;
mod mqtt_listener;

#[tokio::main]
async fn main() {
    println!("IoT Gateway is starting...");

    let (_client, eventloop) = mqtt_listener::setup_mqtt_client("gateway_client_1", "localhost", 1883).await;
    mqtt_listener::run_event_loop(eventloop).await;
}
