use std::f64::consts::E;
use statrs::distribution::{ContinuousCDF, Normal};

pub(crate) fn greeks(s: f64, k: f64, t: f64, r: f64, sig: f64) {
    let mean = 0.0;
    let std_dev = 1.0;
    let normal = Normal::new(mean, std_dev).unwrap();

    let d1 = ((s / k).ln() + (r + (sig * sig) / 2.0) * t) / (sig * t.sqrt());
    let d2 = d1 - (sig * t.sqrt());

    let nd1 = normal.cdf(d1);
    let nd1_neg = normal.cdf(-d1);
    let nd2 = normal.cdf(d2);
    let nd2_neg = normal.cdf(-d2);

    println!(
        "Delta (Call, Put): {:?}\nGamma: {}\nTheta (Call): {}\nTheta (Put): {}\nVega: {}\nRho (Call): {}\nRho (Put): {}",
        deltas(d1, &normal),
        gamma(s, sig, t, d1),
        theta_call(s, k, t, r, sig, d1, nd2),
        theta_put(s, k, t, r, sig, d1, nd2_neg),
        vega(s, t, nd1),
        rho_call(k, t, nd2, r),
        rho_put(k, t, nd2_neg, r)
    );

    fn deltas(d1: f64, normal: &Normal) -> (f64, f64) {
        let delta_call = normal.cdf(d1);
        let delta_put = normal.cdf(d1) - 1.0;
        (delta_call, delta_put)
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

    fn vega(s: f64, t: f64, nd1: f64) -> f64 {
        s * t.sqrt() * nd1
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
}
