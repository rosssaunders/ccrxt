//! KuCoin Private API Example

use std::env;
use rest::secrets::SecretString;
use venues::kucoin::private::rest::RestClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KuCoin Private API Example ===\n");
    
    // Load credentials from environment (for demo purposes, using default values)
    let api_key = env::var("KUCOIN_API_KEY").unwrap_or_else(|_| "demo_key".to_string());
    let api_secret = env::var("KUCOIN_API_SECRET").unwrap_or_else(|_| "demo_secret".to_string());
    let api_passphrase = env::var("KUCOIN_API_PASSPHRASE").unwrap_or_else(|_| "demo_passphrase".to_string());

    // Create client with credentials
    let _client = RestClient::new_with_credentials(
        Box::new(SecretString::new(api_key.into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(api_secret.into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(api_passphrase.into())) as Box<dyn rest::secrets::ExposableSecret>,
    );

    println!("✅ Private REST client created successfully!");
    println!("Note: To use actual API endpoints, set environment variables:");
    println!("  KUCOIN_API_KEY=your_api_key");
    println!("  KUCOIN_API_SECRET=your_api_secret");
    println!("  KUCOIN_API_PASSPHRASE=your_api_passphrase");

    Ok(())
}
