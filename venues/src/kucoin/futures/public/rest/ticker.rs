use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{OrderSide, ResponseHeaders, RestResponse, Result};

// API endpoints
const TICKER_ENDPOINT: &str = "/api/v1/ticker";
const ALL_TICKERS_ENDPOINT: &str = "/api/v1/allTickers";
const STATS_24HR_ENDPOINT: &str = "/api/v1/stats/24hr";

/// Get ticker request
#[derive(Debug, Clone, Serialize)]
pub struct GetTickerRequest {
    pub symbol: String,
}

/// Ticker information response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerInfo {
    /// Sequence number, used to judge whether the messages pushed by Websocket are continuous
    pub sequence: i64,
    /// Symbol of the contract
    pub symbol: String,
    /// Trade side (taker order side)
    pub side: OrderSide,
    /// Filled quantity
    pub size: i64,
    /// Transaction ID
    pub trade_id: String,
    /// Filled price
    pub price: String,
    /// Best bid price
    pub best_bid_price: String,
    /// Best bid size
    pub best_bid_size: i64,
    /// Best ask price
    pub best_ask_price: String,
    /// Best ask size
    pub best_ask_size: i64,
    /// Filled time (nanoseconds)
    pub ts: i64,
}

/// Get all tickers request
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllTickersRequest;

/// All tickers response item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllTickersItem {
    /// Symbol of the contract
    pub symbol: String,
    /// Best bid price
    pub best_bid_price: String,
    /// Best bid size
    pub best_bid_size: i64,
    /// Best ask price
    pub best_ask_price: String,
    /// Best ask size
    pub best_ask_size: i64,
    /// Last traded price
    pub price: String,
    /// Sequence number
    pub sequence: i64,
    /// Last traded size
    pub size: i64,
    /// Last trade time (nanoseconds)
    pub ts: i64,
}

/// Response for getting all tickers
pub type GetAllTickersResponse = Vec<AllTickersItem>;

/// Get 24hr stats request
#[derive(Debug, Clone, Serialize)]
pub struct Get24HrStatsRequest {
    pub symbol: String,
}

/// 24hr stats response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats24Hr {
    /// Symbol of the contract
    pub symbol: String,
    /// 24h volume of contracts (buy side)
    pub volume_buy: f64,
    /// 24h volume of contracts (sell side)
    pub volume_sell: f64,
    /// 24h volume of contracts
    pub volume: f64,
    /// 24h turnover value
    pub turnover: f64,
    /// Last traded price
    pub last: String,
    /// 24h lowest price
    pub low_price: String,
    /// 24h highest price
    pub high_price: String,
    /// 24h price change
    pub price_chg: String,
    /// 24h price change percentage
    pub price_chg_pct: String,
    /// Current open interest
    pub open_interest: f64,
}

impl super::RestClient {
    /// Get ticker information for a specific symbol
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-ticker>
    pub async fn get_ticker(
        &self,
        request: GetTickerRequest,
    ) -> Result<(RestResponse<TickerInfo>, ResponseHeaders)> {
        self.send_request(TICKER_ENDPOINT, Some(&request)).await
    }

    /// Get all ticker information
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-all-tickers>
    pub async fn get_all_tickers(
        &self,
        _request: GetAllTickersRequest,
    ) -> Result<(RestResponse<GetAllTickersResponse>, ResponseHeaders)> {
        self.send_request(ALL_TICKERS_ENDPOINT, None::<&GetAllTickersRequest>)
            .await
    }

    /// Get 24hr stats for a specific symbol
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-24hr-stats>
    pub async fn get_24hr_stats(
        &self,
        request: Get24HrStatsRequest,
    ) -> Result<(RestResponse<Stats24Hr>, ResponseHeaders)> {
        self.send_request(STATS_24HR_ENDPOINT, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ticker_request_creation() {
        let request = GetTickerRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_ticker_info_deserialization() {
        let json = r#"{
            "sequence": 1234567890,
            "symbol": "XBTUSDTM",
            "side": "buy",
            "size": 100,
            "tradeId": "5e8c8c2f1a3b4a001c5d8e31",
            "price": "50000.0",
            "bestBidPrice": "49999.5",
            "bestBidSize": 500,
            "bestAskPrice": "50000.5",
            "bestAskSize": 300,
            "ts": 1634567890123456789
        }"#;

        let ticker: TickerInfo = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "XBTUSDTM");
        assert_eq!(ticker.price, "50000.0");
        assert_eq!(ticker.size, 100);
    }

    #[test]
    fn test_all_tickers_item_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "bestBidPrice": "49999.5",
            "bestBidSize": 500,
            "bestAskPrice": "50000.5",
            "bestAskSize": 300,
            "price": "50000.0",
            "sequence": 1234567890,
            "size": 100,
            "ts": 1634567890123456789
        }"#;

        let ticker: AllTickersItem = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "XBTUSDTM");
        assert_eq!(ticker.price, "50000.0");
        assert_eq!(ticker.best_bid_price, "49999.5");
    }

    #[test]
    fn test_24hr_stats_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "volumeBuy": 10000.0,
            "volumeSell": 9500.0,
            "volume": 19500.0,
            "turnover": 975000000.0,
            "last": "50000.0",
            "lowPrice": "48000.0",
            "highPrice": "52000.0",
            "priceChg": "2000.0",
            "priceChgPct": "0.0417",
            "openInterest": 500000.0
        }"#;

        let stats: Stats24Hr = serde_json::from_str(json).unwrap();
        assert_eq!(stats.symbol, "XBTUSDTM");
        assert_eq!(stats.last, "50000.0");
        assert_eq!(stats.volume, 19500.0);
        assert_eq!(stats.price_chg_pct, "0.0417");
    }
}
