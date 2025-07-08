use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::{
    options::{OptionsContractType, OptionsOrderSide, RestResult},
    shared,
};

/// Request parameters for querying user trades
#[derive(Debug, Clone, Serialize)]
pub struct UserTradesRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Trade ID to start from (returns trades with ID >= this value)
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Start time
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of result sets returned (default: 100, max: 1000)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// User trade record
#[derive(Debug, Clone, Deserialize)]
pub struct UserTrade {
    /// Trade ID
    #[serde(rename = "id")]
    pub id: u64,

    /// Trade ID (same as id)
    #[serde(rename = "tradeId")]
    pub trade_id: u64,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Quote quantity
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Fee amount
    #[serde(rename = "fee")]
    pub fee: Decimal,

    /// Realized profit/loss
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: Decimal,

    /// Trade time
    #[serde(rename = "time")]
    pub time: u64,

    /// Volatility
    #[serde(rename = "volatility")]
    pub volatility: Decimal,

    /// Volatility for Greeks calculation
    #[serde(rename = "volatilityForGreeks")]
    pub volatility_for_greeks: Decimal,

    /// Underlying price
    #[serde(rename = "underlyingPrice")]
    pub underlying_price: Decimal,

    /// Underlying price for Greeks calculation
    #[serde(rename = "underlyingPriceForGreeks")]
    pub underlying_price_for_greeks: Decimal,

    /// Vega
    #[serde(rename = "vega")]
    pub vega: Decimal,

    /// Delta
    #[serde(rename = "delta")]
    pub delta: Decimal,

    /// Gamma
    #[serde(rename = "gamma")]
    pub gamma: Decimal,

    /// Theta
    #[serde(rename = "theta")]
    pub theta: Decimal,

    /// Price scale
    #[serde(rename = "priceScale")]
    pub price_scale: u32,

    /// Quantity scale
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,

    /// Option side (CALL or PUT)
    #[serde(rename = "optionSide")]
    pub option_side: OptionsContractType,

    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
}

impl RestClient {
    /// Query user trades
    ///
    /// Returns user's trade history for the specified symbol.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/trade/Option-Trade-History)
    /// Method: GET /eapi/v1/userTrades
    /// Weight: 5
    /// Requires: API key and signature
    pub async fn get_user_trades(&self, params: UserTradesRequest) -> RestResult<Vec<UserTrade>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/userTrades",
            reqwest::Method::GET,
            params,
            5,
            false,
        )
        .await
    }
}
