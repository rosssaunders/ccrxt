//! User trade endpoints for Binance Options Private API

use serde::{Deserialize, Serialize};

/// Request parameters for user trades
#[derive(Debug, Clone, Serialize)]
pub struct UserTradesRequest {
    /// Option symbol, e.g. BTC-200730-9000-C (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Trade id to fetch from. Default gets most recent trades
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Default 100; max 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}



/// User trade response
#[derive(Debug, Clone, Deserialize)]
pub struct UserTradeResponse {
    /// Unique id
    pub id: u64,
    /// Trade id
    #[serde(rename = "tradeId")]
    pub trade_id: u64,
    /// Order id
    #[serde(rename = "orderId")]
    pub order_id: u64,
    /// Option symbol
    pub symbol: String,
    /// Trade price
    pub price: String,
    /// Trade quantity
    pub quantity: String,
    /// Fee
    pub fee: String,
    /// Realized profit/loss
    #[serde(rename = "realizedProfit")]
    pub realized_profit: String,
    /// Order side
    pub side: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,
    /// Volatility
    pub volatility: String,
    /// TAKER or MAKER
    pub liquidity: String,
    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    /// Trade time
    pub time: u64,
    #[serde(rename = "priceScale")]
    pub price_scale: u32,
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,
    #[serde(rename = "optionSide")]
    pub option_side: String,
}

use crate::binance::options::{PrivateRestClient, RestResult};

impl PrivateRestClient {
    /// Get trades for a specific account and symbol
    ///
    /// # Arguments
    /// * `request` - User trades request parameters
    ///
    /// # Returns
    /// List of user trades
    ///
    /// # Weight
    /// 5
    pub async fn get_user_trades(&self, request: UserTradesRequest) -> RestResult<Vec<UserTradeResponse>> {
        self.send_signed_request(
            "/eapi/v1/userTrades",
            reqwest::Method::GET,
            request,
            5, // weight
            false, // not an order
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_trades_request_creation() {
        let request = UserTradesRequest {
            symbol: None,
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        };
        assert!(request.symbol.is_none());
        assert!(request.from_id.is_none());

        let request_with_params = UserTradesRequest {
            symbol: Some("BTC-200730-9000-C".to_string()),
            from_id: Some(12345),
            start_time: None,
            end_time: None,
            limit: Some(100),
            recv_window: None,
        };
        assert_eq!(
            request_with_params.symbol,
            Some("BTC-200730-9000-C".to_string())
        );
        assert_eq!(request_with_params.from_id, Some(12345));
        assert_eq!(request_with_params.limit, Some(100));
    }
}