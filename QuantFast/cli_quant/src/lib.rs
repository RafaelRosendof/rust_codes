use lapin::{options::BasicConsumeOptions, protocol::channel};
use quant_math::{FinancerequestTo, FinanceMethod};
use serde_json::json;
use std::{error::Error, sync::mpsc::channel};
use futures_util::stream::{Concat, StreamExt};
use lapin::{
    BasicProperties, Connection, ConnectionProperties, options::*, types::FieldTable
};

use net_rabbit::{collect_raw_data, build_producer, publish_message, get_connection};

pub async fn request_stock(name: &str) -> Result<f64, Box<dyn Error>>{

    let stock = collect_raw_data(name).await;

    println!("Stock: {:?} \n", name);
    //println!("[  DATA  ] {:?}", stock);
    Ok(stock?)
}

pub async fn send_request(data_request: FinancerequestTo, exchange: &str, routing_key: &str) -> Result<(), Box<dyn std::error::Error>>{

    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    publish_message(&channel, &exchange, &routing_key, &data_request).await?;


    Ok(())
}

pub async fn create_local_consumer(
    queue_name: &str

) -> Result<(), Box<dyn std::error::Error>>{
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;
    
    let mut queue_opts = QueueDeclareOptions::default();
    queue_opts.exclusive = true;
    queue_opts.auto_delete = true;
    
    channel.queue_declare(
        queue_name,
        queue_opts,
        FieldTable::default()
    ).await?;

    let mut consumer = channel.basic_consume(
        queue_name,
        "cli_rpc_consumer",
        BasicConsumeOptions::default(),
        FieldTable::default()
    ).await?;

    tokio::spawn( async move{
        while let Some(delivery_result) = consumer.next().await{
            match delivery_result{
                Ok(delivery) => {
                    let delivery = delivery;
                    let data = String::from_utf8_lossy(&delivery.data);
                    let corr_id = delivery.properties.correlation_id().clone();
                    println!("\n[!] Resultado Recebido (CorrID: {:?})", corr_id);
                    println!("[!] Dados: {}\n", data);

                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => println!("Erro no consumidor de background: {:?}", e),
            }
        }
    });

    
    Ok(())
}

/*
tokio::spawn( async move{
        while let Some(delivery_result) = consumer.next().await{
            match delivery_result{
                Ok(delivery) => {
                    let delivery = delivery;
                    let data = String::from_utf8_lossy(&delivery.data);
                    let corr_id = delivery.properties.correlation_id().clone();
                    println!("\n[!] Resultado Recebido (CorrID: {:?})", corr_id);
                    println!("[!] Dados: {}\n", data);

                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => println!("Erro no consumidor de background: {:?}", e),
            }
        }
    });

*/