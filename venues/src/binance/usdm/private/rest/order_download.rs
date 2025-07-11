//! Order download async endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM order download endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum OrderDownloadError {
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

/// Request for getting order download ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadIdRequest {
    /// Start time in milliseconds.
    pub start_time: u64,
    /// End time in milliseconds.
    pub end_time: u64,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Response from order download ID endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDownloadIdResponse {
    /// Average time taken for data download in the past 30 days
    pub avg_cost_timestamp: String,
    /// Download ID
    pub download_id: String,
}

/// Request for getting order download link.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDownloadLinkRequest {
    /// Download ID from the previous request.
    pub download_id: String,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Download status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "failed")]
    Failed,
}

/// Response from order download link endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDownloadLinkResponse {
    /// Download link (available when status is completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_link: Option<String>,
    /// Download status
    pub status: DownloadStatus,
}

impl RestClient {
    /// Get download ID for order history.
    pub async fn get_order_download_id(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        start_time: u64,
        end_time: u64,
    ) -> Result<OrderDownloadIdResponse, OrderDownloadError> {
        // Rate limiting for private endpoints (5 weight)
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| OrderDownloadError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetOrderDownloadIdRequest {
            start_time,
            end_time,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| OrderDownloadError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, format!("{}/fapi/v1/order/asyn", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| OrderDownloadError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let download_response: OrderDownloadIdResponse = response
                .json()
                .await
                .map_err(|e| OrderDownloadError::Unknown(e.to_string()))?;
            Ok(download_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| OrderDownloadError::Unknown(e.to_string()))?;
            Err(OrderDownloadError::Unknown(error_response.msg))
        }
    }

    /// Get download link for order history.
    pub async fn get_order_download_link(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        download_id: String,
    ) -> Result<OrderDownloadLinkResponse, OrderDownloadError> {
        // Rate limiting for private endpoints (5 weight)
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| OrderDownloadError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetOrderDownloadLinkRequest {
            download_id,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| OrderDownloadError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::GET,
                format!("{}/fapi/v1/order/asyn/id", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| OrderDownloadError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let link_response: OrderDownloadLinkResponse = response
                .json()
                .await
                .map_err(|e| OrderDownloadError::Unknown(e.to_string()))?;
            Ok(link_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| OrderDownloadError::Unknown(e.to_string()))?;
            Err(OrderDownloadError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_download_id_response_deserialization() {
        let json = r#"
        {
            "avgCostTimestamp": "946684800000",
            "downloadId": "download123456"
        }
        "#;

        let response: OrderDownloadIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_cost_timestamp, "946684800000");
        assert_eq!(response.download_id, "download123456");
    }

    #[test]
    fn test_order_download_link_response_deserialization() {
        let json = r#"
        {
            "downloadLink": "https://bin-prod-user-rebate-bucket.s3.amazonaws.com/...",
            "status": "completed"
        }
        "#;

        let response: OrderDownloadLinkResponse = serde_json::from_str(json).unwrap();
        assert!(response.download_link.is_some());
        assert_eq!(
            response.download_link.unwrap(),
            "https://bin-prod-user-rebate-bucket.s3.amazonaws.com/..."
        );
        matches!(response.status, DownloadStatus::Completed);
    }
}
