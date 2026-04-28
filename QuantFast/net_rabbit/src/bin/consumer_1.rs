use lapin::options::{BasicConsumeOptions, QueueDeclareOptions};
//use QuantFast::{get_connection, create_consumer};
use lapin::{protocol::queue, types::FieldTable, options::BasicAckOptions};
use uuid::Uuid;
use net_rabbit::{FinanceRequest, create_consumer};
use net_rabbit::get_connection;
use serde::{Deserialize, Serialize};
use quant_math::{call_europe_f, put_europe_f};
use futures_util::stream::StreamExt;



//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    let queue_name = "figas_test";
    let exchange = "figas_clay";
    let routing_key = "figas_test";

    channel.queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default()).await?;

    let mut consumer = channel
        .basic_consume(queue_name, "test_consumer", BasicConsumeOptions::default(),FieldTable::default())
        .await?;

    println!("[*] Consumer active. Processing math...");

    while let Some(delivery) = consumer.next().await {

        let delivery = delivery?;
        let data = String::from_utf8_lossy(&delivery.data).to_string();

        tokio::spawn(async move {

            if let Ok(req) = serde_json::from_str::<FinanceRequest>(&data) { //last one
                let result = match req.method.as_str() {
                    "call_europe" => call_europe_f(req.s, req.k, req.r, req.sig, req.t_expiry, req.t_start),
                    "put_europe"  => put_europe_f(req.s, req.k, req.r, req.sig, req.t_expiry, req.t_start),
                    _=> 0.0,
                
                };
                println!("Result for {}: {} = {}", req.to, req.method, result);
            } else {
                eprintln!("Failed to parse message: {}", data)
            }
        });

        delivery.ack(BasicAckOptions::default()).await?;
    }
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