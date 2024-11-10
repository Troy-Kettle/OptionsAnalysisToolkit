use statrs::function::gamma::gamma;

mod black_scholes;
mod greeks;
mod implied_volatility;
mod monte_carlo_sim;

fn main() {
    // black_scholes::black_scholes(100.00, 100.00, 1.00, 0.05, 0.20);
    // greeks::greeks(100.00, 100.00, 1.00, 0.05, 0.20);
    // implied_volatility::implied_volatility(100.0, 100.0, 1.0, 0.05, 10.0, "call", 0.0001, 100);
    //

    monte_carlo_sim::monte_carlo(100.0, 100.0, 0.05, 1.0, 0.20);


}
