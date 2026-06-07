use lapin::protocol::channel;
use lapin::{
    BasicProperties, Connection, ConnectionProperties, options::*, types::FieldTable
};
use quant_math::{FinanceMethod, FinancerequestTo};
use rdkafka::Message;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::FutureProducer;
use uuid::Uuid;
use yfinance_rs::Ticker;
use yfinance_rs::YfClient;
use yfinance_rs::YfClientBuilder;
use yfinance_rs::Candle;
use serde::{Serialize, Deserialize};
use futures_util::stream::{Concat, StreamExt};
use std::{error::Error};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureRecord};
use std::time::Duration;

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

pub async fn collect_raw_data(stock_symbol: &str) -> Result<f64, Box<dyn Error>> {
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
    let price = quote.price;

    if let Some(money) = &price{
        let spot_price: f64 = yfinance_rs::core::conversions::money_to_f64(money);
        println!("Spot price: ${:.2}", spot_price);

        return Ok(spot_price);
    }
    else{
        print!("No spot price: 0.00");
        return Ok(0.00);
    }
    
}

// TODO -> build more functions here

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

pub fn encode_message(message: &FinancerequestTo) -> Vec<u8>{
    serde_json::to_vec(message).expect("Failed to encode message")
}

pub fn decode_message(message: &[u8]) -> FinancerequestTo{
    serde_json::from_slice(message).expect("Failed to decode message")
}

pub async fn publish_rpc_message(
    channel: &lapin::Channel,
    exchange: &str,
    routing_key: &str,
    message: &FinancerequestTo,
    reply_queue: &str,
) -> Result<String, Box<dyn Error>>{


    let message_bytes = encode_message(message);
    let correlation_id = Uuid::new_v4().to_string();

    let props = BasicProperties::default()
    .with_reply_to(reply_queue.into())
    .with_correlation_id(correlation_id.clone().into());

    channel.basic_publish(
        exchange,
        routing_key,
        BasicPublishOptions::default(),
        &message_bytes,
        props,
    )
    .await?
    .await?;
    
    Ok(correlation_id)
}

pub async fn publish_message(
    channel: &lapin::Channel, 
    exchange: &str, 
    routing_key: &str,
    message: &FinancerequestTo
) -> Result<(), Box<dyn Error>> {

    let message  = encode_message(message);
    
    channel.basic_publish(
        exchange,
        routing_key,
        BasicPublishOptions::default(),
        &message,
        BasicProperties::default(),
    )
    .await?
    .await?;

    println!("Message published to exchange: {}", exchange);
    Ok(())
}

pub async fn publish_message_str(
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
    arguments: FieldTable
) -> Result<(), Box<dyn Error>> {
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

    let mut consumer = channel.basic_consume(
        queue_name,
        "my_consumer",
        BasicConsumeOptions::default(),
        FieldTable::default(),
    ).await?;

    println!("[*] Waiting for messages on {}. To exit press CTRL+C", queue_name);

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

// pub async fn send_fin_ticket_kafka(topic: &str, key: &str, payload: &str){
//     let producer: FutureProducer = ClientConfig::new()
//     .set("bootstrap.servers", "localhost:9092")
//     .set("message.timeout.ms", "5000")
//     .create()
//     .expect("Producer creation error");
// 
//     let record = FutureRecord::to(topic)
//     .key(key)
//     .payload(payload);
// 
//     match producer.send(record, Duration::from_secs(0)).await {
//         Ok(delivery) => println!(
//             "Sucesso! Partição: {}, Offset: {}", 
//             delivery.partition, delivery.offset
//         ),
//         Err( (e, _)) => eprint!("Error to send message {:?} ", e),
//     }
// 
// }

pub async fn create_topic(topic: &str){
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Fail to create Kafka AdmClient");

    let new_topic = NewTopic::new(topic, 1, TopicReplication::Fixed(1));
    let opts = AdminOptions::new();

    match admin_client.create_topics(&[new_topic], &opts).await{
        Ok(res) => {
            for r in res{
                match r{
                    Ok(topic_name) => println!("Topic '{}' created/verify ", topic_name),
                    Err((topic_name, err)) => {
                        eprintln!("Advise for topic {} {:?}", topic_name, err);
                    
                    }
                }
            }
        }
        Err(e) => eprintln!("Fatal error {:?} ", e)
    }
}

pub async fn send_ticket_kafka(producer: &FutureProducer, topic: &str, key: &str, payload: &str){
    let record = FutureRecord::to(topic)
    .key(key)
    .payload(payload);
    match producer.send(record, Duration::from_secs(0)).await{
        Ok(delivery) => println!(
            "Got IT! Partition: {}, Offset: {}", 
            delivery.partition, delivery.offset
        ),
        Err( (e, _)) => eprint!("Error to send message {:?} ", e),
    }

}

pub async fn consumer_start(topic: &str){
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "quant_workers")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true") // Auto ack
        .create()
        .expect("Falha ao criar o Kafka Consumer");

    consumer.subscribe( &[topic]).expect("Fail to subscribe in the topic");

    println!("Listening topic ' {}' ... ", topic);

    let mut message_stram = consumer.stream();

    while let Some(message) = message_stram.next().await{
        match message {
            Err(e) => eprint!("Error to receive message from Kafka: {:?} ", e),
            Ok(m) => {
                let paylod = match m.payload_view::<str>(){
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        eprintln!("Erro de desserialização: {:?}", e);
                        ""
                    }
                };
                let key = m.key_view::<str>().unwrap_or_else(|| Ok("NO_KEY")).unwrap_or("KEY_ERROR");
                println!(" Receivig: Key: {:?}, payload: {} ", m.key_view::<str>(), paylod);
            }
        }
    }
}