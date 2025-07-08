//! Change margin type for a symbol on Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    enums::MarginType,
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM margin type endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum MarginTypeError {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Symbol not found: {0}")]
    InvalidSymbol(String),

    #[error("Invalid margin type: {0}")]
    InvalidMarginType(String),

    #[error("Margin type cannot be changed with open positions: {0}")]
    MarginTypeChangeWithOpenPositions(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM margin type operations.
pub type MarginTypeResult<T> = Result<T, MarginTypeError>;

/// Request to change margin type for a symbol.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginTypeRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,
    /// New margin type (cross or isolated).
    pub margin_type: MarginType,
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for ChangeMarginTypeRequest {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            margin_type: MarginType::Cross,
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// Response from changing margin type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginTypeResponse {
    /// Response code (200 indicates success).
    pub code: i32,
    /// Response message.
    pub msg: String,
}

impl RestClient {
    /// Change margin type for a symbol.
    ///
    /// Changes the margin type (cross or isolated) for a specific symbol.
    /// This operation is not allowed when there are open positions for the symbol.
    ///
    /// Weight: 1
    ///
    /// # Arguments
    ///
    /// * `symbol` - Trading symbol (e.g., "BTCUSDT")
    /// * `margin_type` - New margin type (cross or isolated)
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `ChangeMarginTypeResponse` on success.
    ///
    /// # Errors
    ///
    /// Returns `MarginTypeError` if:
    /// - The symbol is invalid
    /// - The margin type is invalid
    /// - There are open positions for the symbol
    /// - Authentication fails
    /// - Rate limits are exceeded
    pub async fn change_margin_type(
        &self,
        symbol: impl Into<String>,
        margin_type: MarginType,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> MarginTypeResult<ChangeMarginTypeResponse> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| MarginTypeError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = ChangeMarginTypeRequest {
            symbol: symbol.into(),
            margin_type,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| MarginTypeError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::POST,
                &format!("{}/fapi/v1/marginType", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .form(&request)
            .send()
            .await
            .map_err(|e| MarginTypeError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let margin_type_response: ChangeMarginTypeResponse = response
                .json()
                .await
                .map_err(|e| MarginTypeError::Unknown(e.to_string()))?;
            Ok(margin_type_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| MarginTypeError::Unknown(e.to_string()))?;
            Err(MarginTypeError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_margin_type_request_serialization() {
        let request = ChangeMarginTypeRequest {
            symbol: "BTCUSDT".to_string(),
            margin_type: MarginType::Cross,
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("marginType=cross"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_change_margin_type_response_deserialization() {
        let json = r#"{"code":200,"msg":"success"}"#;
        let response: ChangeMarginTypeResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }
}
