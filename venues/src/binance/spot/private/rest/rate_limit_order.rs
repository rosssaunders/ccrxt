use serde::{Deserialize, Serialize};

use crate::binance::spot::PrivateRestClient as RestClient;
use crate::binance::spot::RestResult;

const GET_RATE_LIMIT_ORDER_ENDPOINT: &str = "/api/v3/rateLimit/order";

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
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-unfilled-order-count--user_data)
    ///
    /// Method: GET /api/v3/rateLimit/order
    /// Weight: 40
    /// Security: USER_DATA
    pub async fn get_rate_limit_order(
        &self,
        params: Option<RateLimitOrderRequest>,
    ) -> RestResult<Vec<RateLimitOrderResponse>> {
        self.send_get_signed_request(
            GET_RATE_LIMIT_ORDER_ENDPOINT,
            params.unwrap_or_default(),
            40,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_order_request_serialization_default() {
        let request = RateLimitOrderRequest::default();

        // Default should serialize to empty string (no fields)
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_rate_limit_order_request_serialization_with_recv_window() {
        let request = RateLimitOrderRequest {
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "recvWindow=5000");
    }

    #[test]
    fn test_rate_limit_order_request_serialization_none_recv_window() {
        let request = RateLimitOrderRequest { recv_window: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_rate_limit_order_response_deserialization_orders_limit() {
        let json = r#"{
            "rateLimitType": "ORDERS",
            "interval": "SECOND",
            "intervalNum": 10,
            "limit": 50,
            "count": 5
        }"#;

        let response: RateLimitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.rate_limit_type, "ORDERS");
        assert_eq!(response.interval, "SECOND");
        assert_eq!(response.interval_num, 10);
        assert_eq!(response.limit, 50);
        assert_eq!(response.count, 5);
    }

    #[test]
    fn test_rate_limit_order_response_deserialization_orders_minute() {
        let json = r#"{
            "rateLimitType": "ORDERS",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 1200,
            "count": 150
        }"#;

        let response: RateLimitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.rate_limit_type, "ORDERS");
        assert_eq!(response.interval, "MINUTE");
        assert_eq!(response.interval_num, 1);
        assert_eq!(response.limit, 1200);
        assert_eq!(response.count, 150);
    }

    #[test]
    fn test_rate_limit_order_response_deserialization_orders_hour() {
        let json = r#"{
            "rateLimitType": "ORDERS",
            "interval": "HOUR",
            "intervalNum": 24,
            "limit": 200000,
            "count": 12580
        }"#;

        let response: RateLimitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.rate_limit_type, "ORDERS");
        assert_eq!(response.interval, "HOUR");
        assert_eq!(response.interval_num, 24);
        assert_eq!(response.limit, 200000);
        assert_eq!(response.count, 12580);
    }

    #[test]
    fn test_rate_limit_order_response_deserialization_request_weight() {
        let json = r#"{
            "rateLimitType": "REQUEST_WEIGHT",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 6000,
            "count": 1250
        }"#;

        let response: RateLimitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.rate_limit_type, "REQUEST_WEIGHT");
        assert_eq!(response.interval, "MINUTE");
        assert_eq!(response.interval_num, 1);
        assert_eq!(response.limit, 6000);
        assert_eq!(response.count, 1250);
    }

    #[test]
    fn test_rate_limit_order_response_deserialization_raw_requests() {
        let json = r#"{
            "rateLimitType": "RAW_REQUESTS",
            "interval": "MINUTE",
            "intervalNum": 5,
            "limit": 61000,
            "count": 15000
        }"#;

        let response: RateLimitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.rate_limit_type, "RAW_REQUESTS");
        assert_eq!(response.interval, "MINUTE");
        assert_eq!(response.interval_num, 5);
        assert_eq!(response.limit, 61000);
        assert_eq!(response.count, 15000);
    }

    #[test]
    fn test_rate_limit_order_response_array_deserialization_empty() {
        let json = "[]";
        let responses: Vec<RateLimitOrderResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 0);
    }

    #[test]
    fn test_rate_limit_order_response_array_deserialization_single() {
        let json = r#"[
            {
                "rateLimitType": "ORDERS",
                "interval": "DAY",
                "intervalNum": 1,
                "limit": 200000,
                "count": 50000
            }
        ]"#;

        let responses: Vec<RateLimitOrderResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 1);
        assert_eq!(responses[0].rate_limit_type, "ORDERS");
        assert_eq!(responses[0].interval, "DAY");
        assert_eq!(responses[0].interval_num, 1);
        assert_eq!(responses[0].limit, 200000);
        assert_eq!(responses[0].count, 50000);
    }

    #[test]
    fn test_rate_limit_order_response_array_deserialization_multiple() {
        let json = r#"[
            {
                "rateLimitType": "ORDERS",
                "interval": "SECOND",
                "intervalNum": 10,
                "limit": 50,
                "count": 5
            },
            {
                "rateLimitType": "ORDERS",
                "interval": "DAY",
                "intervalNum": 1,
                "limit": 200000,
                "count": 12580
            }
        ]"#;

        let responses: Vec<RateLimitOrderResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 2);

        // Check first response
        assert_eq!(responses[0].rate_limit_type, "ORDERS");
        assert_eq!(responses[0].interval, "SECOND");
        assert_eq!(responses[0].interval_num, 10);
        assert_eq!(responses[0].limit, 50);
        assert_eq!(responses[0].count, 5);

        // Check second response
        assert_eq!(responses[1].rate_limit_type, "ORDERS");
        assert_eq!(responses[1].interval, "DAY");
        assert_eq!(responses[1].interval_num, 1);
        assert_eq!(responses[1].limit, 200000);
        assert_eq!(responses[1].count, 12580);
    }

    #[test]
    fn test_rate_limit_order_response_array_deserialization_all_intervals() {
        let json = r#"[
            {
                "rateLimitType": "ORDERS",
                "interval": "SECOND",
                "intervalNum": 10,
                "limit": 50,
                "count": 10
            },
            {
                "rateLimitType": "ORDERS",
                "interval": "MINUTE",
                "intervalNum": 1,
                "limit": 1200,
                "count": 250
            },
            {
                "rateLimitType": "ORDERS",
                "interval": "HOUR",
                "intervalNum": 24,
                "limit": 200000,
                "count": 50000
            },
            {
                "rateLimitType": "ORDERS",
                "interval": "DAY",
                "intervalNum": 1,
                "limit": 200000,
                "count": 75000
            }
        ]"#;

        let responses: Vec<RateLimitOrderResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 4);

        // Verify all intervals are present
        assert_eq!(responses[0].interval, "SECOND");
        assert_eq!(responses[1].interval, "MINUTE");
        assert_eq!(responses[2].interval, "HOUR");
        assert_eq!(responses[3].interval, "DAY");

        // Verify all have ORDERS type
        for response in &responses {
            assert_eq!(response.rate_limit_type, "ORDERS");
        }
    }

    #[test]
    fn test_rate_limit_order_response_near_limit() {
        let json = r#"{
            "rateLimitType": "ORDERS",
            "interval": "SECOND",
            "intervalNum": 10,
            "limit": 50,
            "count": 49
        }"#;

        let response: RateLimitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.count, 49);
        assert_eq!(response.limit, 50);
        // Near limit - only 1 order left
    }

    #[test]
    fn test_rate_limit_order_response_at_limit() {
        let json = r#"{
            "rateLimitType": "ORDERS",
            "interval": "SECOND",
            "intervalNum": 10,
            "limit": 50,
            "count": 50
        }"#;

        let response: RateLimitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.count, 50);
        assert_eq!(response.limit, 50);
        // At limit - no more orders allowed
    }

    #[test]
    fn test_rate_limit_order_response_zero_count() {
        let json = r#"{
            "rateLimitType": "ORDERS",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 1200,
            "count": 0
        }"#;

        let response: RateLimitOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.count, 0);
        assert_eq!(response.limit, 1200);
        // No orders placed yet
    }

    #[test]
    fn test_rate_limit_order_response_different_interval_nums() {
        let json = r#"[
            {
                "rateLimitType": "ORDERS",
                "interval": "SECOND",
                "intervalNum": 10,
                "limit": 50,
                "count": 5
            },
            {
                "rateLimitType": "ORDERS",
                "interval": "MINUTE",
                "intervalNum": 5,
                "limit": 6000,
                "count": 1500
            },
            {
                "rateLimitType": "ORDERS",
                "interval": "HOUR",
                "intervalNum": 1,
                "limit": 8333,
                "count": 2000
            }
        ]"#;

        let responses: Vec<RateLimitOrderResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 3);

        assert_eq!(responses[0].interval_num, 10);
        assert_eq!(responses[1].interval_num, 5);
        assert_eq!(responses[2].interval_num, 1);
    }
}
