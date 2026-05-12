use cli_quant::{request_stock, send_request};
use quant_math::{FinanceMethod, FinancerequestTo};
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

async fn choice_3(){
    println!("Choice: Input your own data and acess the methods");
    let mut data = vec!["", "", "", "", "", "", "", "", ""];

    for i in 0..data.len() {
        println!("Put the data for {}", data[i]);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    }

    print!("Do you want to see the math options ? Or proceed type directly the method: 1 or 2");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();

    if choice == "1" {
        choice_2();
    } else {
        print!("")
    }
}

async fn choice_4(){
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
            choice_3().await;
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
    request_stock(&ticket).await;
    println!("\n\n ");
    print!("Choose a method: ");
    io::stdout().flush().unwrap();
    let mut method = String::new();
    io::stdin().read_line(&mut method).expect("Failed to read line");
    let method = method.trim();

    //TODO 
    /*
    in here i'm need to parse the data from the ticket to fit inside the method that i'm want to use.
    and maybe it gonna need a custom if else to fit in a perfect method 
     */

}


#[tokio::main]
async fn main(){

    println!("-----------------------------------------------------------------------------------------------------------------");
    println!("################# WELLCOME TO THE QUANT_FAST NEW PLATAFORM ############################");
    println!("We gonna present a intuitive menu that we will provide for you ");
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
    match choice {
        "1" => {
            choice_1().await;
        }
        "2" => {
            choice_2();
        }
        "3" => {
            println!("Choice: Input your own data and acess the methods");
        }
        "4" => {
            println!("Choice: Proceed with stock plus the math methods");
        }
        "5" => {
            println!("Choice: Proceed with monte carlo simulation");
            println!("Choice: Exit");
            break;
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