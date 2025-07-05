//! Example: Get BitMart Futures Contract Assets
// Demonstrates how to use the BitMart RestClient to fetch contract asset details.
//
// Requires API credentials (do not hard-code secrets; use environment variables or a secure method).
//
// Run with: cargo run --example futures_get_contract_assets_example

use secrecy::SecretString;
use venues::bitmart::contract::private::rest::{RestClient, assets_detail::GetContractAssetsRequest};

/// Reads credentials from environment variables
fn get_credentials() -> (SecretString, SecretString) {
    let api_key = std::env::var("BITMART_API_KEY").expect("BITMART_API_KEY not set");
    let api_secret = std::env::var("BITMART_API_SECRET").expect("BITMART_API_SECRET not set");
    (SecretString::new(api_key), SecretString::new(api_secret))
}

#[tokio::main]
async fn main() {
    // Get credentials securely
    let (api_key, api_secret) = get_credentials();
    // Construct the REST client
    let client = RestClient::new(api_key, api_secret);
    // Prepare the request
    let req = GetContractAssetsRequest;
    // Call the endpoint
    match client.get_contract_assets(&req).await {
        Ok(resp) => println!("Assets: {:#?}", resp),
        Err(e) => eprintln!("Error: {e}"),
    }
}
