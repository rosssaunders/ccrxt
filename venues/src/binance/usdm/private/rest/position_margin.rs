//! Modify isolated position margin on Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    enums::{MarginAction, PositionSide},
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM position margin endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum PositionMarginError {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Symbol not found: {0}")]
    InvalidSymbol(String),

    #[error("Invalid position side: {0}")]
    InvalidPositionSide(String),

    #[error("Invalid margin action: {0}")]
    InvalidMarginAction(String),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Symbol margin type is not isolated: {0}")]
    SymbolNotIsolated(String),

    #[error("Position not found: {0}")]
    PositionNotFound(String),

    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM position margin operations.
pub type PositionMarginResult<T> = Result<T, PositionMarginError>;

/// Request to modify isolated position margin.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyPositionMarginRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,
    /// Position side (BOTH, LONG, or SHORT).
    pub position_side: PositionSide,
    /// Margin amount to add or reduce.
    pub amount: String,
    /// Margin action (1 for add, 2 for reduce).
    #[serde(rename = "type")]
    pub action: MarginAction,
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for ModifyPositionMarginRequest {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            position_side: PositionSide::Both,
            amount: String::new(),
            action: MarginAction::Add,
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// Response from modifying position margin.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyPositionMarginResponse {
    /// Margin amount that was modified.
    pub amount: String,
    /// Response code (200 indicates success).
    pub code: i32,
    /// Response message.
    pub msg: String,
    /// Margin action (1 for add, 2 for reduce).
    #[serde(rename = "type")]
    pub action: MarginAction,
}

impl RestClient {
    /// Modify isolated position margin.
    ///
    /// Adds or reduces margin for an isolated position. This operation is only
    /// available for symbols in isolated margin mode and requires an existing position.
    ///
    /// Weight: 1
    ///
    /// # Arguments
    ///
    /// * `symbol` - Trading symbol (e.g., "BTCUSDT")
    /// * `position_side` - Position side (BOTH for one-way mode, LONG/SHORT for hedge mode)
    /// * `amount` - Margin amount to add or reduce
    /// * `action` - Margin action (Add or Reduce)
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `ModifyPositionMarginResponse` containing the modified amount and action.
    ///
    /// # Errors
    ///
    /// Returns `PositionMarginError` if:
    /// - The symbol is invalid or not in isolated margin mode
    /// - The position side is invalid
    /// - The amount is invalid or insufficient balance for reduction
    /// - No position exists for the symbol/side
    /// - Authentication fails
    /// - Rate limits are exceeded
    pub async fn modify_position_margin(
        &self,
        symbol: impl Into<String>,
        position_side: PositionSide,
        amount: impl Into<String>,
        action: MarginAction,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> PositionMarginResult<ModifyPositionMarginResponse> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| PositionMarginError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = ModifyPositionMarginRequest {
            symbol: symbol.into(),
            position_side,
            amount: amount.into(),
            action,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| PositionMarginError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::POST,
                &format!("{}/fapi/v1/positionMargin", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .form(&request)
            .send()
            .await
            .map_err(|e| PositionMarginError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let position_margin_response: ModifyPositionMarginResponse = response
                .json()
                .await
                .map_err(|e| PositionMarginError::Unknown(e.to_string()))?;
            Ok(position_margin_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| PositionMarginError::Unknown(e.to_string()))?;
            Err(PositionMarginError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_position_margin_request_serialization() {
        let request = ModifyPositionMarginRequest {
            symbol: "BTCUSDT".to_string(),
            position_side: PositionSide::Long,
            amount: "100.0".to_string(),
            action: MarginAction::Add,
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("positionSide=LONG"));
        assert!(serialized.contains("amount=100.0"));
        assert!(serialized.contains("type=1"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_modify_position_margin_response_deserialization() {
        let json = r#"{"amount":"100.0","code":200,"msg":"success","type":"1"}"#;
        let response: ModifyPositionMarginResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.amount, "100.0");
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
        assert_eq!(response.action, MarginAction::Add);
    }
}
