use std::f64::consts::E;
use rand::{thread_rng, Rng};
use rand_distr::{Normal, Distribution};

pub(crate) fn monte_carlo(s: f64, k: f64, r: f64, t: f64, sig: f64) {
    let num_of_paths = 100_000;
    let normal = Normal::new(0.0, 1.0).unwrap();
    let discount_factor = E.powf(-r * t);
    let mut rng = thread_rng();

    let mut total_call_payoff = 0.0;
    let mut total_put_payoff = 0.0;

    for _ in 0..num_of_paths {
        // rand sample from norm distribution
        let z: f64 = normal.sample(&mut rng);

        // calc sim price
        let s_end = s * E.powf((r - 0.5 * sig * sig) * t + sig * t.sqrt() * z);

        // calc pay offs
        let call_option_payoff = f64::max(s_end - k, 0.0);
        let put_option_payoff = f64::max(k - s_end, 0.0);

        // calc sum
        total_call_payoff += call_option_payoff;
        total_put_payoff += put_option_payoff;
    }

    // calc discounted avg
    let call_option_price = (total_call_payoff / num_of_paths as f64) * discount_factor;
    let put_option_price = (total_put_payoff / num_of_paths as f64) * discount_factor;

    println!("Estimated Call Option Price: {}", call_option_price);
    println!("Estimated Put Option Price: {}", put_option_price);
}
