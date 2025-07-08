//! Change position mode (dual position side) on Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    enums::PositionMode,
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM position mode endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum PositionModeError {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Invalid dual side position mode: {0}")]
    InvalidDualSidePositionMode(String),

    #[error("Position mode change not allowed with open positions: {0}")]
    PositionModeChangeWithOpenPositions(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM position mode operations.
pub type PositionModeResult<T> = Result<T, PositionModeError>;

/// Request to change position mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionModeRequest {
    /// Dual side position mode.
    /// - true: Hedge Mode (dual position side)
    /// - false: One-way Mode (both position side)
    pub dual_side_position: PositionMode,
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for ChangePositionModeRequest {
    fn default() -> Self {
        Self {
            dual_side_position: PositionMode::False,
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// Response from changing position mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionModeResponse {
    /// Response code (200 indicates success).
    pub code: i32,
    /// Response message.
    pub msg: String,
}

impl RestClient {
    /// Change position mode (dual position side).
    ///
    /// Changes the position mode for the account between hedge mode (dual position side)
    /// and one-way mode (both position side). This operation is not allowed when there
    /// are open positions.
    ///
    /// Weight: 1
    ///
    /// # Arguments
    ///
    /// * `dual_side_position` - Position mode (true for hedge mode, false for one-way mode)
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `ChangePositionModeResponse` on success.
    ///
    /// # Errors
    ///
    /// Returns `PositionModeError` if:
    /// - The position mode value is invalid
    /// - There are open positions
    /// - Authentication fails
    /// - Rate limits are exceeded
    pub async fn change_position_mode(
        &self,
        dual_side_position: PositionMode,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> PositionModeResult<ChangePositionModeResponse> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| PositionModeError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = ChangePositionModeRequest {
            dual_side_position,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| PositionModeError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::POST,
                &format!("{}/fapi/v1/positionSide/dual", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .form(&request)
            .send()
            .await
            .map_err(|e| PositionModeError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let position_mode_response: ChangePositionModeResponse = response
                .json()
                .await
                .map_err(|e| PositionModeError::Unknown(e.to_string()))?;
            Ok(position_mode_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| PositionModeError::Unknown(e.to_string()))?;
            Err(PositionModeError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_position_mode_request_serialization() {
        let request = ChangePositionModeRequest {
            dual_side_position: PositionMode::True,
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("dualSidePosition=true"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_change_position_mode_response_deserialization() {
        let json = r#"{"code":200,"msg":"success"}"#;
        let response: ChangePositionModeResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }
}
