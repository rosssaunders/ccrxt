use super::UsdmClient;
use crate::binance::usdm::RestResult;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Endpoint for querying rate limit order usage.
const RATE_LIMIT_ORDER_ENDPOINT: &str = "/fapi/v1/rateLimit/order";

/// Rate limit type for the order rate limit endpoint.
///
/// Valid values: "ORDERS", "REQUESTS".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    Orders,
    Requests,
}

/// Interval type for the rate limit window.
///
/// Valid values: "SECOND", "MINUTE", "HOUR", "DAY".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntervalType {
    Second,
    Minute,
    Hour,
    Day,
}

/// Request parameters for the rate limit order endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRateLimitOrderRequest {
    /// Optional. The number of milliseconds after timestamp the request is valid for.
    /// If not sent, defaults to the exchange default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Required. Timestamp in milliseconds since epoch.
    pub timestamp: u64,
}

/// Represents a single rate limit window for orders.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitOrderData {
    /// Rate limit type (e.g., "ORDERS").
    pub rate_limit_type: RateLimitType,

    /// Interval type (e.g., "SECOND", "MINUTE").
    pub interval: IntervalType,

    /// Number of intervals (e.g., 10 for 10 seconds).
    pub interval_num: u32,

    /// Maximum allowed count within the interval.
    pub limit: u32,

    /// Current count within the interval.
    pub count: u32,
}

/// Response wrapper for the rate limit order endpoint.
///
/// The API returns a direct array, so this struct is a transparent wrapper.
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct RateLimitOrderResponse {
    /// List of rate limit windows returned by the API.
    pub rate_limits: Vec<RateLimitOrderData>,
}

impl UsdmClient {
    /// Query User Rate Limit
    ///
    /// Queries the current order count usage in the current time window for the account.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Query-Rate-Limit
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `params` - The request parameters for the rate limit order endpoint.
    ///
    /// # Returns
    /// A transparent wrapper containing a list of `RateLimitOrderData`.
    pub async fn get_rate_limit_order(
        &self,
        params: GetRateLimitOrderRequest,
    ) -> RestResult<RateLimitOrderResponse> {
        self.send_signed_request(RATE_LIMIT_ORDER_ENDPOINT, Method::GET, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_order_data_deserialization() {
        let json = r#"
        {
            "rateLimitType": "ORDERS",
            "interval": "SECOND",
            "intervalNum": 10,
            "limit": 50,
            "count": 0
        }
        "#;
        let data: RateLimitOrderData = serde_json::from_str(json).unwrap();
        assert_eq!(data.rate_limit_type, RateLimitType::Orders);
        assert_eq!(data.interval, IntervalType::Second);
        assert_eq!(data.interval_num, 10);
        assert_eq!(data.limit, 50);
        assert_eq!(data.count, 0);
    }

    #[test]
    fn test_rate_limit_order_response_deserialization() {
        let json = r#"
        [
            {
                "rateLimitType": "ORDERS",
                "interval": "SECOND",
                "intervalNum": 10,
                "limit": 50,
                "count": 0
            },
            {
                "rateLimitType": "ORDERS",
                "interval": "DAY",
                "intervalNum": 1,
                "limit": 160000,
                "count": 0
            }
        ]
        "#;
        let response: Vec<RateLimitOrderData> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0].rate_limit_type, RateLimitType::Orders);
        assert_eq!(response[0].interval, IntervalType::Second);
        assert_eq!(response[0].limit, 50);
    }

    #[test]
    fn test_get_rate_limit_order_request_default() {
        let req = GetRateLimitOrderRequest::default();
        assert_eq!(req.recv_window, None);
    }
}
