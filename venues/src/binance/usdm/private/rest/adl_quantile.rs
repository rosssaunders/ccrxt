//! ADL quantile endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use thiserror::Error;


use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;
use serde_urlencoded;

#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum AdlQuantileError {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("ADL quantile error: {0}")]
    AdlQuantile(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdlQuantileErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<AdlQuantileErrorResponse> for AdlQuantileError {
    fn from(e: AdlQuantileErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => AdlQuantileError::InvalidKey(e.msg),
            -1003 => AdlQuantileError::RateLimit(e.msg),
            _ => AdlQuantileError::Other(e.msg),
        }
    }
}

pub type AdlQuantileResult<T> = std::result::Result<T, AdlQuantileError>;

/// Request for getting ADL quantile estimation.
#[derive(Debug, Clone, Serialize)]
pub struct GetAdlQuantileRequest {
    #[serde(skip_serializing)]
    pub api_key: SecretString,
    #[serde(skip_serializing)]
    pub api_secret: SecretString,
    pub symbol: Option<Cow<'static, str>>,
}

/// ADL quantile values for different position sides.
#[derive(Debug, Clone, Deserialize)]
pub struct AdlQuantileValues {
    /// ADL quantile for LONG position in hedge mode or position in one-way mode.
    #[serde(rename = "LONG")]
    pub long: Option<u8>,
    /// ADL quantile for SHORT position in hedge mode.
    #[serde(rename = "SHORT")]
    pub short: Option<u8>,
    /// ADL quantile for position in one-way mode.
    #[serde(rename = "BOTH")]
    pub both: Option<u8>,
    /// Sign for hedge mode (only a sign, ignore the value).
    #[serde(rename = "HEDGE")]
    pub hedge: Option<u8>,
}

/// Response for ADL quantile estimation.
#[derive(Debug, Clone, Deserialize)]
pub struct AdlQuantileResponse {
    pub symbol: String,
    #[serde(rename = "adlQuantile")]
    pub adl_quantile: AdlQuantileValues,
}

impl RestClient {
    /// Get ADL quantile estimation (GET /fapi/v1/adlQuantile)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#position-adl-quantile-estimation-user_data)
    pub async fn get_adl_quantile(
        &self,
        params: GetAdlQuantileRequest,
    ) -> AdlQuantileResult<Vec<AdlQuantileResponse>> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;
        let endpoint = "/fapi/v1/adlQuantile";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Serialize params to query string (excluding api_key/api_secret)
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| AdlQuantileError::Other(format!("Failed to serialize params: {e}")))?;
        if !query_pairs.is_empty() {
            query_pairs.push('&');
        }
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;
        query_pairs.push_str(&format!("timestamp={timestamp}&recvWindow={recv_window}"));

        // 2. Sign
        let signature = sign_query(&query_pairs, &params.api_secret);
        query_pairs.push_str(&format!("&signature={signature}"));

        // 3. Headers
        let headers = vec![("X-MBX-APIKEY", params.api_key.expose_secret().to_string())];

        // 4. Rate limiting
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| AdlQuantileError::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending get ADL quantile request");

        // 5. Execute
        let full_url = format!("{}?{}", url, query_pairs);
        let resp = execute_request::<Vec<AdlQuantileResponse>>(
            &self.client,
            &full_url,
            method,
            Some(headers),
            None,
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                AdlQuantileError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                AdlQuantileError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => AdlQuantileError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                AdlQuantileError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adl_quantile_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "adlQuantile": {
                    "LONG": 3,
                    "SHORT": 2,
                    "BOTH": null,
                    "HEDGE": null
                }
            }
        ]"#;

        let result: Vec<AdlQuantileResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].symbol, "BTCUSDT");
        assert_eq!(result[0].adl_quantile.long, Some(3));
        assert_eq!(result[0].adl_quantile.short, Some(2));
        assert_eq!(result[0].adl_quantile.both, None);
        assert_eq!(result[0].adl_quantile.hedge, None);
    }

    #[test]
    fn test_get_adl_quantile_request_serialization() {
        let request = GetAdlQuantileRequest {
            api_key: SecretString::new("test_key".to_string().into()),
            api_secret: SecretString::new("test_secret".to_string().into()),
            symbol: Some(Cow::Borrowed("BTCUSDT")),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
    }

    #[test]
    fn test_adl_quantile_values_deserialization() {
        let json = r#"{
            "LONG": 1,
            "SHORT": 4,
            "BOTH": null,
            "HEDGE": null
        }"#;

        let values: AdlQuantileValues = serde_json::from_str(json).unwrap();
        assert_eq!(values.long, Some(1));
        assert_eq!(values.short, Some(4));
        assert_eq!(values.both, None);
        assert_eq!(values.hedge, None);
    }
}
