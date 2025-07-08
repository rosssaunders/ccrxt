//! Multi-assets margin status endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM multi-assets margin status endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum MultiAssetsMarginStatusError {
    /// Invalid API key or signature.
    #[error("Invalid API key or signature: {0}")]
    #[serde(rename = "-1022")]
    InvalidSignature(String),
    /// Timestamp for this request is outside of the recv window.
    #[error("Timestamp for this request is outside of the recv window: {0}")]
    #[serde(rename = "-1021")]
    TimestampOutOfRecvWindow(String),
    /// Invalid API key format.
    #[error("Invalid API key format: {0}")]
    #[serde(rename = "-2014")]
    BadApiKeyFmt(String),
    /// Invalid API key, IP, or permissions for action.
    #[error("Invalid API key, IP, or permissions for action: {0}")]
    #[serde(rename = "-2015")]
    RejectedMbxKey(String),
    /// Unknown error.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Request for getting multi-assets margin status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMultiAssetsMarginStatusRequest {
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Response from multi-assets margin status endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiAssetsMarginStatusResponse {
    /// Whether multi-assets mode is enabled
    pub multi_assets_margin: bool,
}

impl RestClient {
    /// Get current multi-assets mode status.
    pub async fn get_multi_assets_margin_status(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> Result<MultiAssetsMarginStatusResponse, MultiAssetsMarginStatusError> {
        // Rate limiting for private endpoints (30 weight)
        self.rate_limiter.acquire_request(30).await.map_err(|e| {
            MultiAssetsMarginStatusError::Unknown(format!("Rate limiting error: {e}"))
        })?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetMultiAssetsMarginStatusRequest {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request).map_err(|_| {
            MultiAssetsMarginStatusError::Unknown("Failed to serialize request".to_string())
        })?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::GET,
                &format!("{}/fapi/v1/multiAssetsMargin", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| MultiAssetsMarginStatusError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let status_response: MultiAssetsMarginStatusResponse = response
                .json()
                .await
                .map_err(|e| MultiAssetsMarginStatusError::Unknown(e.to_string()))?;
            Ok(status_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| MultiAssetsMarginStatusError::Unknown(e.to_string()))?;
            Err(MultiAssetsMarginStatusError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_assets_margin_status_response_deserialization() {
        let json = r#"
        {
            "multiAssetsMargin": true
        }
        "#;

        let response: MultiAssetsMarginStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.multi_assets_margin);
    }
}
