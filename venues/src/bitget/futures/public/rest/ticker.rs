use crate::bitget::{BitgetRestClient, ProductType};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Ticker data for a single symbol
#[derive(Debug, Clone, Deserialize)]
pub struct FuturesTicker {
    /// Trading pair name
    pub symbol: String,

    /// Last price
    #[serde(rename = "lastPr")]
    pub last_price: String,

    /// Ask price
    #[serde(rename = "askPr")]
    pub ask_price: String,

    /// Bid price
    #[serde(rename = "bidPr")]
    pub bid_price: String,

    /// Buying amount
    #[serde(rename = "bidSz")]
    pub bid_size: String,

    /// Selling amount
    #[serde(rename = "askSz")]
    pub ask_size: String,

    /// 24h high
    #[serde(rename = "high24h")]
    pub high_24h: String,

    /// 24h low
    #[serde(rename = "low24h")]
    pub low_24h: String,

    /// Milliseconds format of current data timestamp Unix
    pub ts: String,

    /// Price increase or decrease (24 hours)
    #[serde(rename = "change24h")]
    pub change_24h: String,

    /// Trading volume of the coin
    #[serde(rename = "baseVolume")]
    pub base_volume: String,

    /// Trading volume of quote currency
    #[serde(rename = "quoteVolume")]
    pub quote_volume: String,

    /// Trading volume of USDT
    #[serde(rename = "usdtVolume")]
    pub usdt_volume: String,

    /// UTC0 opening price
    #[serde(rename = "openUtc")]
    pub open_utc: String,

    /// UTC0 24-hour price increase and decrease
    #[serde(rename = "changeUtc24h")]
    pub change_utc_24h: String,

    /// Index price
    #[serde(rename = "indexPrice")]
    pub index_price: String,

    /// Funding rate
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,

    /// Current holding positions (base coin)
    #[serde(rename = "holdingAmount")]
    pub holding_amount: String,

    /// Entry price of the last 24 hours
    #[serde(rename = "open24h")]
    pub open_24h: String,

    /// Delivery start time (only for delivery contracts)
    #[serde(rename = "deliveryStartTime")]
    pub delivery_start_time: Option<String>,

    /// Delivery time (only for delivery contracts)
    #[serde(rename = "deliveryTime")]
    pub delivery_time: Option<String>,

    /// Delivery status (only for delivery contracts)
    #[serde(rename = "deliveryStatus")]
    pub delivery_status: Option<String>,

    /// Mark price
    #[serde(rename = "markPrice")]
    pub mark_price: String,
}

/// Request for getting ticker data
#[derive(Debug, Clone, Serialize)]
pub struct GetTickerRequest {
    /// Trading pair
    pub symbol: String,

    /// Product type
    #[serde(rename = "productType")]
    pub product_type: ProductType,
}

/// Response for getting ticker data
#[derive(Debug, Clone, Deserialize)]
pub struct GetTickerResponse {
    /// Ticker data
    pub data: Vec<FuturesTicker>,
}

impl GetTickerRequest {
    /// Create a new ticker request
    pub fn new(symbol: impl Into<String>, product_type: ProductType) -> Self {
        Self {
            symbol: symbol.into(),
            product_type,
        }
    }
}

impl BitgetRequest for GetTickerRequest {
    type Response = GetTickerResponse;

    fn path(&self) -> String {
        "/api/v2/mix/market/ticker".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        false
    }
}

/// Request for getting all tickers
#[derive(Debug, Clone, Serialize)]
pub struct GetAllTickersRequest {
    /// Product type
    #[serde(rename = "productType")]
    pub product_type: ProductType,
}

/// Response for getting all tickers
#[derive(Debug, Clone, Deserialize)]
pub struct GetAllTickersResponse {
    /// All ticker data
    pub data: Vec<FuturesTicker>,
}

impl GetAllTickersRequest {
    /// Create a new all tickers request
    pub fn new(product_type: ProductType) -> Self {
        Self { product_type }
    }
}

impl BitgetRequest for GetAllTickersRequest {
    type Response = GetAllTickersResponse;

    fn path(&self) -> String {
        "/api/v2/mix/market/tickers".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_request() {
        let request = GetTickerRequest::new("BTCUSDT", ProductType::UsdtFutures);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.product_type, ProductType::UsdtFutures);
        assert_eq!(request.path(), "/api/v2/mix/market/ticker");
        assert_eq!(request.method(), "GET".to_string());
        assert!(!request.need_signature());
    }

    #[test]
    fn test_all_tickers_request() {
        let request = GetAllTickersRequest::new(ProductType::CoinFutures);
        assert_eq!(request.product_type, ProductType::CoinFutures);
        assert_eq!(request.path(), "/api/v2/mix/market/tickers");
        assert_eq!(request.method(), "GET".to_string());
        assert!(!request.need_signature());
    }
}
