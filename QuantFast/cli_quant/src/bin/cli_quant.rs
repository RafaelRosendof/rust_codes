use cli_quant::{create_local_consumer, request_stock, send_request};
use lapin::protocol::{channel, queue};
use quant_math::{FinanceMethod, FinancerequestTo, FinanceRequest};
use rayon::vec;
use std::io::{self, Write};


async fn choice_1(){
    print!("Choice: Select a stock info: ");
    io::stdout().flush().unwrap();
    let mut stock_name = String::new();
    io::stdin().read_line(&mut stock_name).expect("Failed to read line");
    request_stock(&stock_name).await;
    println!("\n\n ");
}

fn choice_2(){
    println!("Choice: See all of our methods");

    let methods = vec![
        FinanceMethod::CallEurope,
        FinanceMethod::PutEurope,
        FinanceMethod::CallDelta,
        FinanceMethod::PutDelta,
        FinanceMethod::CallGamma,
        FinanceMethod::PutGamma,
        FinanceMethod::Vega,
        FinanceMethod::ThetaCall,
        FinanceMethod::ThetaPut,
        FinanceMethod::RhoCall,
        FinanceMethod::RhoPut,
        FinanceMethod::MonteCarlo,
        FinanceMethod::MonteCarloFast,
    ];

    for method in methods {
        println!("{}", method);
    }
    println!("\n\n ");
}

async fn choice_3(queue_name: &str, exchange: &str, routing_key: &str){
    println!("Choice: Input your own data and acess the methods");

    println!("Do you want to see the math options ? Or proceed with the stocks? 1 or 2");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();
    match choice {
        "1" => {
            choice_2();
        }
        "2" => {
            //choice_3(queue_name, exchange, routing_key).await;
            // _
        }
        _ => {
            println!("Invalid choice. Please try again.");
        }
    }

    println!("\n");
    io::stdout().flush().unwrap();

        
    
    print!("Choose a method: for each method in the screen you just need to put the number of the method, 0 for the first and so on: ");
    io::stdout().flush().unwrap();
    let mut method = String::new();
    io::stdin().read_line(&mut method).expect("Failed to read line");
    let method = method.trim();

    let finance_method = match method {
        "0" => FinanceMethod::CallEurope,
        "1" => FinanceMethod::PutEurope,
        "2" => FinanceMethod::CallDelta,
        "3" => FinanceMethod::PutDelta,
        "4" => FinanceMethod::CallGamma,
        "5" => FinanceMethod::PutGamma,
        "6" => FinanceMethod::Vega,
        "7" => FinanceMethod::ThetaCall,
        "8" => FinanceMethod::ThetaPut,
        "9" => FinanceMethod::RhoCall,
        "10" => FinanceMethod::RhoPut,
        "11" => FinanceMethod::MonteCarlo,
        "12" => FinanceMethod::MonteCarloFast,
        _ => {
            println!("Invalid choice. Please try again.");
            return;
        }
    };

    println!("\n\n ");
    io::stdout().flush().unwrap();

    let mut data_fields = vec!["s","k", "r", "sig", "t_expiry", "t_start"];
    let mut data = vec![0.0f64; data_fields.len()];

    for (i, &field) in data_fields.iter().enumerate() {
        loop {
            print!("Put the data for {}: ", field);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<f64>() {
                Ok(val) => { data[i] = val; break; }
                Err(_) => println!("Invalid number, please try again."),
            }
        }
    }
    

    let finance_data = FinancerequestTo{
        to: queue_name.to_string(), // Ex: "reply_queue_cliente_123"
        method: finance_method,
        params: FinanceRequest{
            s: data[0],
            k: data[1],
            r: data[2],
            sig: data[3],
            t_expiry: data[4],
            t_start: data[5],
        }
    };

    println!("\n\n ");
    println!("Your request is gonna be \n {:?}", finance_data);

    if let Err(e) = send_request(finance_data, exchange, routing_key).await {
        println!("Failed to send request: {:?}", e);
    }
    println!("\n");
  
}

