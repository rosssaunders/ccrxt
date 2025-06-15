//! Example usage of the Deribit public/exchange_token endpoint
//!
//! This example demonstrates how to use the exchange_token endpoint to generate
//! tokens for switching between subaccounts.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::Client;
    use crate::deribit::{
        public::rest::{ExchangeTokenRequest, RestClient},
        AccountTier, RateLimiter,
    };

    // Create HTTP client and rate limiter
    let client = Client::new();
    let rate_limiter = RateLimiter::new(AccountTier::Tier4); // Adjust tier based on your account

    // Create the Deribit public REST client
    let rest_client = RestClient::new("https://www.deribit.com", client, rate_limiter);

    // Prepare the exchange token request
    let request = ExchangeTokenRequest {
        refresh_token: "your_refresh_token_here".to_string(),
        subject_id: 12345, // Target subject ID for the new token
        scope: Some("session".to_string()), // Optional scope override
    };

    // Note: This is just an example - in a real application you would handle the response
    println!("Exchange token request prepared:");
    println!("  Refresh token: {}", request.refresh_token);
    println!("  Subject ID: {}", request.subject_id);
    println!("  Scope: {:?}", request.scope);

    // In a real application, you would call:
    // let response = rest_client.exchange_token(&request).await?;
    // println!("Access token: {}", response.access_token);
    // println!("Expires in: {} seconds", response.expires_in);
    // println!("Token type: {}", response.token_type);

    println!("\nExample completed successfully!");
    println!("The endpoint is ready to use with valid Deribit credentials.");

    Ok(())
}