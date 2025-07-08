//! Position risk V3 endpoints for Binance USDM REST API.

use std::borrow::Cow;

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_urlencoded;
use thiserror::Error;

use crate::binance::usdm::{enums::*, private::rest::client::RestClient, signing::sign_query};

#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum PositionRiskV3Error {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("Position risk V3 error: {0}")]
    PositionRiskV3(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionRiskV3ErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<PositionRiskV3ErrorResponse> for PositionRiskV3Error {
    fn from(e: PositionRiskV3ErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => PositionRiskV3Error::InvalidKey(e.msg),
            -1003 => PositionRiskV3Error::RateLimit(e.msg),
            _ => PositionRiskV3Error::Other(e.msg),
        }
    }
}

pub type PositionRiskV3Result<T> = std::result::Result<T, PositionRiskV3Error>;

/// Request for getting position risk info V3.
#[derive(Debug, Clone, Serialize)]
pub struct GetPositionRiskV3Request {
    #[serde(skip_serializing)]
    pub api_key: SecretString,
    #[serde(skip_serializing)]
    pub api_secret: SecretString,
    pub symbol: Option<Cow<'static, str>>,
}

/// Response for position risk info V3.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskV3 {
    pub entry_price: String,
    pub leverage: String,
    pub max_notional_value: String,
    pub liquidation_price: String,
    pub mark_price: String,
    pub position_amt: String,
    pub notional: String,
    pub isolated_wallet: String,
    pub symbol: String,
    pub un_realized_profit: String,
    pub margin_type: MarginType,
    pub isolated_margin: String,
    pub is_auto_add_margin: bool,
    pub position_side: PositionSide,
    pub break_even_price: String,
    pub adl_quantile: u8,
}

impl RestClient {
    /// Get position risk V3 (GET /fapi/v3/positionRisk)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#position-information-v3-user_data)
    pub async fn get_position_risk_v3(
        &self,
        params: GetPositionRiskV3Request,
    ) -> PositionRiskV3Result<Vec<PositionRiskV3>> {
        use tracing::debug;

        use crate::binance::usdm::request::execute_request;
        let endpoint = "/fapi/v3/positionRisk";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Serialize params to query string (excluding api_key/api_secret)
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| PositionRiskV3Error::Other(format!("Failed to serialize params: {e}")))?;
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
            .map_err(|e| PositionRiskV3Error::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending get position risk V3 request");

        // 5. Execute
        let full_url = format!("{}?{}", url, query_pairs);
        let resp = execute_request::<Vec<PositionRiskV3>>(
            &self.client,
            &full_url,
            method,
            Some(headers),
            None,
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                PositionRiskV3Error::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                PositionRiskV3Error::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => PositionRiskV3Error::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                PositionRiskV3Error::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_risk_v3_response_deserialization() {
        let json = r#"[
            {
                "entryPrice": "50000.0",
                "leverage": "10",
                "maxNotionalValue": "1000000",
                "liquidationPrice": "45000.0",
                "markPrice": "51000.0",
                "positionAmt": "0.1",
                "notional": "5100.0",
                "isolatedWallet": "510.0",
                "symbol": "BTCUSDT",
                "unRealizedProfit": "100.0",
                "marginType": "isolated",
                "isolatedMargin": "510.0",
                "isAutoAddMargin": false,
                "positionSide": "LONG",
                "breakEvenPrice": "50025.0",
                "adlQuantile": 2
            }
        ]"#;

        let result: Vec<PositionRiskV3> = serde_json::from_str(json).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].symbol, "BTCUSDT");
        assert_eq!(result[0].position_side, PositionSide::Long);
        assert_eq!(result[0].margin_type, MarginType::Isolated);
        assert_eq!(result[0].adl_quantile, 2);
    }

    #[test]
    fn test_get_position_risk_v3_request_serialization() {
        let request = GetPositionRiskV3Request {
            api_key: SecretString::new("test_key".to_string().into()),
            api_secret: SecretString::new("test_secret".to_string().into()),
            symbol: Some(Cow::Borrowed("BTCUSDT")),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
    }
}
