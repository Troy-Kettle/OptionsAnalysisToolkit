use std::f32::consts::E;
use statrs::distribution::{ContinuousCDF, Normal};

pub(crate)

fn greeks(s:f64, k:f64, t:f64, r:f64 ,sig:f64){
    // This is just for the cumulative normal create
    let mean = 0.0;
    let std_dev = 1.0;
    let normal = Normal::new(mean, std_dev).unwrap();

    // Calcs probability factors that represent the value of a call option
    let d1 = ((s.ln()) / k) + ((r + (sig*sig) / 2_f64) * t);
    let d2 = &d1 - (sig * t.sqrt());

    let nd1_neg = normal.cdf(-d1);

    let nd_2 = normal.cdf(-d2);

    theta_call(100.00, 100.00, 1.00, 0.05, 0.20, 0.5, nd_2);


    fn nd_1(d1: f64) -> f64 {
        let nd_1 = (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * (std::f64::consts::E.powf(-(d1 * d1) / 2.0));
        nd_1
    }


    fn deltas(d1: f64, normal: Normal) -> (f64, f64) {
        // function to return deltas

        let delta_call = normal.cdf(d1);
        let delta_put= (normal.cdf(d1) - 1_f64);

        (delta_call, delta_put)
    }


    // Calcs gamma
    fn gamma(s: f64, sig: f64, t: f64, d1: f64) -> f64 {
        let a: f64 = nd_1(d1);
        let output: f64 = a / (s * sig * t.sqrt());
        output
    }


    // The theta calcs
    fn theta_call(s: f64, k: f64, t: f64, r: f64, sig: f64, d1: f64, nd_2: f64) -> f64 {
        let term1 = (s * nd_1(d1) * sig) / (2.0 * t.sqrt());
        let term2 = (r * k) * std::f64::consts::E.powf(-r * t) * nd_2;
        -term1 - term2
    }

    fn theta_put(s: f64, k: f64, t: f64, r: f64, sig: f64, d1: f64, nd_2: f64) -> f64 {
        let term1 = (s * nd_1(d1) * sig) / (2.0 * t.sqrt());
        let term2 = (r * k) * std::f64::consts::E.powf(-r * t) * (-nd_2);
        -term1 - term2
    }

    fn vega(s: f64, t: f64, nd_1_neg: f64) -> f64{
        let vega :f64 = (s * (t.sqrt() * (nd_1_neg)));
        vega
    }

    fn rho_call(k: f64, t: f64, nd_2: f64, r: f64) -> f64{
        let rho_call = k * (t * E: f64.powf(r-t)) * nd_2;
        rho_call
    }

    fn rho_put(k: f64, t: f64, nd_2_neg: f64, r: f64) -> f64{
        let rho_put = k * (t * E: f64.powf(r-t)) * nd_2_neg;
        rho_put
    }
}




