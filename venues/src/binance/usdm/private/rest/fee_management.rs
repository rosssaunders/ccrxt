//! Fee management endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM fee management endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum FeeManagementError {
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

/// Request for toggling BNB burn status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFeeBurnRequest {
    /// Whether to enable BNB burn for fee
    pub fee_burn: bool,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Response from toggle fee burn endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFeeBurnResponse {
    /// Response code
    pub code: i32,
    /// Response message
    pub msg: String,
}

/// Request for getting BNB burn status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeBurnStatusRequest {
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Response from get fee burn status endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeBurnStatusResponse {
    /// Whether BNB burn is enabled
    pub fee_burn: bool,
}

impl RestClient {
    /// Toggle BNB burn on futures trading and margin interest.
    pub async fn toggle_fee_burn(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        fee_burn: bool,
    ) -> Result<ToggleFeeBurnResponse, FeeManagementError> {
        // Rate limiting for private endpoints (1 weight)
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| FeeManagementError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = ToggleFeeBurnRequest {
            fee_burn,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| FeeManagementError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::POST, &format!("{}/fapi/v1/feeBurn", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| FeeManagementError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let burn_response: ToggleFeeBurnResponse = response
                .json()
                .await
                .map_err(|e| FeeManagementError::Unknown(e.to_string()))?;
            Ok(burn_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| FeeManagementError::Unknown(e.to_string()))?;
            Err(FeeManagementError::Unknown(error_response.msg))
        }
    }

    /// Get BNB burn status.
    pub async fn get_fee_burn_status(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> Result<FeeBurnStatusResponse, FeeManagementError> {
        // Rate limiting for private endpoints (1 weight)
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| FeeManagementError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetFeeBurnStatusRequest {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| FeeManagementError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, &format!("{}/fapi/v1/feeBurn", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| FeeManagementError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let status_response: FeeBurnStatusResponse = response
                .json()
                .await
                .map_err(|e| FeeManagementError::Unknown(e.to_string()))?;
            Ok(status_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| FeeManagementError::Unknown(e.to_string()))?;
            Err(FeeManagementError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_fee_burn_response_deserialization() {
        let json = r#"
        {
            "code": 200,
            "msg": "success"
        }
        "#;

        let response: ToggleFeeBurnResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }

    #[test]
    fn test_fee_burn_status_response_deserialization() {
        let json = r#"
        {
            "feeBurn": true
        }
        "#;

        let response: FeeBurnStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.fee_burn);
    }
}
