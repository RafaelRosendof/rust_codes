/*


*/
use std::vec;
use std::sync::mpsc;
use sha2::{Sha256, Digest};

#[derive(Debug)]

struct Transaction{
    id: i32,
    fee: f32,
    income_account: String,
    outcome_account: String,
    value: f32,
}

struct Block{
    id: i32,
    transactions: Vec<Transaction>,
    hash: String,
    size: i32,
}

struct Blockchain{
    blocks: Vec<Block>,
}

impl Transaction{
    
    fn new(id: i32, fee: f32, income_account: String, outcome_account: String, value: f32) -> Self{
        Transaction {
            id,
            fee,
            income_account,
            outcome_account,
            value,
        }
    }
    
}

impl Block{

    fn new(id: i32, transactions: Vec<Transaction>, hash: String, size: i32) -> Self{
        Block {
            id,
            transactions,
            hash,
            size,
        }
    }

    fn add_transaction(&mut self, transaction: Transaction){
        self.transactions.push(transaction);
        self.size += 1;
    }

    fn size_of_block(&self) -> i32{
        self.size
    }

    fn print_cell(&self, transaction: &Transaction){
        println!("id: {}", transaction.id);
        println!("fee: {}", transaction.fee);
        println!("income_account: {}", transaction.income_account);
        println!("outcome_account: {}", transaction.outcome_account);
        println!("value: {}", transaction.value);
    }

    fn print_block(&self){
        println!("Block ID: {}", self.id);
        println!("Hash: {}", self.hash);
        println!("Size: {}", self.size);
        println!("Transactions:");
        for transaction in &self.transactions{
            self.print_cell(transaction.clone());
            println!("-------------------");
        }
    }


    fn calculate_hash(&self) -> String{
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", self));
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl Blockchain{

    fn new() -> Self{
        Blockchain {
            blocks: Vec::new(),
        }
    }

}
