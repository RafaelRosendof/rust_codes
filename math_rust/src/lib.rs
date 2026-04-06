mod remeber;

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

// european option price 

// calculate the greeks of an option ( delta, gamma, vega, theta, rho )

// monte carlo to simulate the price of an option

// calculate the 

//scraping of data from yahoo and google finance


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
