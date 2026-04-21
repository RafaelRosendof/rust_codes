use core::time;

use lapin::protocol::channel;
use net_rabbit::{build_producer, get_connection, publish_message};


async fn run_producer(message: str, exchange: str, routing_key: str, host: str, port: u16) -> Result<(), Box<dyn std::error::Error>> {

    let conn = get_connection(&host, port);
    let channel = conn.create_channel().await?;

    let first_message = r#"{"to": "client_1", "request": "monte carlo"}"#;

    // if producer exists do not create a new one
    //build_producer(&channel, &exchange, &routing_key, first_message).await?;

    publish_message(&channel, &exchange, &routing_key, &message)
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    let message = r#"{"to": "client_1", "request": "monte carlo"}"#;
    
    build_producer(&channel, "figas_clay", "figas_test", message).await?;

    tokio::time::sleep(time::Duration::from_secs(5)).await;
    for i in 0..10_000 {
        let message =format!(r#" {{"to": "to": "client_{}", "request": "  {}  ", "id": {}}}"#, i, i, i);
        
        publish_message(&channel, "figas_clay", "figas_test", &message).await?;

        tokio::time::sleep(time::Duration::from_millis(100)).await;
    }

    Ok(())
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    for i in 0..100 {
        // Build a valid JSON object
        let message = json!({
            "to": format!("client_{}", i),
            "method": if i % 2 == 0 { "call_europe" } else { "put_europe" },
            "s": 100.0 + (i as f64),
            "k": 110.0,
            "r": 0.05,
            "sig": 0.2,
            "t_expiry": 1.0,
            "t_start": 0.0
        }).to_string();

        // Using your publish logic
        net_rabbit::publish_message(&channel, "figas_clay", "figas_test", &message).await?;
        
        println!("Sent request {}", i);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    Ok(())
}
*/