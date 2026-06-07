

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let topic = "test_topic";
    println!("############# Starting ################");
    net_rabbit::consumer_start(topic).await;

    Ok(())
}