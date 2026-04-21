//use QuantFast::{get_connection, create_consumer};
use lapin::{protocol::queue, types::FieldTable};
use uuid::Uuid;
use net_rabbit::create_consumer;
use net_rabbit::get_connection;
use serde::{Deserialize, Serialize};
use quant_math::{call_europe_f, put_europe_f};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    let arguments = FieldTable::default();
    let exchange = "figas_clay";
    let routing_key = "figas_test"; 
    let queue_name = "figas_clay";
    //let queue_name = format!("queue_name_ {}", Uuid::new_v4());

    create_consumer(&channel, &queue_name, exchange, routing_key, arguments).await?;

    Ok(())
}


/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;
    
    let queue_name = "figas_clay";
    // Declare queue and bind to exchange logic here...

    let mut consumer = channel
        .basic_consume(queue_name, "test_consumer", BasicConsumeOptions::default(), FieldTable::default())
        .await?;

    println!("[*] Consumer active. Processing math...");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        let data = String::from_utf8_lossy(&delivery.data).to_string();

        // SPAWN a new task for each message (Non-blocking)
        tokio::spawn(async move {
            if let Ok(req) = serde_json::from_str::<FinanceRequest>(&data) {
                let result = match req.method.as_str() {
                    "call_europe" => call_europe_f(req.s, req.k, req.r, req.sig, req.t_expiry, req.t_start),
                    "put_europe"  => put_europe_f(req.s, req.k, req.r, req.sig, req.t_expiry, req.t_start),
                    _ => 0.0,
                };
                println!("Result for {}: {} = {}", req.to, req.method, result);
            } else {
                eprintln!("Failed to parse message: {}", data);
            }
        });

        delivery.ack(BasicAckOptions::default()).await?;
    }

    Ok(())
}
*/