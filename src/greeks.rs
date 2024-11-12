use std::f64::consts::E;
use statrs::distribution::{ContinuousCDF, Normal};

pub struct Greeks {
    pub delta_call: f64,
    pub delta_put: f64,
    pub gamma: f64,
    pub theta_call: f64,
    pub theta_put: f64,
    pub vega: f64,
    pub rho_call: f64,
    pub rho_put: f64,
}

pub fn calculate_greeks(s: f64, k: f64, t: f64, r: f64, sig: f64) -> Greeks {
    let mean = 0.0;
    let std_dev = 1.0;
    let normal = Normal::new(mean, std_dev).unwrap();

    let d1 = ((s / k).ln() + (r + (sig * sig) / 2.0) * t) / (sig * t.sqrt());
    let d2 = d1 - (sig * t.sqrt());

    let nd1_neg = normal.cdf(-d1);
    let nd2 = normal.cdf(d2);
    let nd2_neg = normal.cdf(-d2);

    Greeks {
        delta_call: normal.cdf(d1),
        delta_put: normal.cdf(d1) - 1.0,
        gamma: gamma(s, sig, t, d1),
        theta_call: theta_call(s, k, t, r, sig, d1, nd2),
        theta_put: theta_put(s, k, t, r, sig, d1, nd2_neg),
        vega: vega(s, t, nd1_neg),
        rho_call: rho_call(k, t, nd2, r),
        rho_put: rho_put(k, t, nd2_neg, r),
    }
}

fn gamma(s: f64, sig: f64, t: f64, d1: f64) -> f64 {
    let a = nd_1(d1);
    a / (s * sig * t.sqrt())
}

fn theta_call(s: f64, k: f64, t: f64, r: f64, sig: f64, d1: f64, nd2: f64) -> f64 {
    let term1 = (s * nd_1(d1) * sig) / (2.0 * t.sqrt());
    let term2 = (r * k) * E.powf(-r * t) * nd2;
    -term1 - term2
}

fn theta_put(s: f64, k: f64, t: f64, r: f64, sig: f64, d1: f64, nd2_neg: f64) -> f64 {
    let term1 = (s * nd_1(d1) * sig) / (2.0 * t.sqrt());
    let term2 = (r * k) * E.powf(-r * t) * nd2_neg;
    -term1 + term2
}

fn vega(s: f64, t: f64, nd1_neg: f64) -> f64 {
    s * t.sqrt() * nd1_neg
}

fn rho_call(k: f64, t: f64, nd2: f64, r: f64) -> f64 {
    k * t * E.powf(-r * t) * nd2
}

fn rho_put(k: f64, t: f64, nd2_neg: f64, r: f64) -> f64 {
    -k * t * E.powf(-r * t) * nd2_neg
}

fn nd_1(d1: f64) -> f64 {
    (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * E.powf(-d1 * d1 / 2.0)
}
