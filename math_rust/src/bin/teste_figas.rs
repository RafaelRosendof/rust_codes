use std::io;
use rand::{Rng, RngExt};

fn run_figas( mut x: u32) -> u32{
    
    for i in 0..x{
        println!("testing {}", i);
    }

    for i in 0..10{
        println!("testing {}", i);
    }

    for i in 0..=10{
        println!("testing {}", i);
    }


    x += 10;
    x
}

fn test_while(mut x: i32) -> i32{

    while x < 100{
        x += 10;
        println!("testing {}", x);
    }

    x -= 10;
    x
}

fn test_conds(mut y: i32) -> i32{

    if y == 10{
        print!("figas == 10")
    }
    else if y > 10{
        print!("figas > 10")
    }
    else{
        print!("figas < 10")
    }

    y+2
}

fn capture_from_terminal(){

    let mut x = String::new(); 

    io::stdin().read_line(&mut x).expect("Deu erro figas");

    println!("results of {}", x);

    let y: i32 = x.trim().parse().expect("Need to be figas");
}

fn vec_multi(a: Vec<i32>, b: Vec<i32>) -> Vec<i32>{
    let mut c = Vec::new();

    if a.len() < b.len() {
        for i in 0..a.len() {
            c.push(a[i] * b[i]);
        }
    }
    else{
        for i in 0..b.len(){
            c.push(a[i] * b[i]);
        }
    }

    c
    
}

fn vec_multi2(a: Vec<i32>, b: Vec<i32>) -> Vec<i32>{
    a.into_iter()
    .zip(b.into_iter())
    .map(|(x,y)| x*y)
    .collect()

}


fn main(){

    let a = vec![0;100];
    let b = vec![0;100];

    //let _b2 = vec![];

    let mut rand = rand::rng().random_range(1..10);

    let b2: Vec<i32> = a
    .into_iter()
    .map( |x| {
        let r: i32 = rand;
        x * r
    })
    .collect();



    print!("len of a: {}", a.len());
    print!("\n\n");
    print!("len of b: {}", b.len());
    

    // cargo run -p math_rust --bin teste_figas 
    
    println!("{}", rand);

}
