//! Query order endpoints for Binance USDM REST API.

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
pub enum QueryOrderError {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("Query order error: {0}")]
    QueryOrder(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrderErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<QueryOrderErrorResponse> for QueryOrderError {
    fn from(e: QueryOrderErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => QueryOrderError::InvalidKey(e.msg),
            -1003 => QueryOrderError::RateLimit(e.msg),
            _ => QueryOrderError::Other(e.msg),
        }
    }
}

pub type QueryOrderResult<T> = std::result::Result<T, QueryOrderError>;

/// Request for querying an order.
#[derive(Debug, Clone, Serialize)]
pub struct QueryOrderRequest {
    #[serde(skip_serializing)]
    pub api_key: SecretString,
    #[serde(skip_serializing)]
    pub api_secret: SecretString,
    pub symbol: Cow<'static, str>,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<Cow<'static, str>>,
}

/// Response for a queried order.
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrderResponse {
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
    /// Query order (GET /fapi/v1/order)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#query-order-user_data)
    pub async fn query_order(
        &self,
        params: QueryOrderRequest,
    ) -> QueryOrderResult<QueryOrderResponse> {
        use tracing::debug;

        use crate::binance::usdm::request::execute_request;
        let endpoint = "/fapi/v1/order";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Serialize params to query string (excluding api_key/api_secret)
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| QueryOrderError::Other(format!("Failed to serialize params: {e}")))?;
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
            .map_err(|e| QueryOrderError::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending query order request");

        // 5. Execute
        let full_url = format!("{}?{}", url, query_pairs);
        let resp = execute_request::<QueryOrderResponse>(
            &self.client,
            &full_url,
            method,
            Some(headers),
            None,
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                QueryOrderError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                QueryOrderError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => QueryOrderError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                QueryOrderError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}
