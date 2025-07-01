//! Example request implementations showing how to use the unified Binance client.
//!
//! This module provides examples and utilities for making requests to different
//! Binance venues using the unified client architecture.

use serde::{Deserialize, Serialize};

use super::client::BinanceClient;
use super::errors::Errors;
use super::venue_trait::VenueConfig;

/// Example: Get server time (public endpoint, all venues)
#[derive(Deserialize)]
pub struct ServerTime {
    #[serde(rename = "serverTime")]
    pub server_time: i64,
}

impl<V: VenueConfig> BinanceClient<V> {
    /// Get the server time from any Binance venue
    pub async fn get_server_time(&self) -> Result<ServerTime, Errors> {
        let response = self
            .send_public_request::<ServerTime, ()>("/api/v3/time", reqwest::Method::GET, None, 1)
            .await?;

        Ok(response.data)
    }
}

/// Example: Account information request (signed endpoint)
#[derive(Serialize)]
pub struct AccountInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

#[derive(Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "makerCommission")]
    pub maker_commission: i32,
    #[serde(rename = "takerCommission")]
    pub taker_commission: i32,
    #[serde(rename = "buyerCommission")]
    pub buyer_commission: i32,
    #[serde(rename = "sellerCommission")]
    pub seller_commission: i32,
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    pub balances: Vec<Balance>,
}

#[derive(Deserialize)]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

impl<V: VenueConfig> BinanceClient<V> {
    /// Get account information (requires authentication)
    pub async fn get_account_info(&self, recv_window: Option<u64>) -> Result<AccountInfo, Errors> {
        let request = AccountInfoRequest { recv_window };

        let response = self
            .send_signed_request::<AccountInfo, _>(
                "/api/v3/account",
                reqwest::Method::GET,
                request,
                10,    // weight
                false, // not an order
            )
            .await?;

        Ok(response.data)
    }
}

/// Example usage patterns
#[cfg(test)]
mod examples {
    use super::*;
    use crate::binance::shared::venue_trait::configs::*;
    use rest::secrets::{ExposableSecret, SecretValue};
    use secrecy::SecretString;

    #[tokio::test]
    async fn example_public_request() {
        // Create a client for any venue (no authentication needed for public endpoints)
        let _spot_client = BinanceClient::new(SpotConfig);

        // All venues support the server time endpoint
        // Note: This is a test example - we're not actually making the request
        // let server_time = spot_client.get_server_time().await.unwrap();
        // println!("Server time: {}", server_time.server_time);
    }

    #[tokio::test]
    async fn example_authenticated_request() {
        // Create an authenticated client
        let api_key = Box::new(SecretValue::new(SecretString::from("your_api_key"))) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(SecretValue::new(SecretString::from("your_api_secret"))) as Box<dyn ExposableSecret>;

        let _client = BinanceClient::new_authenticated(SpotConfig, api_key, api_secret);

        // Make authenticated requests
        // let account_info = client.get_account_info(None).await.unwrap();
        // println!("Account balances: {:?}", account_info.balances);
    }

    #[tokio::test]
    async fn example_multi_venue_usage() {
        // Create separate secrets for each venue (since trait objects can't be cloned)
        let spot_api_key = Box::new(SecretValue::new(SecretString::from("your_api_key"))) as Box<dyn ExposableSecret>;
        let spot_api_secret = Box::new(SecretValue::new(SecretString::from("your_api_secret"))) as Box<dyn ExposableSecret>;
        
        let futures_api_key = Box::new(SecretValue::new(SecretString::from("your_api_key"))) as Box<dyn ExposableSecret>;
        let futures_api_secret = Box::new(SecretValue::new(SecretString::from("your_api_secret"))) as Box<dyn ExposableSecret>;

        // Create clients for different venues
        let spot_client =
            BinanceClient::new_authenticated(SpotConfig, spot_api_key, spot_api_secret);

        let futures_client =
            BinanceClient::new_authenticated(UsdmConfig, futures_api_key, futures_api_secret);

        // Each client automatically handles venue-specific:
        // - Base URLs
        // - Rate limits
        // - Error codes

        assert_eq!(spot_client.venue().base_url(), "https://api.binance.com");
        assert_eq!(
            futures_client.venue().base_url(),
            "https://fapi.binance.com"
        );

        // Rate limits are different
        assert_ne!(
            spot_client.venue().rate_limits().request_weight_limit,
            futures_client.venue().rate_limits().request_weight_limit
        );
    }

    #[tokio::test]
    async fn example_rate_limit_monitoring() {
        let client = BinanceClient::new(SpotConfig);

        // Check current usage
        let stats = client.get_usage_stats().await;
        println!("Weight used: {}/{}", stats.weight_used, stats.weight_limit);
        println!(
            "Orders (10s): {}/{}",
            stats.orders_10s_used, stats.orders_10s_limit
        );

        // The client automatically:
        // - Checks limits before requests
        // - Records usage after successful requests
        // - Updates from response headers
        // - Handles 429 rate limit responses with retry
    }
}
