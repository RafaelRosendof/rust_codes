use core::time;
use lapin::BasicProperties;
use lapin::options::{BasicAckOptions, ExchangeDeclareOptions};
use quant_math::{FinanceMethod, FinanceRequest, FinancerequestTo, finance_factory};
use futures_util::stream::StreamExt;
use lapin::{options::BasicPublishOptions};
use lapin::{options::{BasicConsumeOptions, QueueDeclareOptions, QueueBindOptions}, protocol::{channel, queue}, types::FieldTable};
use net_rabbit::{build_producer, get_connection};
use serde_json::json;


/*
Create local server here, this server is gonna recieve the payload ie: Local consumer process and them

publish this message back to the request.to back to the cli_client.

*/




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    let exchange_name = "figas_clay";
    let queue_name = "figas_test";
    let routing_key = "figas_test";


    channel.exchange_declare(
        exchange_name,
        lapin::ExchangeKind::Direct,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    ).await?;



    channel.queue_declare(
        queue_name,
        QueueDeclareOptions::default(),
        FieldTable::default()
    ).await?;

    channel.queue_bind(
        queue_name,
        exchange_name,
        routing_key,
        QueueBindOptions::default(),
        FieldTable::default()
    ).await?;

    let mut consumer = channel
        .basic_consume(
            queue_name,
            "backend_worker_1",
            BasicConsumeOptions::default(),
            FieldTable::default()
        ).await?;

    println!("[*] Backend Consumer active. Waiting for math tasks...");

    let publish_channel = channel.clone();
    
    while let Some(delivery_result) = consumer.next().await{
        let delivery = delivery_result?;
        let data = String::from_utf8_lossy(&delivery.data).to_string();
        let pub_ch = publish_channel.clone();

        tokio::spawn(async move {
            println!("Received data: {}", data);
            
            if let Ok(req) = serde_json::from_str::<FinancerequestTo>(&data) { 
                let result = finance_factory(req.method, &req.params);
                println!("Result calculated: {} = {}", req.method, result);

                let response_payload = format!("{{\"result\": {}}}", result);

                
                let _ = pub_ch.basic_publish(
                    "", 
                    &req.to, 
                    BasicPublishOptions::default(),
                    response_payload.as_bytes(),
                    BasicProperties::default(),
                ).await.unwrap().await.unwrap();

            } else {
                eprintln!("Failed to parse message: {}", data);
            }

            // 4. Confirma o processamento apenas DEPOIS de enviar a resposta
            let _ = delivery.ack(BasicAckOptions::default()).await;
        });
    }


    Ok(())

}

/*

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    build_producer(&channel, "figas_clay", "figas_test", "First message in the queue").await?;

    for i in 0..100 {
        let message = json!({
            "to": format!("client_{}", i),
            "method": if i % 2 == 0 { FinanceMethod::CallEurope } else { FinanceMethod::PutEurope },
            "params": {
                "s": 100.0,
                "k": 110.0,
                "r": 0.05,
                "sig": 0.2,
                "t_expiry": 1.0,
                "t_start": 0.0
            }
        }).to_string();
        
        net_rabbit::publish_message(&channel, "figas_clay", "figas_test", &message).await?;
        
        println!("Sent request {}", i);
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
    
    Ok(())
}

*/