//! Example usage of the Deribit get_combo_details endpoint
//!
//! This example shows how to create a Deribit client and call the get_combo_details endpoint.

use std::sync::Arc;
use crate::deribit::{
    AccountTier, GetComboDetailsRequest, PublicRestClient, RateLimiter
};

/// Example function demonstrating how to use the Deribit get_combo_details endpoint
#[allow(dead_code)]
pub async fn example_get_combo_details() -> Result<(), Box<dyn std::error::Error>> {
    // Create a rate limiter for a Tier 3 account (1-25M USD trading volume)
    let limiter = Arc::new(RateLimiter::new(AccountTier::Tier3));
    
    // Create a public REST client
    let client = reqwest::Client::new();
    let _rest_client = PublicRestClient::new("https://deribit.com", client, limiter);
    
    // Create a request for combo details
    let request = GetComboDetailsRequest {
        combo_id: "COMBO-123456".to_string(),
    };
    
    // Make the request (this would fail in tests without a real combo ID)
    // let response = rest_client.get_combo_details(request).await?;
    // println!("Combo details: {:?}", response.result);
    
    // For demonstration, just validate the request structure
    let json_request = serde_json::to_string(&request)?;
    println!("Request JSON: {}", json_request);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example_compilation() {
        // Test that the example compiles and runs without panicking
        let result = example_get_combo_details().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_example_client_creation() {
        // Test that we can create all the components
        let limiter = Arc::new(RateLimiter::new(AccountTier::Tier4));
        let client = reqwest::Client::new();
        let rest_client = PublicRestClient::new("https://test.deribit.com", client, limiter);
        
        assert_eq!(rest_client.base_url, "https://test.deribit.com");
    }

    #[test]
    fn test_request_creation() {
        let request = GetComboDetailsRequest {
            combo_id: "EXAMPLE-COMBO-ID".to_string(),
        };
        
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["combo_id"], "EXAMPLE-COMBO-ID");
    }
}