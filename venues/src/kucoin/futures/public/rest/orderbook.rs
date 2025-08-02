use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

// API endpoints
const FULL_ORDERBOOK_ENDPOINT: &str = "/api/v1/level2/snapshot";
const PART_ORDERBOOK_ENDPOINT_PREFIX: &str = "/api/v1/level2/depth";

/// Get full orderbook request
#[derive(Debug, Clone, Serialize)]
pub struct GetFullOrderBookRequest {
    pub symbol: String,
}

/// Get part orderbook request
#[derive(Debug, Clone, Serialize)]
pub struct GetPartOrderBookRequest {
    pub symbol: String,
    /// Depth size (20 or 100)
    pub depth: OrderBookDepth,
}

/// Order book depth options
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderBookDepth {
    Depth20,
    Depth100,
}

impl OrderBookDepth {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderBookDepth::Depth20 => "20",
            OrderBookDepth::Depth100 => "100",
        }
    }
}

/// Order book level (price, size)
pub type OrderBookLevel = [f64; 2];

/// Full orderbook response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullOrderBook {
    /// Sequence number
    pub sequence: i64,
    /// Symbol of the contract
    pub symbol: String,
    /// Bid levels (price, size) from high to low
    pub bids: Vec<OrderBookLevel>,
    /// Ask levels (price, size) from low to high
    pub asks: Vec<OrderBookLevel>,
    /// Timestamp (nanoseconds)
    pub ts: i64,
}

/// Part orderbook response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartOrderBook {
    /// Symbol of the contract
    pub symbol: String,
    /// Bid levels (price, size) from high to low
    pub bids: Vec<OrderBookLevel>,
    /// Ask levels (price, size) from low to high
    pub asks: Vec<OrderBookLevel>,
    /// Timestamp (milliseconds)
    pub ts: i64,
}

impl super::RestClient {
    /// Get full orderbook depth data
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-full-orderbook>
    pub async fn get_full_orderbook(
        &self,
        request: GetFullOrderBookRequest,
    ) -> Result<(RestResponse<FullOrderBook>, ResponseHeaders)> {
        self.send_request(FULL_ORDERBOOK_ENDPOINT, Some(&request))
            .await
    }

    /// Get part orderbook depth data (20 or 100 levels)
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-part-orderbook>
    pub async fn get_part_orderbook(
        &self,
        request: GetPartOrderBookRequest,
    ) -> Result<(RestResponse<PartOrderBook>, ResponseHeaders)> {
        let endpoint = format!(
            "{}{}",
            PART_ORDERBOOK_ENDPOINT_PREFIX,
            request.depth.as_str()
        );

        self.send_request(&endpoint, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_depth_as_str() {
        assert_eq!(OrderBookDepth::Depth20.as_str(), "20");
        assert_eq!(OrderBookDepth::Depth100.as_str(), "100");
    }

    #[test]
    fn test_full_orderbook_deserialization() {
        let json = r#"{
            "sequence": 1234567890,
            "symbol": "XBTUSDTM",
            "bids": [[49999.5, 100], [49999.0, 200], [49998.5, 150]],
            "asks": [[50000.5, 80], [50001.0, 120], [50001.5, 200]],
            "ts": 1634567890123456789
        }"#;

        let orderbook: FullOrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(orderbook.symbol, "XBTUSDTM");
        assert_eq!(orderbook.sequence, 1234567890);
        assert_eq!(orderbook.bids.len(), 3);
        assert_eq!(orderbook.asks.len(), 3);
        assert_eq!(orderbook.bids[0][0], 49999.5);
        assert_eq!(orderbook.bids[0][1], 100.0);
        assert_eq!(orderbook.asks[0][0], 50000.5);
        assert_eq!(orderbook.asks[0][1], 80.0);
    }

    #[test]
    fn test_part_orderbook_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "bids": [[49999.5, 100], [49999.0, 200]],
            "asks": [[50000.5, 80], [50001.0, 120]],
            "ts": 1634567890123
        }"#;

        let orderbook: PartOrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(orderbook.symbol, "XBTUSDTM");
        assert_eq!(orderbook.bids.len(), 2);
        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.ts, 1634567890123);
    }

    #[test]
    fn test_get_full_orderbook_request() {
        let request = GetFullOrderBookRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_get_part_orderbook_request() {
        let request = GetPartOrderBookRequest {
            symbol: "ETHUSDTM".to_string(),
            depth: OrderBookDepth::Depth20,
        };
        assert_eq!(request.symbol, "ETHUSDTM");
        assert_eq!(request.depth.as_str(), "20");
    }
}
