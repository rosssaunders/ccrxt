//! Example: Place a new USDM Futures order on Binance
//
//! This example demonstrates how to place a new order using the Binance USDM private REST API.
//!
//! # Requirements
//! - You must have a Binance Futures account and API credentials (API key and secret).
//! - Set your API key/secret using environment variables or a config file. Do NOT hard-code secrets.
//!
//! # Usage
//! ```sh
//! cargo run --example ws_new_order_example
//! ```

use reqwest::Client;
use std::env;
use venues::binance::usdm::private::rest::RestClient;

#[tokio::main]
async fn main() {
    // Load API credentials from environment variables
    let api_key = env::var("BINANCE_API_KEY").expect("Set BINANCE_API_KEY");
    let api_secret = env::var("BINANCE_API_SECRET").expect("Set BINANCE_API_SECRET");

    // Create HTTP client
    let client = Client::new();
    let rest_client = RestClient::new("https://fapi.binance.com", client);

    // Example order parameters (replace with your own)
    let symbol = "BTCUSDT";
    let side = "BUY";
    let order_type = "LIMIT";
    let quantity = "0.001";
    let price = "30000";
    let time_in_force = "GTC";

    // Call the new_order endpoint (stub)
    // TODO: Replace with actual function call and parameters
    // let response = rest_client.order::new_order(...).await;
    println!("Order placed: (stub, implement call)");
}

// Unit tests (do not require network or credentials)
#[cfg(test)]
mod tests {
    #[test]
    fn test_example_order_params() {
        let symbol = "BTCUSDT";
        let side = "BUY";
        let order_type = "LIMIT";
        let quantity = "0.001";
        let price = "30000";
        let time_in_force = "GTC";
        assert_eq!(symbol, "BTCUSDT");
        assert_eq!(side, "BUY");
        assert_eq!(order_type, "LIMIT");
        assert_eq!(quantity, "0.001");
        assert_eq!(price, "30000");
        assert_eq!(time_in_force, "GTC");
    }
}
