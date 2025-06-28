//! Get product candles endpoint for Coinbase Exchange REST API
//!
//! Historic rates for a product in OHLCV format.

use serde::{Deserialize, Serialize};

use crate::coinbase::RestResult;

use super::RestClient;

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
    pub String,
    pub String,
    pub String,
    pub String,
    pub String,
);

impl Candle {
    /// Get the timestamp
    pub fn timestamp(&self) -> u64 {
        self.0
    }

    /// Get the low price
    pub fn low(&self) -> &str {
        &self.1
    }

    /// Get the high price
    pub fn high(&self) -> &str {
        &self.2
    }

    /// Get the open price
    pub fn open(&self) -> &str {
        &self.3
    }

    /// Get the close price
    pub fn close(&self) -> &str {
        &self.4
    }

    /// Get the volume
    pub fn volume(&self) -> &str {
        &self.5
    }
}

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
    ///
    /// # Example
    /// ```no_run
    /// use ccrxt::venues::coinbase::public::rest::{GetProductCandlesRequest, RestClient};
    /// use reqwest::Client;
    /// use ccrxt::venues::coinbase::RateLimiter;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new(
    ///     "https://api.exchange.coinbase.com",
    ///     Client::new(),
    ///     RateLimiter::new(),
    /// );
    ///
    /// let request = GetProductCandlesRequest {
    ///     granularity: Some(3600), // 1 hour candles
    ///     start: None,
    ///     end: None,
    /// };
    /// let candles = client.get_product_candles("BTC-USD", &request).await?;
    /// println!("Found {} candles", candles.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_product_candles(
        &self,
        product_id: &str,
        request: &GetProductCandlesRequest,
    ) -> RestResult<GetProductCandlesResponse> {
        let endpoint = format!("products/{}/candles", product_id);
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
        let json = r#"[1609459200, "28000.00", "29000.00", "28500.00", "28800.00", "150.5"]"#;
        let candle: Candle = serde_json::from_str(json).unwrap();

        assert_eq!(candle.timestamp(), 1609459200);
        assert_eq!(candle.low(), "28000.00");
        assert_eq!(candle.high(), "29000.00");
        assert_eq!(candle.open(), "28500.00");
        assert_eq!(candle.close(), "28800.00");
        assert_eq!(candle.volume(), "150.5");
    }

    #[test]
    fn test_get_product_candles_response_deserialization() {
        let json = r#"[
            [1609459200, "28000.00", "29000.00", "28500.00", "28800.00", "150.5"],
            [1609462800, "28800.00", "29500.00", "28800.00", "29200.00", "200.3"]
        ]"#;

        let candles: GetProductCandlesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(candles.len(), 2);
        assert_eq!(candles[0].timestamp(), 1609459200);
        assert_eq!(candles[1].timestamp(), 1609462800);
    }
}