async fn choice_4(queue_name: &str, exchange: &str, routing_key: &str){
    println!("Choice: Proceed with stock plus the math methods");
    println!("Do you want to see the math options ? Or proceed with the stocks? 1 or 2");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();
    match choice {
        "1" => {
            choice_2();
        }
        "2" => {
            //choice_3(queue_name, exchange, routing_key).await;
        }
        _ => {
            println!("Invalid choice. Please try again.");
        }
    }
    println!("\n\n ");
    print!("Proceed with the ticket stock: ");
    
    io::stdout().flush().unwrap();
    let mut ticket = String::new();
    io::stdin().read_line(&mut ticket).expect("Failed to read line");
    let ticket = ticket.trim();
    let value = request_stock(&ticket).await;
    
    println!("\n\n ");

    print!("Choose a method: for each method in the screen you just need to put the number of the method, 0 for the first and so on: ");
    io::stdout().flush().unwrap();
    let mut method = String::new();
    io::stdin().read_line(&mut method).expect("Failed to read line");
    let method = method.trim();

    let finance_method = match method {
        "0" => FinanceMethod::CallEurope,
        "1" => FinanceMethod::PutEurope,
        "2" => FinanceMethod::CallDelta,
        "3" => FinanceMethod::PutDelta,
        "4" => FinanceMethod::CallGamma,
        "5" => FinanceMethod::PutGamma,
        "6" => FinanceMethod::Vega,
        "7" => FinanceMethod::ThetaCall,
        "8" => FinanceMethod::ThetaPut,
        "9" => FinanceMethod::RhoCall,
        "10" => FinanceMethod::RhoPut,
        "11" => FinanceMethod::MonteCarlo,
        "12" => FinanceMethod::MonteCarloFast,
        _ => {
            println!("Invalid choice. Please try again.");
            return;
        }
    };

    println!("\n\n ");
    print!("So now that you choose your method lets put your data: the current stock price is {:?}", value);
    io::stdout().flush().unwrap();

    let mut data_fields = vec!["k", "r", "sig", "t_expiry", "t_start"];
    let mut data = vec![0.0f64; data_fields.len()];

    for (i, &field) in data_fields.iter().enumerate() {
        loop {
            print!("Put the data for {}: ", field);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<f64>() {
                Ok(val) => { data[i] = val; break; }
                Err(_) => println!("Invalid number, please try again."),
            }
        }
    }
    

    let f64_value = value.unwrap_or(0.0);
    let finance_data = FinancerequestTo{
        to: queue_name.to_string(), // Ex: "reply_queue_cliente_123"
        method: finance_method,
        params: FinanceRequest{
            s: f64_value,
            k: data[0],
            r: data[1],
            sig: data[2],
            t_expiry: data[3],
            t_start: data[4],
        }
    };

    println!("\n\n ");
    println!("Your request is gonna be \n {:?}", finance_data);

    if let Err(e) = send_request(finance_data, exchange, routing_key).await {
        println!("Failed to send request: {:?}", e);
    }
    println!("\n\n ");
    //TODO 
    /*
    in here i'm need to parse the data from the ticket to fit inside the method that i'm want to use.
    and maybe it gonna need a custom if else to fit in a perfect method 
     */

}

async fn choice_5(){
    println!("Choice: Proceed with monte carlo simulation");
    //todo
}


#[tokio::main]
async fn main(){

    println!("-----------------------------------------------------------------------------------------------------------------");
    println!("################# WELLCOME TO THE QUANT_FAST NEW PLATAFORM ############################");
    println!("We gonna present a intuitive menu that we will provide for you ");
    
    let reply_queue = uuid::Uuid::new_v4().to_string();
    if let Err(e) = create_local_consumer(&reply_queue).await {
        eprintln!("Erro ao criar o consumidor local: {}", e);
        return;
    }

    println!("[*] Consumer queue created: {}", reply_queue);
    println!("\n\n\n");

    println!("\n\n\n");
    loop {
        println!( " 
        
        Menu Stocks Option 

    (1) -> Select a stock info 
    (2) -> See all of our methods
    (3) -> Input your own data and acess the methods
    (4) -> Proceed with stock plus the math methods
    (5) -> Proceed with monte carlo simulation
    (6) -> Exit
       
        ");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    let choice = choice.trim();
        // in here, i'm gonna make a better function, each choice gonna be a single function be better in this way
    let exchange_to_send = "figas_clay";
    let routing_key_to_send = "figas_test";
    match choice {
        "1" => {
            choice_1().await;
        }
        "2" => {
            choice_2();
        }
        "3" => {
            //println!("Choice: Input your own data and acess the methods");
            choice_3(&reply_queue, exchange_to_send, routing_key_to_send).await;
        }
        "4" => {
            //println!("Choice: Proceed with stock plus the math methods");
            choice_4(&reply_queue, exchange_to_send, routing_key_to_send).await;
        }
        "5" => {
            println!("Choice: Proceed with monte carlo simulation");
            choice_5().await;
        }
        "6" => {
            println!("Choice: Exit");
            break;
        }
        _ => {
            println!("Invalid choice. Please try again.");
        }
        }
    }
}