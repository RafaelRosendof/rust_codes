// code to vectorize operations in Rust with threads and iterators


use std::thread;
use rand;
use std::time;
use std::io;

fn build_vector(size: usize) -> Vec<usize>{
    let mut vec1: Vec<usize> = (0..size).collect();
    for i in 0..size{
        let rng = rand::random_range(0..=100);
        vec1[i] = i+rng;
    }

    vec1
}

fn slow_function(vec: &[usize]) -> Vec<usize>{
    let mut vec2: Vec<usize> = vec.to_vec();
    
    let rng = rand::random_range(0..=1000);

    for i in 0..vec.len(){
        vec2[i] = vec[i] * rng;
    }
    vec2    
}

fn fast_built_in(vec: &[usize]) -> Vec<usize>{

    //let mut dest: Vec<usize> = vec.to_vec();
    //rand::fill(mut vec2);
//
    //vec.iter()
    //.zip(dest.iter())
    //.map(|(&x, &y)| x * y)
    //.collect()
    vec.iter()
    .map(|x| x * 55)
    .collect()

}


fn multri_thread_op(vec: &[usize]) -> Vec<usize>{
    
    let mut vec2 = vec.to_vec();

    let count = thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(1);


    let chunk_size = (vec2.len() + count - 1) / count;

    if chunk_size == 0{
        return vec2;
    }

    thread::scope(|s| {

        for chunk in vec2.chunks_mut(chunk_size){
            
            s.spawn( move || {
                for item in chunk {
                    *item *=2;
                }
            });
        }
    });

    vec2
}

fn run_experiments(size: usize, tipo: String){

    let vec1 = build_vector(size);

    if tipo.eq("normal"){
        let start_time = time::Instant::now();
        let vec2 = slow_function(&vec1);
        let elapsed_time = start_time.elapsed();
        println!("Elapsed time: {:?}", elapsed_time);
        println!("First element of vec2: {}", vec2[0]);
    }
    else if tipo.eq("built_in"){
        let start_time = time::Instant::now();
        let vec2 = fast_built_in(&vec1);
        let elapsed_time = start_time.elapsed();
        println!("Elapsed time: {:?}", elapsed_time);
        println!("First element of vec2: {}", vec2[0]);
    }
    else if tipo.eq("multi_thread"){
        let start_time = time::Instant::now();
        let vec2 = multri_thread_op(&vec1);
        let elapsed_time = start_time.elapsed();
        println!("Elapsed time: {:?}", elapsed_time);
        println!("First element of vec2: {}", vec2[0]);

    }
    
}

fn main(){

    let sizes = Vec::from([1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000]);

    for size in sizes{
        println!("Running experiment with size: {}", size);
        run_experiments(size, "normal".to_string());
        run_experiments(size, "built_in".to_string());
        run_experiments(size, "multi_thread".to_string());
        println!("----------------------------- \n\n");
    }

    

}