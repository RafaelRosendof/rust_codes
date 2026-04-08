use lapin::{
    BasicProperties, Connection, ConnectionProperties, options::*, protocol::exchange, types::FieldTable
};
use futures_util::stream::StreamExt;
use rand_distr::Normal;
use std::{error::Error, f32::consts::EULER_GAMMA, f64::consts::{E, LN_10}, intrinsics::{powf64, sqrtf64}};
use statrs::distribution::{self, Continuous, ContinuousCDF};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


pub fn map(vec: &[usize], f: fn(usize) -> usize) -> Vec<usize>{
    vec.iter()
    .map(|&x| f(x))
    .collect()

    // run with --release
}

pub fn reduce(vec: &[usize], f: fn(usize, usize) -> usize) -> usize{
    vec.iter()
    .fold(vec[0], |acc, &x| f(acc, x))
}

// --------------- Finance math Methods ------------------

// european option price 

//TODO methods
//C(S,t) is the price of a european call option 
//P(S,t) is the price of a european put option
//V(S,t) is the price of the option as a function of the underlying asset S at time t, in particular

//help -> S_t is the stock price in time t

pub fn call_europe(S: f64, K: f64, r: f64, sig: f64, T: f64, t: f64) -> f64{

    let d1 = 1.0 / (sig * (T-t).sqrt()) * ( ( S / K).ln() + (r + sig * sig / 2.0) * (T - t) );

    let d2 = d1 - sig * (T - t).sqrt();

    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    let nd1 = n.cdf(d1) * S;
    let nd2 = n.cdf(d2);
    let term = K * E.powf(-r * (T - t));

    nd1 - nd2 * term
}

pub fn put_europe(S: f64, K: f64, r: f64, sig: f64, T: f64, t: f64) -> f64{
    let p1 = K * E.powf( r * -1.0 * (T-t)) - S; // +
    let p2 = call_europe(S, K, r, sig, T, t);

    p1 + p2
}


fn d1(S: f64, K: f64, r: f64, sig: f64, T: f64, t: f64) -> f64{

    let d1 = 1.0 / (sig * (T-t).sqrt()) * ( ( S / K).ln() + (r + sig * sig / 2.0) * (T - t) );

    d1
}


fn d2(S: f64, K: f64, r: f64, sig: f64, T: f64, t: f64) -> f64{

    let d1 = 1.0 / (sig * (T-t).sqrt()) * ( ( S / K).ln() + (r + sig * sig / 2.0) * (T - t) );

    let d2 = d1 - sig * (T - t).sqrt();

    d2
}

/*
 N'(x) denotes the standard normal probability density function: 

    Delta -> call N(d1) put -> -N(-d1)

    Gamma -> N'(d1) / S * sig * (T-t).sqrt()

    Vega -> S * N'(d1) * (T-t).sqrt()

    Theta call -( (S * N'(d1) * sig) / 2 * (T - t).sqrt() ) - 
    
    r * K * E.powf(-r * (T - t)) * N(d2) 


    Theta put  -( (S * N'(d1) * sig) / 2 * (T - t).sqrt() ) +
    
    r * K * E.powf(-r * (T - t)) * N(d2)


    Rho call -> K(T - t) * E.powf(-r * (T - t)) * N(d2)

    Rho put -> -K(T - t) * E.powf(-r * (T - t)) * N(-d2)

*/

fn call_delta(d1: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    n.cdf(d1)
}

fn put_delta(d1: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    n.cdf(-d1)
}

fn call_gamma(d1: f64, S: f64, sig: f64, T: f64, t: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();
    n.pdf(d1) / S * sig * (T-t).sqrt()
}

fn put_gamma(d1: f64, S: f64, sig: f64, T: f64, t: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();
    n.pdf(d1) / S * sig * (T-t).sqrt()
}





// monte carlo to simulate the price of an option

// calculate the variance of a stock price

// bin/options.rs


// ---------------- Scraoping data from yahoo and google finance ----------------------

//scraping of data from yahoo and google finance

// bin/scraping.rs


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

    //channel.basic_publish(
    //    exchange,
    //    routing_key,
    //    BasicPublishOptions::default(),
    //    payload, // it was with .to_vec()
    //    BasicProperties::default(),
    //)
    //.await?
    //.await?; // Second await is to confirm the message was received by the broker

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

//  bin/server.rs (build a server that listens to the rabbitMQ and responds to the rabbit to the client )

// bin/client.rs (build a client that sends a request to the rabbitMQ and listens for the response from the server )

// bin/producer.rs (build a producer that sends a message to the rabbitMQ )

// bin/consumer.rs (build a consumer that listens to the rabbitMQ and processes the messages )





// ---------------- CLI Logic ---------------------------- 

// build CLI main logic 

// build CLI commands to call all the methods above

// bin/cli.rs


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
