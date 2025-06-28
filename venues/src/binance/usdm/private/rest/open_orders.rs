//! Get, query, and cancel open orders on Binance USDM REST API.

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
pub enum OpenOrdersError {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("Open orders error: {0}")]
    OpenOrders(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrdersErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<OpenOrdersErrorResponse> for OpenOrdersError {
    fn from(e: OpenOrdersErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => OpenOrdersError::InvalidKey(e.msg),
            -1003 => OpenOrdersError::RateLimit(e.msg),
            _ => OpenOrdersError::Other(e.msg),
        }
    }
}

pub type OpenOrdersResult<T> = std::result::Result<T, OpenOrdersError>;

/// Request for getting all open orders.
#[derive(Debug, Clone, Serialize)]
pub struct GetOpenOrdersRequest {
    #[serde(skip_serializing)]
    pub api_key: SecretString,
    #[serde(skip_serializing)]
    pub api_secret: SecretString,
    pub symbol: Option<Cow<'static, str>>,
}

/// Response for open orders.
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrder {
    pub symbol: Cow<'static, str>,
    pub order_id: u64,
    pub client_order_id: Cow<'static, str>,
    pub price: Cow<'static, str>,
    pub orig_qty: Cow<'static, str>,
    pub executed_qty: Cow<'static, str>,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub working_type: WorkingType,
}

impl RestClient {
    /// Get all open orders (GET /fapi/v1/openOrders)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#current-all-open-orders-user_data)
    pub async fn get_open_orders(
        &self,
        params: GetOpenOrdersRequest,
    ) -> OpenOrdersResult<Vec<OpenOrder>> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;
        let endpoint = "/fapi/v1/openOrders";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Serialize params to query string (excluding api_key/api_secret)
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| OpenOrdersError::Other(format!("Failed to serialize params: {e}")))?;
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
            .map_err(|e| OpenOrdersError::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending get open orders request");

        // 5. Execute
        let full_url = format!("{}?{}", url, query_pairs);
        let resp =
            execute_request::<Vec<OpenOrder>>(&self.client, &full_url, method, Some(headers), None)
                .await
                .map_err(|e| match e {
                    crate::binance::usdm::Errors::ApiError(api_err) => {
                        OpenOrdersError::Other(format!("API error: {api_err}"))
                    }
                    crate::binance::usdm::Errors::HttpError(http_err) => {
                        OpenOrdersError::Other(format!("HTTP error: {http_err}"))
                    }
                    crate::binance::usdm::Errors::Error(msg) => OpenOrdersError::Other(msg),
                    crate::binance::usdm::Errors::InvalidApiKey() => {
                        OpenOrdersError::InvalidKey("Invalid API key or signature".to_string())
                    }
                })?;

        Ok(resp.data)
    }
}
