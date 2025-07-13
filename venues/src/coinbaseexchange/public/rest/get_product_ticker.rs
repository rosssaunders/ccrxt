//! Get product ticker endpoint for Coinbase Exchange REST API
//!
//! Gets snapshot information about the last trade, best bid/ask and 24h volume.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::coinbaseexchange::RestResult;

/// Endpoint URL path for getting product ticker
const ENDPOINT_PATH: &str = "products/{}/ticker";

/// Request to get product ticker
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductTickerRequest {}

/// Product ticker information
#[derive(Debug, Clone, Deserialize)]
pub struct ProductTicker {
    /// Best ask price
    pub ask: String,

    /// Best bid price
    pub bid: String,

    /// 24-hour volume
    pub volume: String,

    /// Trade ID of the last trade
    pub trade_id: u64,

    /// Price of the last trade
    pub price: String,

    /// Size of the last trade
    pub size: String,

    /// Time of the last trade
    pub time: DateTime<Utc>,

    /// RFQ volume
    #[serde(default)]
    pub rfq_volume: String,

    /// Conversions volume
    #[serde(default)]
    pub conversions_volume: String,
}

/// Response from getting product ticker
pub type GetProductTickerResponse = ProductTicker;

impl RestClient {
    /// Get product ticker
    ///
    /// Gets snapshot information about the last trade (tick), best bid/ask and 24h volume.
    /// For real-time updates, Coinbase recommends connecting with the WebSocket stream
    /// and listening for match messages, rather than polling.
    ///
    /// # Arguments
    /// * `product_id` - The product ID (e.g., "BTC-USD")
    /// * `request` - The product ticker request parameters
    ///
    /// # Returns
    /// A result containing the product ticker or an error
    pub async fn get_product_ticker(
        &self,
        product_id: &str,
        request: &GetProductTickerRequest,
    ) -> RestResult<GetProductTickerResponse> {
        let endpoint = ENDPOINT_PATH.replace("{}", product_id);
        self.send_request(&endpoint, reqwest::Method::GET, Some(request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_ticker_request_serialization() {
        let request = GetProductTickerRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.is_empty());
    }

    #[test]
    fn test_product_ticker_deserialization() {
        let json = r#"{
            "ask": "29000.00",
            "bid": "28999.00",
            "volume": "1500.25",
            "trade_id": 12345,
            "price": "28999.50",
            "size": "0.1",
            "time": "2021-01-01T00:00:00.000Z",
            "rfq_volume": "100.00",
            "conversions_volume": "50.00"
        }"#;

        let ticker: ProductTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.ask, "29000.00");
        assert_eq!(ticker.bid, "28999.00");
        assert_eq!(ticker.volume, "1500.25");
        assert_eq!(ticker.trade_id, 12345);
        assert_eq!(ticker.price, "28999.50");
        assert_eq!(ticker.size, "0.1");
        assert_eq!(ticker.rfq_volume, "100.00");
        assert_eq!(ticker.conversions_volume, "50.00");
    }
}
