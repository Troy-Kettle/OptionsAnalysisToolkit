mod black_scholes;

fn main() {
    black_scholes::black_scholes(100.00, 100.00, 1.00, 0.05, 0.20, "put")
}
