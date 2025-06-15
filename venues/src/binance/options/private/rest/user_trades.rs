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

impl UserTradesRequest {
    pub fn new() -> Self {
        Self {
            symbol: None,
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        }
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

impl Default for UserTradesRequest {
    fn default() -> Self {
        Self::new()
    }
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

    /// Get recent trades for all symbols
    pub async fn get_recent_trades(&self) -> RestResult<Vec<UserTradeResponse>> {
        self.get_user_trades(UserTradesRequest::default()).await
    }

    /// Get trades for a specific symbol
    pub async fn get_trades_for_symbol(&self, symbol: String) -> RestResult<Vec<UserTradeResponse>> {
        self.get_user_trades(UserTradesRequest::new().symbol(symbol)).await
    }

    /// Get trades within a time range
    pub async fn get_trades_in_range(
        &self,
        start_time: u64,
        end_time: u64,
    ) -> RestResult<Vec<UserTradeResponse>> {
        self.get_user_trades(
            UserTradesRequest::new()
                .start_time(start_time)
                .end_time(end_time),
        )
        .await
    }
}