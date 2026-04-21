use futures_util::stream::StreamExt;
use std::{error::Error};
use rand_distr::{Distribution, num_traits::Pow};
use rayon::{iter::{IntoParallelIterator}, prelude};
use rayon::prelude::*;
use std::{f64::consts::{E}};
use statrs::distribution;
use statrs::distribution::{Continuous, ContinuousCDF};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]

pub struct OptionInputs {
    pub s: f64,    // Spot price
    pub k: f64,    // Strike price
    pub r: f64,    // Risk-free rate
    pub sig: f64,  // Volatility
    pub t: f64,    // Time to maturity
}

pub struct GreeksOptions{
    pub S: f64,
    pub K: f64,
    pub r: f64,
    pub sig: f64,
    pub T: f64,
    pub t: f64,
}

pub struct FinanceData{
    // 
}


pub enum FinanceMethod{
    CallEurope,
    PutEurope,
    CallDelta,
    PutDelta,
    CallGamma,
    PutGamma,
    Vega,
    ThetaCall,
    ThetaPut,
    RhoCall,
    RhoPut,
    MonteCarlo,
    MonteCarloFast,
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



pub fn call_europe_f(S: f64, K: f64, r: f64, sig: f64, T: f64, t: f64) -> f64{

    let d1 = 1.0 / (sig * (T-t).sqrt()) * ( ( S / K).ln() + (r + sig * sig / 2.0) * (T - t) );

    let d2 = d1 - sig * (T - t).sqrt();

    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    let nd1 = n.cdf(d1) * S;
    let nd2 = n.cdf(d2);
    let term = K * E.powf(-r * (T - t));

    nd1 - nd2 * term
}

pub fn put_europe_f(S: f64, K: f64, r: f64, sig: f64, T: f64, t: f64) -> f64{
    let p1 = K * E.powf( r * -1.0 * (T-t)) - S; // +
    let p2 = call_europe_f(S, K, r, sig, T, t);

    p1 + p2
}


fn d1(S: f64, K: f64, r: f64, sig: f64, T: f64, t: f64) -> f64{

    let d1 = 1.0 / (sig * (T-t).sqrt()) * ( ( S / K).ln() + (r + sig * sig / 2.0) * (T - t) );

    d1
}


fn d2(S: f64, K: f64, r: f64, sig: f64, T: f64, t: f64) -> f64{

    let d1 = 1.0 / (sig * (T-t).sqrt()) * ( ( S / K).ln() + (r + sig * sig / 2.0) * (T - t) );

    let d2 = d1 - sig * (T - t).sqrt();

    d2
}

fn call_delta_f(d1: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    n.cdf(d1)
}

fn put_delta_f(d1: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    n.cdf(-d1)
}

fn call_gamma_f(d1: f64, S: f64, sig: f64, T: f64, t: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();
    n.pdf(d1) / S * sig * (T-t).sqrt()
}

fn put_gamma_f(d1: f64, S: f64, sig: f64, T: f64, t: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();
    n.pdf(d1) / S * sig * (T-t).sqrt()
}

fn vega_f(S: f64, d1: f64, T: f64, t: f64) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();
    S * n.pdf(d1) * (T-t).sqrt()
}

fn theta_call_f(
            S: f64, d1: f64, sig: f64,
            T: f64, t: f64, r: f64,
            K: f64, d2: f64,
        ) -> f64{

            let n = distribution::Normal::new(0.0, 1.0).unwrap();
            
            let f1 = -(S * n.pdf(d1) * sig) / 2.0 * (T - t).sqrt();

            let f2 = r * K * E.powf(-r * (T - t)) * n.cdf(d2);
            
            f1 - f2

            
        }

fn theta_put_f(
            S: f64, d1: f64, sig: f64,
            T: f64, t: f64, r: f64,
            K: f64, d2: f64,
        ) -> f64{

            let n = distribution::Normal::new(0.0, 1.0).unwrap();
            
            let f1 = -(S * n.pdf(d1) * sig) / 2.0 * (T - t).sqrt();

            let f2 = r * K * E.powf(-r * (T - t)) * n.cdf(d2);
            
            f1 + f2
    }


