//! Place and manage batch orders on Binance USDM REST API.

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{
    private::rest::{
        client::RestClient,
        order::{NewOrderRequest, NewOrderResponse},
    },
    signing::sign_query,
};

/// Error type for USDM batch order endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum BatchOrderError {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("Batch order error: {0}")]
    BatchOrder(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchOrderErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<BatchOrderErrorResponse> for BatchOrderError {
    fn from(e: BatchOrderErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => BatchOrderError::InvalidKey(e.msg),
            -1003 => BatchOrderError::RateLimit(e.msg),
            _ => BatchOrderError::Other(e.msg),
        }
    }
}

pub type BatchOrderResult<T> = std::result::Result<T, BatchOrderError>;

/// Request parameters for placing batch orders.
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrderRequest {
    /// API key (SecretString, securely stored)
    #[serde(skip_serializing)]
    pub api_key: SecretString,

    /// API secret (SecretString, securely stored)
    #[serde(skip_serializing)]
    pub api_secret: SecretString,

    /// List of orders to place
    pub batch_orders: Vec<NewOrderRequest>,
}

/// Response for a batch order request.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchOrderResponse {
    /// List of order responses
    pub orders: Vec<NewOrderResponse>,
}

impl RestClient {
    /// Place multiple orders (POST /fapi/v1/batchOrders)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#place-multiple-orders-trade)
    pub async fn place_batch_orders(
        &self,
        params: BatchOrderRequest,
    ) -> BatchOrderResult<BatchOrderResponse> {
        use tracing::debug;

        use crate::binance::usdm::request::execute_request;
        let endpoint = "/fapi/v1/batchOrders";
        let method = Method::POST;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Prepare batchOrders as JSON string (Binance expects this as a stringified array)
        let batch_orders_json = serde_json::to_string(&params.batch_orders).map_err(|e| {
            BatchOrderError::Other(format!("Failed to serialize batch_orders: {e}"))
        })?;

        // 2. Build query string
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;
        let mut query_pairs = format!(
            "batchOrders={}&timestamp={}&recvWindow={}",
            urlencoding::encode(&batch_orders_json),
            timestamp,
            recv_window
        );

        // 3. Sign
        let signature = sign_query(&query_pairs, &params.api_secret);
        query_pairs.push_str(&format!("&signature={signature}"));

        // 4. Headers
        let headers = vec![
            ("X-MBX-APIKEY", params.api_key.expose_secret().to_string()),
            (
                "Content-Type",
                "application/x-www-form-urlencoded".to_string(),
            ),
        ];

        // 5. Rate limiting
        self.rate_limiter
            .acquire_order()
            .await
            .map_err(|e| BatchOrderError::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending batch order request");

        // 6. Execute
        let resp = execute_request::<BatchOrderResponse>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                BatchOrderError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                BatchOrderError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => BatchOrderError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                BatchOrderError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}
