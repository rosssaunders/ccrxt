//! Rate limit order endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::private::rest::order::OrderErrorResponse;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;

/// Error type for USDM rate limit order endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum RateLimitOrderError {
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

/// Request for getting rate limit order count.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRateLimitOrderRequest {
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Rate limit data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitOrderData {
    /// Rate limit type
    pub rate_limit_type: String,
    /// Interval for the rate limit
    pub interval: String,
    /// Interval unit (e.g., "MINUTE")
    pub interval_num: u32,
    /// Maximum allowed count within the interval
    pub limit: u32,
    /// Current count within the interval
    pub count: u32,
}

/// Response from rate limit order endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitOrderResponse {
    /// List of rate limit data
    pub rate_limits: Vec<RateLimitOrderData>,
}

impl RestClient {
    /// Query current order count usage in current time window.
    pub async fn get_rate_limit_order(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> Result<Vec<RateLimitOrderData>, RateLimitOrderError> {
        // Rate limiting for private endpoints (20 weight)
        self.rate_limiter
            .acquire_request(20)
            .await
            .map_err(|e| RateLimitOrderError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetRateLimitOrderRequest {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request)
            .map_err(|_| RateLimitOrderError::Unknown("Failed to serialize request".to_string()))?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(Method::GET, &format!("{}/fapi/v1/rateLimit/order", self.base_url))
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| RateLimitOrderError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let rate_limit_response: Vec<RateLimitOrderData> = response
                .json()
                .await
                .map_err(|e| RateLimitOrderError::Unknown(e.to_string()))?;
            Ok(rate_limit_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| RateLimitOrderError::Unknown(e.to_string()))?;
            Err(RateLimitOrderError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_order_response_deserialization() {
        let json = r#"
        [
            {
                "rateLimitType": "ORDERS",
                "interval": "SECOND",
                "intervalNum": 10,
                "limit": 50,
                "count": 0
            },
            {
                "rateLimitType": "ORDERS",
                "interval": "DAY",
                "intervalNum": 1,
                "limit": 160000,
                "count": 0
            }
        ]
        "#;

        let response: Vec<RateLimitOrderData> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0].rate_limit_type, "ORDERS");
        assert_eq!(response[0].interval, "SECOND");
        assert_eq!(response[0].limit, 50);
    }
}
