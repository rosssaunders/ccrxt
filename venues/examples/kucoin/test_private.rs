//! Test private API example

use rest::secrets::SecretString;
use venues::kucoin::private::rest::RestClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing KuCoin private API credentials...");

    // Test credentials (won't make actual API calls without real credentials)
    let api_key = "test_key";
    let api_secret = "test_secret";
    let api_passphrase = "test_passphrase";

    // Create client with credentials using SecretString directly
    let _client = RestClient::new_with_credentials(
        Box::new(SecretString::new(api_key.to_string().into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(api_secret.to_string().into())) as Box<dyn rest::secrets::ExposableSecret>,
        Box::new(SecretString::new(api_passphrase.to_string().into())) as Box<dyn rest::secrets::ExposableSecret>,
    );

    println!("✅ Client created successfully with credentials!");
    println!("Note: This example doesn't make actual API calls without real credentials.");

    Ok(())
}
