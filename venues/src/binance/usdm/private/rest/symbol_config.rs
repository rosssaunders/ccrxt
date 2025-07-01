//! User symbol configuration endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::enums::MarginType;
use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::private::rest::order::OrderErrorResponse;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;

/// Error type for USDM symbol config endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum SymbolConfigError {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Symbol not found: {0}")]
    InvalidSymbol(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM symbol config operations.
pub type SymbolConfigResult<T> = Result<T, SymbolConfigError>;

/// Request for getting symbol configuration.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSymbolConfigRequest {
    /// Trading symbol (optional, if not provided returns all symbols).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for GetSymbolConfigRequest {
    fn default() -> Self {
        Self {
            symbol: None,
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// User symbol configuration response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolConfigResponse {
    /// Trading symbol.
    pub symbol: String,
    /// Current leverage for the symbol.
    pub leverage: u32,
    /// Margin type for the symbol.
    pub margin_type: MarginType,
    /// Whether isolated positions are allowed for the symbol.
    pub is_isolated: bool,
}

impl RestClient {
    /// Get user symbol configuration.
    ///
    /// Retrieves the current configuration for a specific symbol or all symbols,
    /// including leverage, margin type, and isolation settings.
    ///
    /// Weight: 1
    ///
    /// # Arguments
    ///
    /// * `symbol` - Trading symbol (optional, if not provided returns all symbols)
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `Vec<SymbolConfigResponse>` containing symbol configurations.
    ///
    /// # Errors
    ///
    /// Returns `SymbolConfigError` if:
    /// - The symbol is invalid (if provided)
    /// - Authentication fails
    /// - Rate limits are exceeded
    pub async fn get_symbol_config(
        &self,
        symbol: Option<impl Into<String>>,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> SymbolConfigResult<Vec<SymbolConfigResponse>> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| SymbolConfigError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetSymbolConfigRequest {
            symbol: symbol.map(|s| s.into()),
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| SymbolConfigError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, &format!("{}/fapi/v1/symbolConfig", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| SymbolConfigError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let config_response: Vec<SymbolConfigResponse> = response
                .json()
                .await
                .map_err(|e| SymbolConfigError::Unknown(e.to_string()))?;
            Ok(config_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| SymbolConfigError::Unknown(e.to_string()))?;
            Err(SymbolConfigError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_symbol_config_request_serialization_with_symbol() {
        let request = GetSymbolConfigRequest {
            symbol: Some("BTCUSDT".to_string()),
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_get_symbol_config_request_serialization_without_symbol() {
        let request = GetSymbolConfigRequest {
            symbol: None,
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(!serialized.contains("symbol="));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_symbol_config_response_deserialization() {
        let json = r#"[{
            "symbol": "BTCUSDT",
            "leverage": 20,
            "marginType": "cross",
            "isIsolated": false
        }]"#;
        
        let response: Vec<SymbolConfigResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].leverage, 20);
        assert_eq!(response[0].margin_type, MarginType::Cross);
        assert!(!response[0].is_isolated);
    }
}
