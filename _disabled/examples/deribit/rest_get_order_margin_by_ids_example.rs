//! Example: Retrieve initial margins for given order IDs using Deribit private REST API
//!
//! This example demonstrates how to use the Deribit RestClient to call the /private/get_order_margin_by_ids endpoint.
//!
//! # Usage
//! 1. Set your API key and secret in environment variables: DERIBIT_API_KEY and DERIBIT_API_SECRET.
//! 2. Run with `cargo run --example rest_get_order_margin_by_ids_example --features reqwest`
//!
//! Note: This example requires valid credentials and network access.

use venues::deribit::private::rest::client::RestClient;
use venues::deribit::private::rest::GetOrderMarginByIdsResponse;
use venues::deribit::RateLimiter;
use reqwest::Client;
use std::env;

struct EnvSecret(String);
impl rest::secrets::ExposableSecret for EnvSecret {
    fn expose_secret(&self) -> String { self.0.clone() }
}

#[tokio::main]
async fn main() {
    // Load credentials from environment
    let api_key = std::env::var("DERIBIT_API_KEY").expect("DERIBIT_API_KEY not set");
    let api_secret = std::env::var("DERIBIT_API_SECRET").expect("DERIBIT_API_SECRET not set");

    let api_key = Box::new(EnvSecret(api_key)) as Box<dyn rest::secrets::ExposableSecret>;
    let api_secret = Box::new(EnvSecret(api_secret)) as Box<dyn rest::secrets::ExposableSecret>;
    let client = Client::new();
    let rate_limiter = RateLimiter::default();
    let rest_client = RestClient::new(
        api_key,
        api_secret,
        "https://www.deribit.com",
        rate_limiter,
        client,
    );

    // Replace with your actual order IDs
    let order_ids = vec!["1234567890".to_string()];

    match rest_client.get_order_margin_by_ids(order_ids).await {
        Ok(response) => {
            println!("Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
