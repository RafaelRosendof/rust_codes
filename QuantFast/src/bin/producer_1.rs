use core::time;


use QuantFast::{build_producer, get_connection, publish_message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    let message = r#"{"to": "client_1", "request": "monte carlo"}"#;
    
    build_producer(&channel, "figas_clay", "figas_test", message).await?;
    tokio::time::sleep(time::Duration::from_secs(5)).await;
    for i in 0..10_000 {
        //let message = format!(r#"{{"to": "client_1", "request": "monte carlo {}", "id": {}}}"#, i, i);
        let message =format!(r#" {{"to": "to": "client_{}", "request": "  {}  ", "id": {}}}"#, i, i, i);
        
        publish_message(&channel, "figas_clay", "figas_test", &message).await?;

        tokio::time::sleep(time::Duration::from_millis(100)).await;
    }

    Ok(())
}