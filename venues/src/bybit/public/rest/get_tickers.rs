use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult, enums::*};

/// Endpoint URL path for tickers
const ENDPOINT_PATH: &str = "/v5/market/tickers";

/// Request parameters for getting tickers
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickersRequest {
    /// Product type
    pub category: Category,

    /// Symbol name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Base coin. For Option only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,

    /// Expiry date. For Option only, e.g., "25DEC22"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
}

/// Ticker information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerInfo {
    /// Symbol name
    pub symbol: String,

    /// Last traded price
    #[serde(rename = "lastPrice")]
    pub last_price: String,

    /// Index price
    #[serde(rename = "indexPrice")]
    pub index_price: Option<String>,

    /// Mark price
    #[serde(rename = "markPrice")]
    pub mark_price: Option<String>,

    /// Market price 24 hours ago
    #[serde(rename = "prevPrice24h")]
    pub prev_price_24h: String,

    /// Percentage change of market price relative to 24h
    #[serde(rename = "price24hPcnt")]
    pub price_24h_pcnt: String,

    /// Highest price in the last 24 hours
    #[serde(rename = "highPrice24h")]
    pub high_price_24h: String,

    /// Lowest price in the last 24 hours
    #[serde(rename = "lowPrice24h")]
    pub low_price_24h: String,

    /// Market price an hour ago. Option only
    #[serde(rename = "prevPrice1h")]
    pub prev_price_1h: Option<String>,

    /// Open interest. Includes both buyers and sellers side
    #[serde(rename = "openInterest")]
    pub open_interest: Option<String>,

    /// Open interest value
    #[serde(rename = "openInterestValue")]
    pub open_interest_value: Option<String>,

    /// Turnover for 24h
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,

    /// Volume for 24h
    #[serde(rename = "volume24h")]
    pub volume_24h: String,

    /// Funding rate
    #[serde(rename = "fundingRate")]
    pub funding_rate: Option<String>,

    /// Next funding time in milliseconds
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: Option<String>,

    /// Predicted delivery price. Futures only
    #[serde(rename = "predictedDeliveryPrice")]
    pub predicted_delivery_price: Option<String>,

    /// Basis rate. Futures only
    #[serde(rename = "basisRate")]
    pub basis_rate: Option<String>,

    /// Basis. Futures only
    pub basis: Option<String>,

    /// Delivery fee rate. Futures only
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: Option<String>,

    /// Delivery timestamp in milliseconds. Futures only
    #[serde(rename = "deliveryTime")]
    pub delivery_time: Option<String>,

    /// Best ask size
    #[serde(rename = "ask1Size")]
    pub ask1_size: String,

    /// Best bid price
    #[serde(rename = "bid1Price")]
    pub bid1_price: String,

    /// Best ask price
    #[serde(rename = "ask1Price")]
    pub ask1_price: String,

    /// Best bid size
    #[serde(rename = "bid1Size")]
    pub bid1_size: String,

    /// Pre-open price. Linear & Inverse only
    #[serde(rename = "preOpenPrice")]
    pub pre_open_price: Option<String>,

    /// Pre-open quantity. Linear & Inverse only
    #[serde(rename = "preQty")]
    pub pre_qty: Option<String>,

    /// Current pre-listing phase. Spot only
    #[serde(rename = "curPreListingPhase")]
    pub cur_pre_listing_phase: Option<String>,

    /// USD index price. Linear only
    #[serde(rename = "usdIndexPrice")]
    pub usd_index_price: Option<String>,
}

/// Tickers data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetTickersData {
    /// Product type
    pub category: Category,

    /// Array of ticker info
    pub list: Vec<TickerInfo>,
}

/// Response from the tickers endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetTickersResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetTickersData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get tickers
    ///
    /// Query for the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/tickers)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The tickers request parameters including:
    ///   - `category`: Product type
    ///   - `symbol`: Optional symbol name
    ///   - `base_coin`: Optional base coin (for Option only)
    ///   - `exp_date`: Optional expiry date (for Option only)
    ///
    /// # Returns
    /// A result containing the tickers response with price and volume data or an error
    pub async fn get_tickers(&self, request: GetTickersRequest) -> RestResult<GetTickersResponse> {
        self.send_public_request(ENDPOINT_PATH, Some(&request), EndpointType::Market)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tickers_request_builder() {
        let request = GetTickersRequest {
            category: Category::Linear,
            symbol: Some("BTCUSDT".to_string()),
            base_coin: None,
            exp_date: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert!(request.base_coin.is_none());
        assert!(request.exp_date.is_none());
    }

    #[test]
    fn test_get_tickers_request_options() {
        let request = GetTickersRequest {
            category: Category::Option,
            symbol: None,
            base_coin: Some("BTC".to_string()),
            exp_date: Some("25DEC22".to_string()),
        };

        assert_eq!(request.category, Category::Option);
        assert_eq!(request.base_coin, Some("BTC".to_string()));
        assert_eq!(request.exp_date, Some("25DEC22".to_string()));
        assert!(request.symbol.is_none());
    }
}
