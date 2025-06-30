use tokio;
use venues::gateio::public::RestClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create public client
    let client = RestClient::new(false)?;

    // Test basic functionality - get server time
    println!("Testing Gate.io Options API access...");
    
    // Check if we can at least create the client
    println!("✓ Client created successfully");
    println!("  Base URL: {}", client.base_url());
    
    // Try some basic public endpoints that should work
    match client.get::<serde_json::Value>("/spot/time").await {
        Ok(time) => println!("✓ Server time: {:?}", time),
        Err(e) => println!("✗ Server time error: {}", e),
    }

    // Try options endpoints directly
    match client.get::<serde_json::Value>("/options/underlyings").await {
        Ok(underlyings) => println!("✓ Options underlyings: {} found", 
            underlyings.as_array().map(|a| a.len()).unwrap_or(0)),
        Err(e) => println!("✗ Options underlyings error: {}", e),
    }

    match client.get::<serde_json::Value>("/options/expirations").await {
        Ok(expirations) => println!("✓ Options expirations: {} found", 
            expirations.as_array().map(|a| a.len()).unwrap_or(0)),
        Err(e) => println!("✗ Options expirations error: {}", e),
    }

    Ok(())
}