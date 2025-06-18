//! Example usage of the get_block_rfq_trades endpoint
//!
//! This example demonstrates how to use the newly implemented endpoint.

use venues::deribit::{
    Currency, GetBlockRfqTradesRequest, AccountTier, RateLimiter, 
    PublicRestClient as RestClient
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a public REST client
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    let rest_client = RestClient::new("https://www.deribit.com", client, rate_limiter);

    // Example 1: Get BTC block RFQ trades with basic parameters
    let request = GetBlockRfqTradesRequest {
        currency: Currency::BTC,
        continuation: None,
        count: Some(10),
    };

    println!("Making request for BTC block RFQ trades...");
    // Uncomment the following line to make actual API call:
    // let response = rest_client.get_block_rfq_trades(request).await?;
    // println!("Received {} block RFQs", response.result.block_rfqs.len());

    // Example 2: Get all currencies with pagination
    let request_all = GetBlockRfqTradesRequest {
        currency: Currency::Any,
        continuation: Some(50), // Start from ID 50 and go backwards
        count: Some(25),
    };

    println!("Making request for all currencies with continuation...");
    // Uncomment the following line to make actual API call:
    // let response_all = rest_client.get_block_rfq_trades(request_all).await?;
    // println!("Received {} block RFQs", response_all.result.block_rfqs.len());

    // Example 3: Demonstrate request serialization
    let json_request = serde_json::to_string_pretty(&request)?;
    println!("Example request JSON:\n{}", json_request);

    println!("Example completed successfully!");
    Ok(())
}