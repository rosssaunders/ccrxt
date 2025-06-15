//! Example usage of the Coinbase Exchange private REST API
//!
//! This example demonstrates how to:
//! - Create a private REST client
//! - Authenticate with the Coinbase Exchange API
//! - Get account balances

use rest::secrets::SecretValue;
use secrecy::SecretString;
use venues::coinbase::{
    GetAccountBalancesRequest, PrivateRestClient, RateLimiter,
};

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the HTTP client
    let http_client = reqwest::Client::new();
    
    // Initialize the rate limiter
    let rate_limiter = RateLimiter::new();
    
    // Create API credentials (these would come from environment variables or secure storage in practice)
    let api_key = Box::new(SecretValue::new(SecretString::new(
        std::env::var("COINBASE_API_KEY")
            .expect("COINBASE_API_KEY environment variable not set")
            .into()
    )));
    
    let api_secret = Box::new(SecretValue::new(SecretString::new(
        std::env::var("COINBASE_API_SECRET")
            .expect("COINBASE_API_SECRET environment variable not set")
            .into()
    )));
    
    let api_passphrase = Box::new(SecretValue::new(SecretString::new(
        std::env::var("COINBASE_API_PASSPHRASE")
            .expect("COINBASE_API_PASSPHRASE environment variable not set")
            .into()
    )));
    
    // Create the private REST client
    let client = PrivateRestClient::new(
        api_key,
        api_secret,
        api_passphrase,
        "https://api.exchange.coinbase.com", // Use sandbox URL for testing: https://api-public.sandbox.exchange.coinbase.com
        http_client,
        rate_limiter,
    );
    
    // Example 1: Get account balances
    println!("Fetching account balances...");
    let balances_request = GetAccountBalancesRequest::default();
    
    match client.get_account_balances(&balances_request).await {
        Ok(response) => {
            println!("Account balances:");
            for account in response.accounts {
                println!(
                    "  Currency: {}, Balance: {}, Available: {}, Hold: {}",
                    account.currency, account.balance, account.available, account.hold
                );
            }
        }
        Err(e) => {
            eprintln!("Error fetching account balances: {}", e);
        }
    }
    
    // Example 2: Get account balances with pagination
    println!("\nFetching account balances with pagination...");
    let balances_request = GetAccountBalancesRequest {
        before: None,
        after: None,
        limit: Some(10), // Limit to 10 results per page
    };
    
    match client.get_account_balances(&balances_request).await {
        Ok(response) => {
            println!("First 10 account balances:");
            for account in response.accounts {
                println!(
                    "  ID: {}, Currency: {}, Balance: {}",
                    account.id, account.currency, account.balance
                );
            }
            
            // Display pagination info if available
            if let Some(pagination) = response.pagination {
                if let Some(before) = pagination.before {
                    println!("  Before cursor: {}", before);
                }
                if let Some(after) = pagination.after {
                    println!("  After cursor: {}", after);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching paginated account balances: {}", e);
        }
    }
    
    Ok(())
}

/// Example of how to set up the client with sandbox credentials for testing
#[allow(dead_code)]
fn create_sandbox_client() -> PrivateRestClient {
    use rest::secrets::SecretValue;
    use secrecy::SecretString;
    
    let http_client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();
    
    // Example sandbox credentials (replace with your actual sandbox credentials)
    let api_key = Box::new(SecretValue::new(SecretString::new(
        "your_sandbox_api_key".to_string().into()
    )));
    
    let api_secret = Box::new(SecretValue::new(SecretString::new(
        "your_base64_encoded_sandbox_secret".to_string().into()
    )));
    
    let api_passphrase = Box::new(SecretValue::new(SecretString::new(
        "your_sandbox_passphrase".to_string().into()
    )));
    
    PrivateRestClient::new(
        api_key,
        api_secret,
        api_passphrase,
        "https://api-public.sandbox.exchange.coinbase.com", // Sandbox URL
        http_client,
        rate_limiter,
    )
}