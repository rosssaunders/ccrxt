//! Leverage bracket endpoints for Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{client::RestClient, order::OrderErrorResponse},
    signing::sign_query,
};

/// Error type for USDM leverage bracket endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum LeverageBracketError {
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

/// Request for getting leverage bracket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageBracketRequest {
    /// Trading symbol (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Timestamp in milliseconds.
    pub timestamp: u64,
    /// Signature for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Leverage bracket data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    /// Bracket level
    pub bracket: u32,
    /// Initial leverage
    pub initial_leverage: u32,
    /// Notional cap
    pub notional_cap: String,
    /// Notional floor
    pub notional_floor: String,
    /// Maintenance margin ratio
    pub maint_margin_ratio: String,
    /// Cumulative maintenance margin
    pub cum: String,
}

/// Response from leverage bracket endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracketResponse {
    /// Trading symbol
    pub symbol: String,
    /// Brackets for this symbol
    pub brackets: Vec<LeverageBracket>,
}

impl RestClient {
    /// Get notional and leverage brackets.
    pub async fn get_leverage_bracket(
        &self,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        symbol: Option<String>,
    ) -> Result<Vec<LeverageBracketResponse>, LeverageBracketError> {
        // Rate limiting for private endpoints (1 weight)
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| LeverageBracketError::Unknown(format!("Rate limiting error: {e}")))?;

        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let timestamp = Utc::now().timestamp_millis() as u64;

        let mut request = GetLeverageBracketRequest {
            symbol,
            timestamp,
            signature: None,
        };

        // Create query string for signing
        let query_string = serde_urlencoded::to_string(&request).map_err(|_| {
            LeverageBracketError::Unknown("Failed to serialize request".to_string())
        })?;

        // Sign the request
        let signature = sign_query(&query_string, &api_secret);
        request.signature = Some(signature);

        // Make the request
        let response = self
            .client
            .request(
                Method::GET,
                &format!("{}/fapi/v1/leverageBracket", self.base_url),
            )
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .query(&request)
            .send()
            .await
            .map_err(|e| LeverageBracketError::Unknown(e.to_string()))?;

        if response.status().is_success() {
            let bracket_response: Vec<LeverageBracketResponse> = response
                .json()
                .await
                .map_err(|e| LeverageBracketError::Unknown(e.to_string()))?;
            Ok(bracket_response)
        } else {
            let error_response: OrderErrorResponse = response
                .json()
                .await
                .map_err(|e| LeverageBracketError::Unknown(e.to_string()))?;
            Err(LeverageBracketError::Unknown(error_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leverage_bracket_response_deserialization() {
        let json = r#"
        [
            {
                "symbol": "BTCUSDT",
                "brackets": [
                    {
                        "bracket": 1,
                        "initialLeverage": 125,
                        "notionalCap": "50000",
                        "notionalFloor": "0",
                        "maintMarginRatio": "0.004",
                        "cum": "0.0"
                    }
                ]
            }
        ]
        "#;

        let response: Vec<LeverageBracketResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].brackets.len(), 1);
        assert_eq!(response[0].brackets[0].bracket, 1);
        assert_eq!(response[0].brackets[0].initial_leverage, 125);
    }
}
