use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

const RECENT_TRADES_ENDPOINT: &str = "/v5/market/recent-trade";

/// Request parameters for getting recent trades
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecentTradesRequest {
    /// Product type
    pub category: Category,

    /// Symbol name. Required for Spot/Linear/Inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Base coin. For Option only, returns all option symbols of the base coin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,

    /// Option type. For Option only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_type: Option<OptionType>,

    /// Limit for data size per page. Spot: [1,60], others: [1,1000]. Default: 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Trade information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeInfo {
    /// Execution ID
    pub exec_id: String,

    /// Symbol name
    pub symbol: String,

    /// Trade price
    pub price: String,

    /// Trade size
    pub size: String,

    /// Side of taker
    pub side: Side,

    /// Trade time in milliseconds
    pub time: String,

    /// Whether it's a block trade
    pub is_block_trade: bool,

    /// Whether it's a RPI trade. Valid for Spot only
    #[serde(rename = "isRPITrade")]
    pub is_rpi_trade: Option<bool>,

    /// Mark price. Valid for Option only
    #[serde(rename = "mP")]
    pub mark_price: Option<String>,

    /// Index price. Valid for Option only
    #[serde(rename = "iP")]
    pub index_price: Option<String>,

    /// Mark IV. Valid for Option only
    #[serde(rename = "mIv")]
    pub mark_iv: Option<String>,

    /// IV. Valid for Option only
    pub iv: Option<String>,
}

/// Recent trades data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetRecentTradesData {
    /// Product type
    pub category: Category,

    /// Array of trade data
    pub list: Vec<TradeInfo>,
}

/// Response from the recent trades endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetRecentTradesResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetRecentTradesData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get recent public trades
    ///
    /// Query recent public trading history in Bybit.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/recent-trade)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The recent trades request parameters including:
    ///   - `category`: Product type
    ///   - `symbol`: Optional symbol name (required for Spot/Linear/Inverse)
    ///   - `base_coin`: Optional base coin (for Option only)
    ///   - `option_type`: Optional option type (for Option only)
    ///   - `limit`: Optional result limit
    ///
    /// # Returns
    /// A result containing the recent trades response with trade data or an error
    pub async fn get_recent_trades(
        &self,
        request: GetRecentTradesRequest,
    ) -> RestResult<GetRecentTradesResponse> {
        self.send_public_request(RECENT_TRADES_ENDPOINT, Some(&request), EndpointType::Market)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_recent_trades_request_direct_construction() {
        let request = GetRecentTradesRequest {
            category: Category::Spot,
            symbol: Some("BTCUSDT".to_string()),
            limit: Some(10),
            base_coin: None,
            option_type: None,
        };

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.limit, Some(10));
        assert!(request.base_coin.is_none());
        assert!(request.option_type.is_none());
    }

    #[test]
    fn test_get_recent_trades_request_options() {
        let request = GetRecentTradesRequest {
            category: Category::Option,
            symbol: None,
            base_coin: Some("ETH".to_string()),
            option_type: Some(OptionType::Call),
            limit: Some(100),
        };

        assert_eq!(request.category, Category::Option);
        assert_eq!(request.base_coin, Some("ETH".to_string()));
        assert_eq!(request.option_type, Some(OptionType::Call));
        assert_eq!(request.limit, Some(100));
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_trade_info_deserialization() {
        let json = r#"{
            "execId": "2100000000007764263",
            "symbol": "BTCUSDT",
            "price": "16618.49",
            "size": "0.00012",
            "side": "Buy",
            "time": "1672052955758",
            "isBlockTrade": false,
            "isRPITrade": true
        }"#;

        let trade: TradeInfo = serde_json::from_str(json).unwrap();
        assert_eq!(trade.exec_id, "2100000000007764263");
        assert_eq!(trade.symbol, "BTCUSDT");
        assert_eq!(trade.side, Side::Buy);
        assert!(!trade.is_block_trade);
        assert_eq!(trade.is_rpi_trade, Some(true));
    }
}
