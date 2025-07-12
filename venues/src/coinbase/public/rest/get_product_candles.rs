//! Get product candles endpoint for Coinbase Exchange REST API
//!
//! Historic rates for a product in OHLCV format.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::coinbase::RestResult;

/// Endpoint URL path for getting product candles
const ENDPOINT_PATH: &str = "products/{}/candles";

/// Request to get product candles
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductCandlesRequest {
    /// Granularity in seconds. Valid values: 60, 300, 900, 3600, 21600, 86400
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<u32>,

    /// Start timestamp for the range of aggregations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// End timestamp for the range of aggregations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// Candle data
///
/// Array format: [timestamp, price_low, price_high, price_open, price_close, volume]
#[derive(Debug, Clone, Deserialize)]
pub struct Candle(
    pub u64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
);

/// Response from getting product candles
pub type GetProductCandlesResponse = Vec<Candle>;

impl RestClient {
    /// Get product candles
    ///
    /// Historic rates for a product. Rates are returned in grouped buckets.
    /// Candle schema is of the form [timestamp, price_low, price_high, price_open, price_close, volume].
    ///
    /// # Arguments
    /// * `product_id` - The product ID (e.g., "BTC-USD")
    /// * `request` - The product candles request parameters
    ///
    /// # Returns
    /// A result containing the product candles or an error
    pub async fn get_product_candles(
        &self,
        product_id: &str,
        request: &GetProductCandlesRequest,
    ) -> RestResult<GetProductCandlesResponse> {
        let endpoint = ENDPOINT_PATH.replace("{}", product_id);
        self.send_request(&endpoint, reqwest::Method::GET, Some(request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_candles_request_serialization() {
        let request = GetProductCandlesRequest {
            granularity: Some(3600),
            start: Some("2021-01-01T00:00:00Z".to_string()),
            end: Some("2021-01-02T00:00:00Z".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("granularity=3600"));
        assert!(serialized.contains("start=2021-01-01T00%3A00%3A00Z"));
        assert!(serialized.contains("end=2021-01-02T00%3A00%3A00Z"));
    }

    #[test]
    fn test_get_product_candles_request_default() {
        let request = GetProductCandlesRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.is_empty());
    }

    #[test]
    fn test_candle_deserialization() {
        let json = r#"[1609459200, 28000.00, 29000.00, 28500.00, 28800.00, 150.5]"#;
        let candle: Candle = serde_json::from_str(json).unwrap();

        assert_eq!(candle.0, 1609459200);
        assert_eq!(candle.1, 28000.00);
        assert_eq!(candle.2, 29000.00);
        assert_eq!(candle.3, 28500.00);
        assert_eq!(candle.4, 28800.00);
        assert_eq!(candle.5, 150.5);
    }

    #[test]
    fn test_get_product_candles_response_deserialization() {
        let json = r#"[
            [1609459200, 28000.00, 29000.00, 28500.00, 28800.00, 150.5],
            [1609462800, 28800.00, 29500.00, 28800.00, 29200.00, 200.3]
        ]"#;

        let candles: GetProductCandlesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(candles.len(), 2);
        assert_eq!(candles[0].0, 1609459200);
        assert_eq!(candles[1].0, 1609462800);
    }
}
