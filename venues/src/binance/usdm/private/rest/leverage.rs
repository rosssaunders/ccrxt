//! Change initial leverage for a symbol on Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM leverage endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum LeverageError {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Symbol not found: {0}")]
    InvalidSymbol(String),

    #[error("Invalid leverage: {0}")]
    InvalidLeverage(String),

    #[error("Leverage cannot be changed with open positions: {0}")]
    LeverageChangeWithOpenPositions(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM leverage operations.
pub type LeverageResult<T> = Result<T, LeverageError>;

/// Request to change initial leverage for a symbol.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,
    /// Target initial leverage (1 to 125).
    pub leverage: u32,
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for ChangeLeverageRequest {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            leverage: 1,
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// Response from changing leverage.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    /// Current leverage value after change.
    pub leverage: u32,
    /// Maximum notional value for this leverage.
    pub max_notional_value: String,
    /// Trading symbol.
    pub symbol: String,
}

impl RestClient {
    /// Change initial leverage for a symbol.
    ///
    /// Changes the initial leverage for the specified symbol. The leverage value
    /// must be between 1 and 125, and the maximum allowed leverage depends on
    /// the symbol and the user's risk level.
    ///
    /// Weight: 1
    ///
    /// # Arguments
    ///
    /// * `symbol` - Trading symbol (e.g., "BTCUSDT")
    /// * `leverage` - Target initial leverage (1 to 125)
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `ChangeLeverageResponse` containing the new leverage and max notional value.
    ///
    /// # Errors
    ///
    /// Returns `LeverageError` if:
    /// - The symbol is invalid
    /// - The leverage value is invalid or exceeds the maximum allowed
    /// - There are open positions that prevent leverage change
    /// - Authentication fails
    /// - Rate limits are exceeded
    pub async fn change_leverage(
        &self,
        symbol: impl Into<String>,
        leverage: u32,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> LeverageResult<ChangeLeverageResponse> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| LeverageError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = ChangeLeverageRequest {
            symbol: symbol.into(),
            leverage,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| LeverageError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::POST, &format!("{}/fapi/v1/leverage", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .form(&request)
            .send()
            .await
            .map_err(|e| LeverageError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let leverage_response: ChangeLeverageResponse = response
                .json()
                .await
                .map_err(|e| LeverageError::Unknown(e.to_string()))?;
            Ok(leverage_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| LeverageError::Unknown(e.to_string()))?;
            Err(LeverageError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_leverage_request_serialization() {
        let request = ChangeLeverageRequest {
            symbol: "BTCUSDT".to_string(),
            leverage: 10,
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("leverage=10"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_change_leverage_response_deserialization() {
        let json = r#"{"leverage":10,"maxNotionalValue":"1000000","symbol":"BTCUSDT"}"#;
        let response: ChangeLeverageResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.leverage, 10);
        assert_eq!(response.max_notional_value, "1000000");
        assert_eq!(response.symbol, "BTCUSDT");
    }
}
