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

//fn vec_multi(a: Vec<i32>, b: Vec<i32>) -> Vec<i32>{
fn vec_multi(a: &[i32], b: &[i32]) -> Vec<i32>{
    let mut c = Vec::new();

    let len_2 = std::cmp::min(a.len(), b.len());

    for i in 0..len_2{
        c.push(a[i] * b[i]);
    }
    c
    
}

fn vec_multi2(a: &[i32], b: &[i32]) -> Vec<i32>{
    a.iter()
    .zip(b.iter())
    .map(|(x,y)| x*y)
    .collect()

}


fn main(){

    //let a = vec![1;100];
    let mut a: Vec<i32> = (0..100).collect();
    let mut b = vec![1;100];
    let mut c_random = vec![1;100];

    for i in 0..c_random.len(){
        c_random[i] = rand::rng().random_range(1..100);
    }


    let mut rand_fi = rand::rng().random_range(1..10);

    for i in 0..a.len(){
        a[i] = a[i] * rand_fi;
    }

    print!("{}\n", rand_fi);
    //for i in &a{
    //    println!("a2: {}", i);
    //}

    for i in 0..b.len(){
        
        b[i] = b[i] * rand_fi;
    }

    println!("Vendo um exemplar de c {}", c_random[10]);

    let c1 = vec_multi(&a, &b);
    let c2 = vec_multi(&c_random, &b);
    let c3 = vec_multi2(&a, &b);

    for i in &c1{
        println!("c1: {}", i);
    }


}
