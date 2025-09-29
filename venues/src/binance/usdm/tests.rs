use std::collections::HashMap;

use crate::binance::{
    shared::client::ResponseHeaders as SharedResponseHeaders,
    usdm::{
        RateLimitHeader, ResponseHeaders,
        rate_limit::{IntervalUnit, RateLimitHeaderKind},
    },
};

#[test]
fn test_response_headers_from_shared() {
    // Create a shared ResponseHeaders with sample rate limit headers
    let mut shared_headers = HashMap::new();
    shared_headers.insert("x-mbx-used-weight-1m".to_string(), "123".to_string());
    shared_headers.insert("x-mbx-order-count-10s".to_string(), "5".to_string());
    shared_headers.insert("x-mbx-order-count-1m".to_string(), "50".to_string());
    shared_headers.insert("x-mbx-order-count-1d".to_string(), "1000".to_string());
    // Add some non-rate-limit headers to ensure they're ignored
    shared_headers.insert("content-type".to_string(), "application/json".to_string());
    shared_headers.insert("x-custom-header".to_string(), "value".to_string());

    let shared = SharedResponseHeaders {
        headers: shared_headers,
    };

    // Convert to USDM ResponseHeaders
    let usdm_headers = ResponseHeaders::from_shared(shared);

    // Verify the converted values
    assert_eq!(usdm_headers.values.len(), 4);

    // Check used weight
    let weight_header = RateLimitHeader {
        kind: RateLimitHeaderKind::UsedWeight,
        interval_value: 1,
        interval_unit: IntervalUnit::Minute,
    };
    assert_eq!(usdm_headers.values.get(&weight_header), Some(&123));

    // Check order count 10s
    let order_10s_header = RateLimitHeader {
        kind: RateLimitHeaderKind::OrderCount,
        interval_value: 10,
        interval_unit: IntervalUnit::Second,
    };
    assert_eq!(usdm_headers.values.get(&order_10s_header), Some(&5));

    // Check order count 1m
    let order_1m_header = RateLimitHeader {
        kind: RateLimitHeaderKind::OrderCount,
        interval_value: 1,
        interval_unit: IntervalUnit::Minute,
    };
    assert_eq!(usdm_headers.values.get(&order_1m_header), Some(&50));

    // Check order count 1d
    let order_1d_header = RateLimitHeader {
        kind: RateLimitHeaderKind::OrderCount,
        interval_value: 1,
        interval_unit: IntervalUnit::Day,
    };
    assert_eq!(usdm_headers.values.get(&order_1d_header), Some(&1000));
}

#[test]
fn test_response_headers_from_shared_with_invalid_values() {
    // Create a shared ResponseHeaders with some invalid values
    let mut shared_headers = HashMap::new();
    shared_headers.insert(
        "x-mbx-used-weight-1m".to_string(),
        "not-a-number".to_string(),
    );
    shared_headers.insert("x-mbx-order-count-10s".to_string(), "5".to_string());

    let shared = SharedResponseHeaders {
        headers: shared_headers,
    };

    // Convert to USDM ResponseHeaders
    let usdm_headers = ResponseHeaders::from_shared(shared);

    // Should only have 1 valid header
    assert_eq!(usdm_headers.values.len(), 1);

    // Check that only the valid header was parsed
    let order_10s_header = RateLimitHeader {
        kind: RateLimitHeaderKind::OrderCount,
        interval_value: 10,
        interval_unit: IntervalUnit::Second,
    };
    assert_eq!(usdm_headers.values.get(&order_10s_header), Some(&5));
}

#[test]
fn test_response_headers_from_shared_empty() {
    // Create an empty shared ResponseHeaders
    let shared = SharedResponseHeaders {
        headers: HashMap::new(),
    };

    // Convert to USDM ResponseHeaders
    let usdm_headers = ResponseHeaders::from_shared(shared);

    // Should be empty
    assert!(usdm_headers.values.is_empty());
}
