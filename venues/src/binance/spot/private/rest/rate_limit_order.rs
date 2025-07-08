use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

/// Request parameters for rate limit order query
#[derive(Debug, Clone, Serialize, Default)]
pub struct RateLimitOrderRequest {
    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Rate limit order response
#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitOrderResponse {
    /// Rate limit type
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,

    /// Interval
    #[serde(rename = "interval")]
    pub interval: String,

    /// Interval number
    #[serde(rename = "intervalNum")]
    pub interval_num: u32,

    /// Limit
    #[serde(rename = "limit")]
    pub limit: u32,

    /// Count
    #[serde(rename = "count")]
    pub count: u32,
}

impl RestClient {
    /// Display user's unfilled order count for all intervals
    ///
    /// Display user's unfilled order count for all intervals.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-unfilled-order-count--user_data)
    /// Method: GET /api/v3/rateLimit/order
    /// Weight: 40
    /// Security: USER_DATA
    pub async fn get_rate_limit_order(
        &self,
        params: Option<RateLimitOrderRequest>,
    ) -> RestResult<Vec<RateLimitOrderResponse>> {
        let query_string = if let Some(p) = params {
            if p.recv_window.is_some() {
                Some(serde_urlencoded::to_string(&p).map_err(|e| {
                    crate::binance::spot::Errors::Error(format!("URL encoding error: {e}"))
                })?)
            } else {
                None
            }
        } else {
            None
        };

        self.send_request(
            "/api/v3/rateLimit/order",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            40,
            false,
        )
        .await
    }
}
