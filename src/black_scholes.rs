use statrs::distribution::{ContinuousCDF, Normal};
pub(crate)


fn black_scholes(s:f64, k:f64, t:f64, r:f64 ,sig:f64) -> (f64, f64){


    // This is just for the cumulative normal create
    let mean = 0.0;
    let std_dev = 1.0;
    let normal = Normal::new(mean, std_dev).unwrap();

    // Calcs probability factors that represent the value of a call option
    let d1 = ((s.ln()) / k) + ((r + (sig*sig) / 2_f64) * t);
    let d2 = &d1 - (sig * t.sqrt());

    // Calcs cumulative probabilities (the chance the stock ends up higher than the strike price)
    let nd1 = normal.cdf(d1);
    let nd2 = normal.cdf(d2);


    // formula for the put price
    let nd1_neg = normal.cdf(-d1);
    let nd2_neg = normal.cdf(-d2);

    // calcs for put and call
    let call_price = (s * nd1) - (k * (-r * t).exp()) * nd2;
    let put_price = k * ((-r * t).exp()) * nd2_neg - s * nd1_neg;

    (call_price ,put_price)
 }