fn rho_call_f(
        K: f64, T: f64, t: f64,
        r: f64, d2: f64
) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    let call = K * (T - t) * E.powf(-r * (T - t)) * n.cdf(d2);

    call 
}

fn rho_put_f(
        K: f64, T: f64, t: f64,
        r: f64, d2: f64
) -> f64{
    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    let put = -K * (T - t) * E.powf(-r * (T - t)) * n.cdf(-d2);

    put
}

pub fn monte_carlo_f(
    s0: f64,
    r: f64,
    sig: f64,
    t: f64,
    K: f64,
    steps: usize,
    iterations: usize,
) -> f64{

    let mut rng = rand::thread_rng();
    let n = distribution::Normal::new(0.0, 1.0).unwrap();

    let dt = t / steps as f64;

    let mut total_payoff = 0.0;

    let sgm = |r: f64, sig: f64, dt: f64, z: f64 |

    ((r - 0.5 * sig * sig) * dt + sig * dt.sqrt() * z).exp();


    for _ in 0..iterations{
        let mut st = s0;
        for _ in 0..steps{
            let z = n.sample(& mut rng);
            st = sgm(r, sig, dt, z);
        }
        
        let payoff = (st - K).max(0.0);
        total_payoff += payoff;
    
        } 

    (total_payoff / iterations as f64) * (-r * t).exp()
}

// fast monte carlo
pub fn monte_carlo_fast_f(
    s0: f64,
    r: f64,
    sig: f64,
    t: f64,
    K: f64,
    steps: usize,
    iterations: usize,
) -> f64{

    let dt = t / steps as f64;
    let drift = (r - 0.5 * sig * sig) * dt;
    let vol = sig * dt.sqrt();

    let total_payoff: f64 = (0..iterations)
    .into_par_iter()
    .map(|_| {

        let mut rng = rand::thread_rng();
        let n = distribution::Normal::new(0.0, 1.0).unwrap();

        let st = (0..steps).fold(s0, |acc_s, _ |{
            let z = n.sample(&mut rng);
            acc_s * (drift + vol * z).exp()
        });

        (st - K).max(0.0)
    })
    .sum();
    
    (total_payoff / iterations as f64) * (-r * t).exp()
}

pub fn finance_factory(method: FinanceMethod, data: &GreeksOptions) -> f64{
    let GreeksOptions {S, K, r, sig, T, t} = *data;

    let d1 = || d1(S, K, r, sig, T, t);
    let d2 = || d2(S, K, r, sig, T, t);

    match method {
        FinanceMethod::CallDelta => call_delta_f(d1()),
        FinanceMethod::PutDelta => put_delta_f(d1()),
        FinanceMethod::PutEurope => put_europe_f(S, K, r, sig, T, t),
        FinanceMethod::CallEurope => call_europe_f(S, K, r, sig, T, t),
        FinanceMethod::CallGamma => call_gamma_f(d1(), S, sig, T, t),
        FinanceMethod::PutGamma => put_gamma_f(d1(), S, sig, T, t),
        FinanceMethod::Vega => vega_f(S, d1(), T, t),
        FinanceMethod::ThetaCall => theta_call_f(S, d1(), sig, T, t, r, K, d2()),
        FinanceMethod::ThetaPut => theta_put_f(S, d1(), sig, T, t, r, K, d2()),
        FinanceMethod::RhoCall => rho_call_f(K, T, t, r, d2()),
        FinanceMethod::RhoPut => rho_put_f(K, T, t, r, d2()),
        FinanceMethod::MonteCarlo => monte_carlo_f(S, r, sig, T, K, 1000, 10_000),
        FinanceMethod::MonteCarloFast => monte_carlo_fast_f(S, r, sig, T, K, 1000, 10_000),
    }
}
