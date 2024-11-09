use statrs::distribution::{ContinuousCDF, Normal};

pub(crate) fn implied_volatility(
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    market_price: f64,
    option_type: &str,
    tol: f64,
    max_iter: usize,
) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut sig: f64 = 0.20;

    for _ in 0..max_iter {
        let d1 = ((s / k).ln() + (r + (sig * sig) / 2.0) * t) / (sig * t.sqrt());
        let d2 = d1 - (sig * t.sqrt());

        let bs_price = match option_type {
            "call" => s * normal.cdf(d1) - k * (-r * t).exp() * normal.cdf(d2),
            "put" => k * (-r * t).exp() * normal.cdf(-d2) - s * normal.cdf(-d1),
            _ => panic!("Invalid option type. Use 'call' or 'put'."),
        };

        let nd1_pdf = (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * (-d1 * d1 / 2.0).exp();
        let vega = s * t.sqrt() * nd1_pdf;

        let price_diff = bs_price - market_price;

        // Check if the solution is within the tolerance level
        if price_diff.abs() < tol {
            println!("Implied volatility: {}", sig); // Print sig here when converged
            return sig;
        }

        sig -= price_diff / vega;
    }

    sig
}
