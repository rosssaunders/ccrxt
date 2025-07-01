//! Position margin change history endpoints for Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Deserializer, Serialize};
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
pub enum PositionMarginHistoryError {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("Position margin history error: {0}")]
    PositionMarginHistory(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionMarginHistoryErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<PositionMarginHistoryErrorResponse> for PositionMarginHistoryError {
    fn from(e: PositionMarginHistoryErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => PositionMarginHistoryError::InvalidKey(e.msg),
            -1003 => PositionMarginHistoryError::RateLimit(e.msg),
            _ => PositionMarginHistoryError::Other(e.msg),
        }
    }
}

pub type PositionMarginHistoryResult<T> = std::result::Result<T, PositionMarginHistoryError>;

/// Margin modification type for position margin history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginModificationType {
    Add = 1,
    Reduce = 2,
}

impl Serialize for MarginModificationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MarginModificationType::Add => serializer.serialize_u8(1),
            MarginModificationType::Reduce => serializer.serialize_u8(2),
        }
    }
}

impl<'de> Deserialize<'de> for MarginModificationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            1 => Ok(MarginModificationType::Add),
            2 => Ok(MarginModificationType::Reduce),
            other => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(other as u64),
                &"1 or 2",
            )),
        }
    }
}

/// Request for getting position margin change history.
#[derive(Debug, Clone, Serialize)]
pub struct GetPositionMarginHistoryRequest {
    #[serde(skip_serializing)]
    pub api_key: SecretString,
    #[serde(skip_serializing)]
    pub api_secret: SecretString,
    pub symbol: Cow<'static, str>,
    #[serde(rename = "type")]
    pub modification_type: Option<MarginModificationType>,
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
}

/// Position margin change history entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionMarginHistoryEntry {
    pub amount: String,
    pub asset: String,
    pub symbol: String,
    pub time: u64,
    #[serde(rename = "type")]
    pub modification_type: MarginModificationType,
    pub position_side: PositionSide,
}

impl RestClient {
    /// Get position margin change history (GET /fapi/v1/positionMargin/history)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#get-position-margin-change-history-trade)
    pub async fn get_position_margin_history(
        &self,
        params: GetPositionMarginHistoryRequest,
    ) -> PositionMarginHistoryResult<Vec<PositionMarginHistoryEntry>> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;
        let endpoint = "/fapi/v1/positionMargin/history";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Serialize params to query string (excluding api_key/api_secret)
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| PositionMarginHistoryError::Other(format!("Failed to serialize params: {e}")))?;
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
            .acquire_request(1)
            .await
            .map_err(|e| PositionMarginHistoryError::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending get position margin history request");

        // 5. Execute
        let full_url = format!("{}?{}", url, query_pairs);
        let resp = execute_request::<Vec<PositionMarginHistoryEntry>>(
            &self.client,
            &full_url,
            method,
            Some(headers),
            None,
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                PositionMarginHistoryError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                PositionMarginHistoryError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => PositionMarginHistoryError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                PositionMarginHistoryError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_margin_history_response_deserialization() {
        let json = r#"[
            {
                "amount": "100.00000000",
                "asset": "USDT",
                "symbol": "BTCUSDT",
                "time": 1578047897183,
                "type": 1,
                "positionSide": "LONG"
            },
            {
                "amount": "50.00000000",
                "asset": "USDT",
                "symbol": "BTCUSDT",
                "time": 1578047897184,
                "type": 2,
                "positionSide": "SHORT"
            }
        ]"#;

        let result: Vec<PositionMarginHistoryEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].symbol, "BTCUSDT");
        assert_eq!(result[0].amount, "100.00000000");
        assert_eq!(result[0].asset, "USDT");
        assert_eq!(result[0].position_side, PositionSide::Long);
        assert!(matches!(result[0].modification_type, MarginModificationType::Add));
        assert!(matches!(result[1].modification_type, MarginModificationType::Reduce));
    }

    #[test]
    fn test_get_position_margin_history_request_serialization() {
        let request = GetPositionMarginHistoryRequest {
            api_key: SecretString::new("test_key".to_string().into()),
            api_secret: SecretString::new("test_secret".to_string().into()),
            symbol: Cow::Borrowed("BTCUSDT"),
            modification_type: Some(MarginModificationType::Add),
            start_time: Some(1578047897183),
            end_time: Some(1578047897184),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("type=1"));
        assert!(serialized.contains("startTime=1578047897183"));
        assert!(serialized.contains("endTime=1578047897184"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_margin_modification_type_serialization() {
        let add_type = MarginModificationType::Add;
        let reduce_type = MarginModificationType::Reduce;

        let add_json = serde_json::to_string(&add_type).unwrap();
        let reduce_json = serde_json::to_string(&reduce_type).unwrap();

        assert_eq!(add_json, "1");
        assert_eq!(reduce_json, "2");
    }
}
