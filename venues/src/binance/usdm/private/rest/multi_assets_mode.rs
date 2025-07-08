//! Change multi-assets mode on Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM multi-assets mode endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
pub enum MultiAssetsError {
    #[error("Invalid API key or signature: {0}")]
    InvalidApiKeyOrSignature(String),

    #[error("Invalid multi-assets mode: {0}")]
    InvalidMultiAssetsMode(String),

    #[error("Multi-assets mode change not allowed with open positions: {0}")]
    MultiAssetsModeChangeWithOpenPositions(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for USDM multi-assets mode operations.
pub type MultiAssetsResult<T> = Result<T, MultiAssetsError>;

/// Request to change multi-assets mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetsModeRequest {
    /// Multi-assets mode.
    /// - true: Multi-assets mode enabled
    /// - false: Multi-assets mode disabled (single-asset mode)
    pub multi_assets_margin: bool,
    /// Request timestamp in milliseconds.
    pub timestamp: u64,
    /// Request signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl Default for ChangeMultiAssetsModeRequest {
    fn default() -> Self {
        Self {
            multi_assets_margin: false,
            timestamp: Utc::now().timestamp_millis() as u64,
            signature: None,
        }
    }
}

/// Response from changing multi-assets mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetsModeResponse {
    /// Response code (200 indicates success).
    pub code: i32,
    /// Response message.
    pub msg: String,
}

impl RestClient {
    /// Change multi-assets mode.
    ///
    /// Changes the multi-assets mode for the account. When multi-assets mode is enabled,
    /// users can use multiple assets as margin. This operation is not allowed when there
    /// are open positions.
    ///
    /// Weight: 1
    ///
    /// # Arguments
    ///
    /// * `multi_assets_margin` - Multi-assets mode (true to enable, false to disable)
    /// * `api_key` - API key for authentication
    /// * `api_secret` - API secret for signing requests
    ///
    /// # Returns
    ///
    /// Returns `ChangeMultiAssetsModeResponse` on success.
    ///
    /// # Errors
    ///
    /// Returns `MultiAssetsError` if:
    /// - The multi-assets mode value is invalid
    /// - There are open positions
    /// - Authentication fails
    /// - Rate limits are exceeded
    pub async fn change_multi_assets_mode(
        &self,
        multi_assets_margin: bool,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> MultiAssetsResult<ChangeMultiAssetsModeResponse> {
        // Acquire rate limit permit
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| MultiAssetsError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = ChangeMultiAssetsModeRequest {
            multi_assets_margin,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| MultiAssetsError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::POST,
                format!("{}/fapi/v1/multiAssetsMargin", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .form(&request)
            .send()
            .await
            .map_err(|e| MultiAssetsError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let multi_assets_response: ChangeMultiAssetsModeResponse = response
                .json()
                .await
                .map_err(|e| MultiAssetsError::Unknown(e.to_string()))?;
            Ok(multi_assets_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| MultiAssetsError::Unknown(e.to_string()))?;
            Err(MultiAssetsError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_multi_assets_mode_request_serialization() {
        let request = ChangeMultiAssetsModeRequest {
            multi_assets_margin: true,
            timestamp: 1625097600000,
            signature: Some("test_signature".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("multiAssetsMargin=true"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("signature=test_signature"));
    }

    #[test]
    fn test_change_multi_assets_mode_response_deserialization() {
        let json = r#"{"code":200,"msg":"success"}"#;
        let response: ChangeMultiAssetsModeResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }
}
