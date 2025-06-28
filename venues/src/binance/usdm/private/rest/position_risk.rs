//! Position risk endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use thiserror::Error;

use crate::binance::usdm::enums::*;
use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;
use serde_urlencoded;

#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum PositionRiskError {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("Position risk error: {0}")]
    PositionRisk(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionRiskErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<PositionRiskErrorResponse> for PositionRiskError {
    fn from(e: PositionRiskErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => PositionRiskError::InvalidKey(e.msg),
            -1003 => PositionRiskError::RateLimit(e.msg),
            _ => PositionRiskError::Other(e.msg),
        }
    }
}

pub type PositionRiskResult<T> = std::result::Result<T, PositionRiskError>;

/// Request for getting position risk info.
#[derive(Debug, Clone, Serialize)]
pub struct GetPositionRiskRequest {
    #[serde(skip_serializing)]
    pub api_key: SecretString,
    #[serde(skip_serializing)]
    pub api_secret: SecretString,
    pub symbol: Option<Cow<'static, str>>,
}

/// Response for position risk info.
#[derive(Debug, Clone, Deserialize)]
pub struct PositionRisk {
    pub entry_price: Cow<'static, str>,
    pub margin_type: MarginType,
    pub is_auto_add_margin: bool,
    pub isolated_margin: Cow<'static, str>,
    pub leverage: Cow<'static, str>,
    pub liquidation_price: Cow<'static, str>,
    pub mark_price: Cow<'static, str>,
    pub max_notional_value: Cow<'static, str>,
    pub position_amt: Cow<'static, str>,
    pub symbol: Cow<'static, str>,
    pub un_realized_profit: Cow<'static, str>,
    pub position_side: PositionSide,
}

impl RestClient {
    /// Get position risk (GET /fapi/v2/positionRisk)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#position-information-v2-user_data)
    pub async fn get_position_risk(
        &self,
        params: GetPositionRiskRequest,
    ) -> PositionRiskResult<Vec<PositionRisk>> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;
        let endpoint = "/fapi/v2/positionRisk";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Serialize params to query string (excluding api_key/api_secret)
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| PositionRiskError::Other(format!("Failed to serialize params: {e}")))?;
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
            .acquire_request(2)
            .await
            .map_err(|e| PositionRiskError::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending get position risk request");

        // 5. Execute
        let full_url = format!("{}?{}", url, query_pairs);
        let resp = execute_request::<Vec<PositionRisk>>(
            &self.client,
            &full_url,
            method,
            Some(headers),
            None,
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                PositionRiskError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                PositionRiskError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => PositionRiskError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                PositionRiskError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}
