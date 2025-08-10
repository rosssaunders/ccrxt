//! Ticker endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for ticker
const TICKER_ENDPOINT: &str = "/trading-api/v1/markets/{}/tick";

/// Request parameters for getting ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTickerRequest {
    /// Market symbol
    pub symbol: String,
}

/// 24-hour ticker statistics
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// Market symbol
    pub symbol: String,

    /// Last trade price
    pub last_price: String,

    /// 24h price change
    pub price_change: String,

    /// 24h price change percentage
    pub price_change_percent: String,

    /// 24h weighted average price
    pub weighted_avg_price: String,

    /// 24h opening price
    pub open_price: String,

    /// 24h high price
    pub high_price: String,

    /// 24h low price
    pub low_price: String,

    /// 24h volume in base asset
    pub volume: String,

    /// 24h volume in quote asset
    pub quote_volume: String,

    /// Best bid price
    pub bid_price: String,

    /// Best bid quantity
    pub bid_qty: String,

    /// Best ask price
    pub ask_price: String,

    /// Best ask quantity
    pub ask_qty: String,

    /// Number of trades in 24h
    pub count: u64,

    /// Ticker timestamp
    pub timestamp: u64,
}

impl RestClient {
    /// Get 24-hour ticker statistics for a market
    ///
    /// Retrieve 24-hour price and volume statistics for a specific market.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-/tick
    ///
    /// # Arguments
    /// * `request` - Request parameters containing the market symbol
    ///
    /// # Returns
    /// 24-hour ticker statistics including price, volume, and order book data
    pub async fn get_ticker(&self, request: &GetTickerRequest) -> RestResult<Ticker> {
        let url = TICKER_ENDPOINT.replace("{}", &request.symbol);

        self.send_get_request(&url, EndpointType::PublicTicker)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDC",
            "lastPrice": "30000.0",
            "priceChange": "500.0",
            "priceChangePercent": "1.69",
            "weightedAvgPrice": "29850.0",
            "openPrice": "29500.0",
            "highPrice": "30500.0",
            "lowPrice": "29000.0",
            "volume": "1000.0",
            "quoteVolume": "29850000.0",
            "bidPrice": "29950.0",
            "bidQty": "10.0",
            "askPrice": "30050.0",
            "askQty": "15.0",
            "count": 5432,
            "timestamp": 1640995200000
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDC");
        assert_eq!(ticker.last_price, "30000.0");
        assert_eq!(ticker.price_change, "500.0");
        assert_eq!(ticker.price_change_percent, "1.69");
        assert_eq!(ticker.volume, "1000.0");
        assert_eq!(ticker.count, 5432);
        assert_eq!(ticker.timestamp, 1640995200000);
    }
}
