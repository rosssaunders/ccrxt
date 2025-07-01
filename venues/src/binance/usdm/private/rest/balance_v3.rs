//! Future account balance V3 endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::private::rest::order::OrderErrorResponse;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;

/// Error type for USDM balance V3 endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum BalanceV3Error {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM balance V3 operations.
pub type BalanceV3Result<T> = Result<T, BalanceV3Error>;

/// Request for getting account balance V3.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceV3Request {
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for GetBalanceV3Request {
    fn default() -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// Account balance V3 response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceV3Response {
    /// Asset name.
    pub asset: String,
    /// Account balance.
    pub balance: String,
    /// Cross wallet balance.
    pub cross_wallet_balance: String,
    /// Cross unrealized PnL.
    pub cross_un_pnl: String,
    /// Available balance.
    pub available_balance: String,
    /// Maximum amount for transfer out.
    pub max_withdraw_amount: String,
    /// Whether the asset can be used as margin in Multi-Assets mode.
    pub margin_available: bool,
    /// Last update time.
    pub update_time: u64,
}

impl RestClient {
    /// Get future account balance V3.
    ///
    /// Retrieves the current account balance for all assets with detailed information
    /// including cross wallet balance, unrealized PnL, and margin availability.
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
    /// Returns `Vec<BalanceV3Response>` containing balance information for all assets.
    ///
    /// # Errors
    ///
    /// Returns `BalanceV3Error` if:
    /// - Authentication fails
    /// - Rate limits are exceeded
    /// - API error occurs
    pub async fn get_balance_v3(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> BalanceV3Result<Vec<BalanceV3Response>> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| BalanceV3Error::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetBalanceV3Request {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| BalanceV3Error::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, &format!("{}/fapi/v3/balance", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| BalanceV3Error::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let balance_response: Vec<BalanceV3Response> = response
                .json()
                .await
                .map_err(|e| BalanceV3Error::Unknown(e.to_string()))?;
            Ok(balance_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| BalanceV3Error::Unknown(e.to_string()))?;
            Err(BalanceV3Error::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_balance_v3_request_serialization() {
        let request = GetBalanceV3Request {
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_balance_v3_response_deserialization() {
        let json = r#"[{
            "asset": "USDT",
            "balance": "1000.00000000",
            "crossWalletBalance": "1000.00000000",
            "crossUnPnl": "0.00000000",
            "availableBalance": "1000.00000000",
            "maxWithdrawAmount": "1000.00000000",
            "marginAvailable": true,
            "updateTime": 1625097600000
        }]"#;

        let response: Vec<BalanceV3Response> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].asset, "USDT");
        assert_eq!(response[0].balance, "1000.00000000");
        assert!(response[0].margin_available);
    }
}
