use QuantFast::{get_connection, create_consumer};
use lapin::{types::FieldTable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection("127.0.0.1", 5672).await?;
    let channel = conn.create_channel().await?;

    let arguments = FieldTable::default();
    let exchange = "figas_clay";
    let routing_key = "figas_test";
    let queue_name = "figas_test";


    create_consumer(&channel, queue_name, exchange, routing_key, arguments).await?;

    Ok(())
}