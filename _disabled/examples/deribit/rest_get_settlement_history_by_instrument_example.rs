//! Example: Retrieve settlement, delivery, and bankruptcy events for a Deribit instrument
//!
//! This example demonstrates how to use the private REST API to fetch settlement history for a given instrument.
//!
//! # Prerequisites
//! - Set the `DERIBIT_CLIENT_ID` and `DERIBIT_CLIENT_SECRET` environment variables with your API credentials.
//!
//! # Usage
//! ```sh
//! cargo run --example rest_get_settlement_history_by_instrument_example --features deribit
//! ```

use std::env;
use venues::deribit::{AccountTier, GetSettlementHistoryByInstrumentRequest, PrivateRestClient, RateLimiter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment variables
    let client_id = env::var("DERIBIT_CLIENT_ID")?;
    let client_secret = env::var("DERIBIT_CLIENT_SECRET")?;

    // Create a rate limiter (choose your account tier)
    let limiter = RateLimiter::new(AccountTier::Tier3);
    let client = reqwest::Client::new();
    let rest_client = PrivateRestClient::new(
        "https://www.deribit.com",
        client,
        limiter,
        client_id,
        client_secret,
    );

    // Build the request
    let request = GetSettlementHistoryByInstrumentRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        r#type: None, // You can use Some("settlement"), Some("delivery"), or Some("bankruptcy")
        count: Some(5),
        continuation: None,
        search_start_timestamp: None,
    };

    // Send the request
    let response = rest_client
        .get_settlement_history_by_instrument(request)
        .await?;
    println!("Settlement events:");
    for event in response.settlements {
        println!("{:#?}", event);
    }
    Ok(())
}
