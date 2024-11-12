use yew::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use crate::implied_volatility::implied_volatility; // Import the implied_volatility function

// Define response structure for Yahoo Finance API
#[derive(Deserialize, Debug)]
struct ApiResponse {
    quoteResponse: QuoteResponse,
}

#[derive(Deserialize, Debug)]
struct QuoteResponse {
    result: Vec<StockData>,
}

#[derive(Deserialize, Debug)]
struct StockData {
    symbol: String,
    regularMarketPrice: f64,
}

// Define structure for Greeks
struct Greeks {
    delta_call: f64,
    delta_put: f64,
    gamma: f64,
    theta_call: f64,
    theta_put: f64,
    vega: f64,
    rho_call: f64,
    rho_put: f64,
}

// Placeholder for Greeks calculation logic
fn calculate_greeks(spot_price: f64, strike_price: f64, time: f64, rate: f64, volatility: f64) -> Greeks {
    // Replace with actual calculations
    Greeks {
        delta_call: 0.5,
        delta_put: -0.5,
        gamma: 0.1,
        theta_call: -0.01,
        theta_put: -0.02,
        vega: 0.2,
        rho_call: 0.03,
        rho_put: -0.03,
    }
}

#[function_component(GreeksTable)]
pub fn greeks_table() -> Html {
    let ticker = use_state(|| "AAPL".to_string());
    let spot_price = use_state(|| 0.0);
    let strike_price = use_state(|| 100.0);
    let time_to_expiration = use_state(|| 1.0);
    let risk_free_rate = use_state(|| 0.05);
    let market_price = use_state(|| 0.0);
    let implied_vol = use_state(|| 0.2); // Renamed to avoid conflict
    let greeks = use_state(|| calculate_greeks(*spot_price, *strike_price, *time_to_expiration, *risk_free_rate, *implied_vol));

    // Update ticker
    let on_ticker_change = {
        let ticker = ticker.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            ticker.set(value);
        })
    };

    // Fetch spot price from Yahoo Finance
    let fetch_spot_price = {
        let ticker = ticker.clone();
        let spot_price = spot_price.clone();
        Callback::from(move |_| {
            let ticker = ticker.clone();
            let spot_price = spot_price.clone();
            spawn_local(async move {
                let client = Client::new();
                let url = format!(
                    "https://yahoo-finance166.p.rapidapi.com/api/market/get-quote?symbols={}",
                    *ticker
                );
                match client
                    .get(&url)
                    .header("X-RapidAPI-Key", "8f00bd502amshb6b6aea577c2ef4p1b1975jsn7c2016df5864")
                    .header("X-RapidAPI-Host", "yahoo-finance166.p.rapidapi.com")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            if let Ok(data) = response.json::<ApiResponse>().await {
                                if let Some(stock) = data.quoteResponse.result.get(0) {
                                    spot_price.set(stock.regularMarketPrice);
                                }
                            }
                        }
                    }
                    Err(e) => web_sys::console::log_1(&format!("Error: {}", e).into()),
                }
            });
        })
    };

    // Calculate implied volatility
    let calculate_implied_volatility = {
        let implied_vol = implied_vol.clone(); // Use new name
        let s = *spot_price;
        let k = *strike_price;
        let t = *time_to_expiration;
        let r = *risk_free_rate;
        let market_price = *market_price;
        Callback::from(move |_| {
            let vol = implied_volatility(s, k, t, r, market_price, "call", 1e-5, 100);
            implied_vol.set(vol);
        })
    };

    // Calculate Greeks
    let calculate_greeks_action = {
        let greeks = greeks.clone();
        let s = *spot_price;
        let k = *strike_price;
        let t = *time_to_expiration;
        let r = *risk_free_rate;
        let vol = *implied_vol; // Use new name
        Callback::from(move |_| {
            greeks.set(calculate_greeks(s, k, t, r, vol));
        })
    };

    html! {
        <div>
            <div>
                <label>{ "Ticker Symbol: " }</label>
                <input type="text" value={(*ticker).clone()} oninput={on_ticker_change} />
                <button onclick={fetch_spot_price}>{ "Fetch Spot Price" }</button>
            </div>
            <div>
                <label>{ "Spot Price (S): " }</label>
                <input type="number" value={spot_price.to_string()} disabled=true />

                <label>{ "Strike Price (K): " }</label>
                <input type="number" value={strike_price.to_string()} oninput={Callback::from(move |e: InputEvent| strike_price.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(100.0)))} />

                <label>{ "Time to Expiration (T): " }</label>
                <input type="number" value={time_to_expiration.to_string()} oninput={Callback::from(move |e: InputEvent| time_to_expiration.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(1.0)))} />

                <label>{ "Risk-Free Rate (R): " }</label>
                <input type="number" value={risk_free_rate.to_string()} oninput={Callback::from(move |e: InputEvent| risk_free_rate.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.05)))} />

                <label>{ "Market Price: " }</label>
                <input type="number" value={market_price.to_string()} oninput={Callback::from(move |e: InputEvent| market_price.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value().parse().unwrap_or(0.0)))} />

                <label>{ "Implied Volatility (σ): " }</label>
                <input type="number" value={implied_vol.to_string()} disabled=true />
                <button onclick={calculate_implied_volatility}>{ "Calculate Implied Volatility" }</button>

                <button onclick={calculate_greeks_action}>{ "Calculate Greeks" }</button>
            </div>

            <table>
                <thead>
                    <tr><th>{ "Greek" }</th><th>{ "Value" }</th></tr>
                </thead>
                <tbody>
                    <tr><td>{ "Delta (Call)" }</td><td>{ greeks.delta_call }</td></tr>
                    <tr><td>{ "Delta (Put)" }</td><td>{ greeks.delta_put }</td></tr>
                    <tr><td>{ "Gamma" }</td><td>{ greeks.gamma }</td></tr>
                    <tr><td>{ "Theta (Call)" }</td><td>{ greeks.theta_call }</td></tr>
                    <tr><td>{ "Theta (Put)" }</td><td>{ greeks.theta_put }</td></tr>
                    <tr><td>{ "Vega" }</td><td>{ greeks.vega }</td></tr>
                    <tr><td>{ "Rho (Call)" }</td><td>{ greeks.rho_call }</td></tr>
                    <tr><td>{ "Rho (Put)" }</td><td>{ greeks.rho_put }</td></tr>
                </tbody>
            </table>
        </div>
    }
}
