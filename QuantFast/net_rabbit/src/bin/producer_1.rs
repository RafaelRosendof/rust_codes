use core::time;

use lapin::protocol::channel;
use net_rabbit::{build_producer, get_connection, publish_message};
use serde_json::json;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    build_producer(&channel, "figas_clay", "figas_test", "First message in the queue");

    for i in 0..100 {
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
        
        net_rabbit::publish_message(&channel, "figas_clay", "figas_test", &message).await?;
        
        println!("Sent request {}", i);
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
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