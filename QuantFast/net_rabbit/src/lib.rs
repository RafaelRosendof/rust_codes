use lapin::protocol::channel;
use lapin::{
    BasicProperties, Connection, ConnectionProperties, options::*, types::FieldTable
};
use quant_math::FinanceMethod;
use yfinance_rs::Ticker;
use yfinance_rs::YfClient;
use yfinance_rs::YfClientBuilder;
use yfinance_rs::Candle;
use serde::{Serialize, Deserialize};
use futures_util::stream::StreamExt;
use std::{error::Error};
//use yahoo_finance::{Bar, Interval, history, DateTime};

pub struct OptionInputs {
    pub s: f64,    // Spot price
    pub k: f64,    // Strike price
    pub r: f64,    // Risk-free rate
    pub sig: f64,  // Volatility
    pub t: f64,    // Time to maturity
}

pub struct GreeksOptions{
    //
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinanceData{
    pub s: f64,
    pub k: f64,
    pub r: f64,
    pub sig: f64,
    pub t_expiry: f64,
    pub t_start: f64,
}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct FinanceRequest {
//    pub to: String,
//    pub method: FinanceMethod,
//    pub s: f64,
//    pub k: f64,
//    pub r: f64,
//    pub sig: f64,
//    pub t_expiry: f64,
//    pub t_start: f64,
//}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinanceRequestRabbit {
    pub to: String,
    pub method: FinanceMethod,
    pub data: FinanceData,
}

// ---------------- Scraoping data from yahoo and google finance ----------------------

//scraping of data from yahoo and google finance

//pub async fn collect_retrieve(symbol: &str) -> Result<Vec<Bar>>{
//
//    match history::retrieve(symbol).await{
//
//        Err(e) => println!("Failed to call yahoo finance api for {:?}", e),
//        Ok(data) => 
//            for bar in &data{
//                println!("{:?}", bar);
//            }
//    }
//}
//
//pub async fn collect_retrieve_interval(symbol: &str, interval: Interval) -> Result<Vec<Bar>>{
//
//    match history::retrieve_interval(symbol, interval).await{
//
//        Err(e) => println!("Failed to call yahoo finance api for {:?}", e),
//        Ok(data) => 
//            for bar in &data{
//                println!("{:?}", bar);
//            }
//    }
//
//}
//
//pub async fn collect_data_range(symbol: &str, start: DateTime<Utc>, end: Option<DateTime<Utc>>) -> Result<Vec<Bar>>{
//
//    match history::retrieve_range(symbol, start, end).await{
//        Err(e) => println!("Failed to call yahoo finance api for {:?}", e),
//        Ok(data) => 
//            for bar in &data{
//                println!("{:?}", bar);
//            }
//    }
//
//}

pub async fn collect_raw_data(stock_symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = YfClient::default();
    let ticker = Ticker::new(&client, stock_symbol);

    let quote = ticker.quote().await?;

    println!("Quote for {}: {:?}", stock_symbol, quote);
    println!("Quote for {}: {:?}", stock_symbol, quote
    .price.as_ref().map(|p| yfinance_rs::core::conversions::money_to_f64(p)).unwrap_or(0.0));
    
    let hist = ticker.history(Some(yfinance_rs::Range::D5),
    Some(yfinance_rs::Interval::D1), false ).await?;

    if let Some(last_bar) = hist.last(){
        println!("Last closing price: ${:.2} on timestamp {}", yfinance_rs::core::conversions::money_to_f64(&last_bar.close), last_bar.ts);
    }

    let recs = ticker.recommendations().await?;
    if let Some(latest_rec) = recs.first() {
        println!("Latest recommendation period: {}", latest_rec.period);
    }

    Ok(())
}


// --------------- Web Server and RabbitMQ Methods ----------------------

// build a reactive server that listens to the rabbitMQ and responds to the rabbit to the client 
// client (request) -> rabbit -> server (response) -> rabbit -> client (response)

pub async fn get_connection(host: &str, port: u16) -> Result<Connection, lapin::Error> {
    let addr = format!("amqp://guest:guest@{}:{}/%2f", host, port);
    Connection::connect(&addr, ConnectionProperties::default()).await
}

pub async fn build_producer(
    channel: &lapin::Channel, 
    exchange: &str, 
    routing_key: &str, 
    message: &str
) -> Result<(), Box<dyn Error>> { // what is the man of Box<dyn Error> here ?
    let payload = message.as_bytes();

    channel.exchange_declare(
        exchange,
        lapin::ExchangeKind::Direct,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    ).await?;

    channel.basic_publish(
        exchange,
        routing_key,
        BasicPublishOptions::default(),
        payload,
        BasicProperties::default(),
    ).await?.await?; 

    println!("Message published to exchange: {}", exchange);
    Ok(())

}

pub async fn publish_message(
    channel: &lapin::Channel, 
    exchange: &str, 
    routing_key: &str, 
    message: &str
) -> Result<(), Box<dyn Error>> {
    let payload = message.as_bytes();
    
    channel.basic_publish(
        exchange,
        routing_key,
        BasicPublishOptions::default(),
        payload,
        BasicProperties::default(),
    )
    .await?
    .await?;

    println!("Message published to exchange: {}", exchange);
    Ok(())
}

pub async fn create_consumer(
    channel: &lapin::Channel,
    queue_name: &str,
    exchange: &str,
    routing_key: &str,
    //options: QueueBindOptions,
    arguments: FieldTable
) -> Result<(), Box<dyn Error>> {
    // 1. Declare the queue just in case
    channel.queue_declare(
        queue_name, 
        QueueDeclareOptions::default(), 
        FieldTable::default()
    ).await?;

    channel.queue_bind(queue_name,
         exchange,
         routing_key,
         QueueBindOptions::default(),
         arguments
        ).await?;

    // 2. Start the consumer
    let mut consumer = channel.basic_consume(
        queue_name,
        "my_consumer",
        BasicConsumeOptions::default(),
        FieldTable::default(),
    ).await?;

    println!("[*] Waiting for messages on {}. To exit press CTRL+C", queue_name);

    // 3. Instead of spawning, we loop directly in the async function
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        let data = String::from_utf8_lossy(&delivery.data);
        
        println!("--------------------------");
        println!("Received: {}", data);
        println!("Correlation ID: {:?}", delivery.properties.correlation_id());
        println!("Reply To: {:?}", delivery.properties.reply_to());
        
        // Acknowledge the message
        delivery.ack(BasicAckOptions::default()).await?;
    }

    Ok(())
}
