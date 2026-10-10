use std::thread;
use std::time::Duration;

use paho_mqtt as mqtt;

fn main() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://broker.hivemq.com")
        .client_id("echo_client")
        .finalize();

    let mut client = mqtt::Client::new(create_opts).unwrap();

    let conn_opts = mqtt::ConnectOptionsBuilder::new().finalize();

    if let Err(e) = client.connect(conn_opts) {
        panic!("Failed to connect: {:?}", e);
    }

    let rx = client.start_consuming();

    client.subscribe("1", mqtt::QOS_1).unwrap();

    thread::spawn(move || loop {
        let _ = client.publish(mqtt::Message::new("1", "echo", mqtt::QOS_1));
        thread::sleep(Duration::from_secs(1));
    });

    for msg in rx.iter() {
        if let Some(msg) = msg {
            let payload = msg.payload_str().trim().to_string();
            if !payload.is_empty() {
                println!("{}", payload);
            }
        }
    }
}
