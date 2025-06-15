//! Example usage of the Deribit public /auth endpoint
//!
//! This example demonstrates how to:
//! - Create a public REST client
//! - Authenticate with different grant types
//! - Handle OAuth responses

use crate::deribit::{
    AccountTier, PublicRestClient, RateLimiter,
    public::rest::AuthRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the HTTP client
    let http_client = reqwest::Client::new();
    
    // Initialize the rate limiter for Tier 4 account (default)
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    
    // Create the public REST client
    let client = PublicRestClient::new(
        "https://www.deribit.com/api/v2", // Use test.deribit.com for testing
        http_client,
        rate_limiter,
    );
    
    // Example 1: Authenticate with client credentials
    println!("Example 1: Client Credentials Authentication");
    let client_credentials_request = AuthRequest::client_credentials(
        "your_client_id".to_string(),
        "your_client_secret".to_string(),
    ).with_scope("session:test".to_string());
    
    match client.auth(&client_credentials_request).await {
        Ok(response) => {
            println!("✅ Authentication successful!");
            println!("  Access Token: {}...", &response.result.access_token[..20]);
            println!("  Token Type: {}", response.result.token_type);
            println!("  Expires In: {} seconds", response.result.expires_in);
            println!("  Scope: {}", response.result.scope);
        }
        Err(e) => {
            println!("❌ Authentication failed: {}", e);
        }
    }
    
    // Example 2: Authenticate with refresh token
    println!("\nExample 2: Refresh Token Authentication");
    let refresh_token_request = AuthRequest::refresh_token(
        "your_refresh_token".to_string(),
    ).with_state("example_state".to_string());
    
    match client.auth(&refresh_token_request).await {
        Ok(response) => {
            println!("✅ Token refresh successful!");
            println!("  New Access Token: {}...", &response.result.access_token[..20]);
            println!("  New Refresh Token: {}...", &response.result.refresh_token[..20]);
            if let Some(state) = response.result.state {
                println!("  State: {}", state);
            }
        }
        Err(e) => {
            println!("❌ Token refresh failed: {}", e);
        }
    }
    
    // Example 3: Authenticate with client signature (advanced)
    println!("\nExample 3: Client Signature Authentication");
    let timestamp = chrono::Utc::now().timestamp_millis();
    
    // Note: In a real implementation, you would calculate the signature using HMAC-SHA256
    // with your secret key according to Deribit's documentation
    let signature = "calculated_signature_here".to_string();
    
    let client_signature_request = AuthRequest::client_signature(
        "your_client_id".to_string(),
        timestamp,
        signature,
    )
    .with_nonce("random_nonce_123".to_string())
    .with_data("optional_user_data".to_string());
    
    match client.auth(&client_signature_request).await {
        Ok(response) => {
            println!("✅ Signature authentication successful!");
            println!("  Access Token: {}...", &response.result.access_token[..20]);
            println!("  Enabled Features: {:?}", response.result.enabled_features);
            println!("  Google Login: {}", response.result.google_login);
        }
        Err(e) => {
            println!("❌ Signature authentication failed: {}", e);
        }
    }
    
    Ok(())
}

/// Helper function to calculate HMAC-SHA256 signature for client_signature grant type
/// 
/// This is a simplified example - refer to Deribit documentation for the exact algorithm
#[allow(dead_code)]
fn calculate_signature(
    client_id: &str,
    timestamp: i64,
    nonce: Option<&str>,
    data: Option<&str>,
    secret_key: &str,
) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    // Build the signature payload according to Deribit specification
    let mut payload = format!("{}{}", timestamp, client_id);
    
    if let Some(nonce) = nonce {
        payload.push_str(nonce);
    }
    
    if let Some(data) = data {
        payload.push_str(data);
    }
    
    // Calculate HMAC-SHA256
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    
    hex::encode(mac.finalize().into_bytes())
}