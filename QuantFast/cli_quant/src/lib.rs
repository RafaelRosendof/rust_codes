use quant_math::{FinancerequestTo, FinanceMethod};
use serde_json::json;
use std::{error::Error};
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
    queue_name: &str,
    exchange: &str,
    routing_key: &str,

) -> Result<(), Box<dyn std::error::Error>>{



    Ok(())
}