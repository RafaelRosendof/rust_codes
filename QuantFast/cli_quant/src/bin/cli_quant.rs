use cli_quant::{request_stock, send_request};
use quant_math::{FinanceMethod, FinancerequestTo};
use std::io::{self, Write};

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
    (5) -> Exit  
        ");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    let choice = choice.trim();
        // in here, i'm gonna make a better function, each choice gonna be a single function be better in this way
    match choice {
        "1" => {
            //print!("Choice: Select a stock info \n");
            print!("Please insert the stock name: ");
            io::stdout().flush().unwrap();
            let mut stock_name = String::new();
            std::io::stdin().read_line(&mut stock_name).expect("Failed to read line");
            let stock_name = stock_name.trim();
            request_stock(stock_name).await;
            println!("\n\n ");
            
        }
        "2" => {
            println!("Choice: See all of our methods");
        }
        "3" => {
            println!("Choice: Input your own data and acess the methods");
        }
        "4" => {
            println!("Choice: Proceed with stock plus the math methods");
        }
        "5" => {
            println!("Choice: Exit");
            break;
        }
        _ => {
            println!("Invalid choice. Please try again.");
        }
        }
    }
}