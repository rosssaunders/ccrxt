//! API trading status endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    enums::*,
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM API trading status endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum ApiTradingStatusError {
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

/// Request for getting API trading status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiTradingStatusRequest {
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Indicator representing trading function status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingStatusIndicator {
    /// Whether the function is locked
    pub is_locked: bool,
    /// Planned recovery time (0 if no recovery is planned)
    pub planned_recover_time: u64,
    /// Trigger condition that caused the lock
    pub trigger_condition: TriggerCondition,
    /// Update time of this status
    pub update_time: u64,
}

/// Response from the API trading status endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTradingStatusResponse {
    /// Trading status for the futures API
    pub is_locked: bool,
    /// Planned recovery time (0 if no recovery is planned)
    pub planned_recover_time: u64,
    /// Trigger condition that caused the lock
    pub trigger_condition: TriggerCondition,
    /// Update time of this status
    pub update_time: u64,
    /// Trading status indicators for various trading functions
    pub indicators: Vec<TradingStatusIndicator>,
}

impl RestClient {
    /// Get account API trading status.
    pub async fn get_api_trading_status(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
    ) -> Result<ApiTradingStatusResponse, ApiTradingStatusError> {
        // Rate limiting for private endpoints (5 weight)
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| ApiTradingStatusError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetApiTradingStatusRequest {
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request).map_err(|_| {
            ApiTradingStatusError::Unknown("Failed to serialize request".to_string())
        })?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::GET,
                &format!("{}/fapi/v1/apiTradingStatus", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| ApiTradingStatusError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let trading_status_response: ApiTradingStatusResponse = response
                .json()
                .await
                .map_err(|e| ApiTradingStatusError::Unknown(e.to_string()))?;
            Ok(trading_status_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| ApiTradingStatusError::Unknown(e.to_string()))?;
            Err(ApiTradingStatusError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_trading_status_response_deserialization() {
        let json = r#"
        {
            "isLocked": false,
            "plannedRecoverTime": 0,
            "triggerCondition": {
                "GCR": 150,
                "IFER": 150,
                "UFR": 300
            },
            "updateTime": 1547630471725,
            "indicators": [
                {
                    "isLocked": false,
                    "plannedRecoverTime": 0,
                    "triggerCondition": {
                        "GCR": 150,
                        "IFER": 150,
                        "UFR": 300
                    },
                    "updateTime": 1547630471725
                }
            ]
        }
        "#;

        let response: ApiTradingStatusResponse = serde_json::from_str(json).unwrap();
        assert!(!response.is_locked);
        assert_eq!(response.planned_recover_time, 0);
        assert_eq!(response.update_time, 1547630471725);
        assert_eq!(response.indicators.len(), 1);
        assert!(!response.indicators[0].is_locked);
    }
}
