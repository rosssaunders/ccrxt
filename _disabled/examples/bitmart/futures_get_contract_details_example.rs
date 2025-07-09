//! Example: Get BitMart Futures Contract Details
// Demonstrates how to use the BitMart RestClient to fetch contract details (public endpoint).
//
// No credentials required for this example.
//
// Run with: cargo run --example futures_get_contract_details_example

use venues::bitmart::contract::public::rest::{RestClient, details::{GetContractDetailsRequest}};

#[tokio::main]
async fn main() {
    // Construct the REST client (no credentials needed for public endpoints)
    let client = RestClient::default();
    // Prepare the request (no symbol for all contracts)
    let req = GetContractDetailsRequest { symbol: None };
    // Call the endpoint
    match client.get_contract_details(&req).await {
        Ok(resp) => println!("Contract Details: {:#?}", resp),
        Err(e) => eprintln!("Error: {e}"),
    }
}
