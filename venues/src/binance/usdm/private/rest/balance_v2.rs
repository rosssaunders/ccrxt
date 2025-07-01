//! Future account balance V2 endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::private::rest::order::OrderErrorResponse;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;

/// Error type for USDM balance V2 endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum BalanceV2Error {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM balance V2 operations.
pub type BalanceV2Result<T> = Result<T, BalanceV2Error>;

/// Request for getting account balance V2.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceV2Request {
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for GetBalanceV2Request {
    fn default() -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// Account balance V2 response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceV2Response {
    /// Account alias.
    pub account_alias: String,
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
}

impl RestClient {
    /// Get future account balance V2.
    ///
    /// Retrieves the current account balance for all assets with basic information
    /// including cross wallet balance and unrealized PnL.
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
    /// Returns `Vec<BalanceV2Response>` containing balance information for all assets.
    ///
    /// # Errors
    ///
    /// Returns `BalanceV2Error` if:
    /// - Authentication fails
    /// - Rate limits are exceeded
    /// - API error occurs
    pub async fn get_balance_v2(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> BalanceV2Result<Vec<BalanceV2Response>> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| BalanceV2Error::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetBalanceV2Request {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| BalanceV2Error::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, &format!("{}/fapi/v2/balance", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| BalanceV2Error::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let balance_response: Vec<BalanceV2Response> = response
                .json()
                .await
                .map_err(|e| BalanceV2Error::Unknown(e.to_string()))?;
            Ok(balance_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| BalanceV2Error::Unknown(e.to_string()))?;
            Err(BalanceV2Error::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_balance_v2_request_serialization() {
        let request = GetBalanceV2Request {
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_balance_v2_response_deserialization() {
        let json = r#"[{
            "accountAlias": "futures",
            "asset": "USDT",
            "balance": "1000.00000000",
            "crossWalletBalance": "1000.00000000",
            "crossUnPnl": "0.00000000",
            "availableBalance": "1000.00000000",
            "maxWithdrawAmount": "1000.00000000"
        }]"#;
        
        let response: Vec<BalanceV2Response> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].asset, "USDT");
        assert_eq!(response[0].balance, "1000.00000000");
        assert_eq!(response[0].account_alias, "futures");
    }
}
