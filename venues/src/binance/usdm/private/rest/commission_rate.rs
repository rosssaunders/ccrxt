//! User commission rate endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM commission rate endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum CommissionRateError {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Symbol not found: {0}")]
    InvalidSymbol(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM commission rate operations.
pub type CommissionRateResult<T> = Result<T, CommissionRateError>;

/// Request for getting commission rate.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionRateRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for GetCommissionRateRequest {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// User commission rate response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRateResponse {
    /// Trading symbol.
    pub symbol: String,
    /// Maker commission rate.
    pub maker_commission_rate: String,
    /// Taker commission rate.
    pub taker_commission_rate: String,
}

impl RestClient {
    /// Get user commission rate.
    ///
    /// Retrieves the current maker and taker commission rates for a specific symbol.
    /// The commission rate varies based on the user's VIP level and trading volume.
    ///
    /// Weight: 20
    ///
    /// # Arguments
    ///
    /// * `symbol` - Trading symbol (e.g., "BTCUSDT")
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `CommissionRateResponse` containing maker and taker commission rates.
    ///
    /// # Errors
    ///
    /// Returns `CommissionRateError` if:
    /// - The symbol is invalid
    /// - Authentication fails
    /// - Rate limits are exceeded
    pub async fn get_commission_rate(
        &self,
        symbol: impl Into<String>,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> CommissionRateResult<CommissionRateResponse> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(20)
            .await
            .map_err(|e| CommissionRateError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetCommissionRateRequest {
            symbol: symbol.into(),
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| CommissionRateError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::GET,
                format!("{}/fapi/v1/commissionRate", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| CommissionRateError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let commission_response: CommissionRateResponse = response
                .json()
                .await
                .map_err(|e| CommissionRateError::Unknown(e.to_string()))?;
            Ok(commission_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| CommissionRateError::Unknown(e.to_string()))?;
            Err(CommissionRateError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_commission_rate_request_serialization() {
        let request = GetCommissionRateRequest {
            symbol: "BTCUSDT".to_string(),
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_commission_rate_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "makerCommissionRate": "0.0002",
            "takerCommissionRate": "0.0004"
        }"#;

        let response: CommissionRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.maker_commission_rate, "0.0002");
        assert_eq!(response.taker_commission_rate, "0.0004");
    }
}
