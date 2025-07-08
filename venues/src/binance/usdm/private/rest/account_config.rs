//! User account configuration endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM account config endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum AccountConfigError {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM account config operations.
pub type AccountConfigResult<T> = Result<T, AccountConfigError>;

/// Request for getting account configuration.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountConfigRequest {
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for GetAccountConfigRequest {
    fn default() -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// User account configuration response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountConfigResponse {
    /// Fee tier.
    pub fee_tier: u32,
    /// Whether the account can trade.
    pub can_trade: bool,
    /// Whether the account can deposit.
    pub can_deposit: bool,
    /// Whether the account can withdraw.
    pub can_withdraw: bool,
    /// Fee burn option for spot trading.
    pub fee_burn: bool,
    /// Multi-assets margin enabled.
    pub multi_assets_margin: bool,
    /// Last update time.
    pub update_time: u64,
}

impl RestClient {
    /// Get user account configuration.
    ///
    /// Retrieves the current account configuration including fee tier,
    /// trading permissions, and margin settings.
    ///
    /// Weight: 5
    ///
    /// # Arguments
    ///
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `AccountConfigResponse` containing account configuration.
    ///
    /// # Errors
    ///
    /// Returns `AccountConfigError` if:
    /// - Authentication fails
    /// - Rate limits are exceeded
    /// - API error occurs
    pub async fn get_account_config(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> AccountConfigResult<AccountConfigResponse> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| AccountConfigError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetAccountConfigRequest {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| AccountConfigError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::GET,
                format!("{}/fapi/v1/accountConfig", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| AccountConfigError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let config_response: AccountConfigResponse = response
                .json()
                .await
                .map_err(|e| AccountConfigError::Unknown(e.to_string()))?;
            Ok(config_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| AccountConfigError::Unknown(e.to_string()))?;
            Err(AccountConfigError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_config_request_serialization() {
        let request = GetAccountConfigRequest {
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_account_config_response_deserialization() {
        let json = r#"{
            "feeTier": 0,
            "canTrade": true,
            "canDeposit": true,
            "canWithdraw": true,
            "feeBurn": false,
            "multiAssetsMargin": false,
            "updateTime": 1625097600000
        }"#;

        let response: AccountConfigResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.fee_tier, 0);
        assert!(response.can_trade);
        assert!(!response.multi_assets_margin);
    }
}
