use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecentTradesRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_type: Option<OptionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeInfo {
    pub exec_id: String,
    pub symbol: String,
    pub price: String,
    pub size: String,
    pub side: Side,
    pub time: String,
    pub is_block_trade: bool,
    #[serde(rename = "isRPITrade")]
    pub is_rpi_trade: Option<bool>,
    #[serde(rename = "mP")]
    pub mark_price: Option<String>,
    #[serde(rename = "iP")]
    pub index_price: Option<String>,
    #[serde(rename = "mIv")]
    pub mark_iv: Option<String>,
    pub iv: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecentTradesData {
    pub category: Category,
    pub list: Vec<TradeInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecentTradesResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetRecentTradesData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get recent public trades
    ///
    /// Query recent public trading history in Bybit.
    ///
    /// # Arguments
    /// * `request` - The recent trades request parameters
    ///
    /// # Returns
    /// A result containing the recent trades response or an error
    pub async fn get_recent_trades(
        &self,
        request: GetRecentTradesRequest,
    ) -> RestResult<GetRecentTradesResponse> {
        self.send_public_request(
            "/v5/market/recent-trade",
            Some(&request),
            EndpointType::Market,
        )
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