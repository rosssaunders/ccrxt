use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::{
    options::{OptionsContractType, OptionsPositionSide, RestResult},
    shared,
};

/// Request parameters for querying position information
#[derive(Debug, Clone, Serialize, Default)]
pub struct PositionRequest {
    /// Option trading pair (if omitted, returns all positions)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Position information
#[derive(Debug, Clone, Deserialize)]
pub struct Position {
    /// Average entry price
    #[serde(rename = "entryPrice")]
    pub entry_price: Decimal,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Position direction (LONG or SHORT)
    #[serde(rename = "side")]
    pub side: OptionsPositionSide,

    /// Number of positions (positive for long, negative for short)
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Number of positions that can be reduced
    #[serde(rename = "reducibleQty")]
    pub reducible_qty: Decimal,

    /// Current market value
    #[serde(rename = "markValue")]
    pub mark_value: Decimal,

    /// Rate of return
    #[serde(rename = "ror")]
    pub ror: Decimal,

    /// Unrealized profit/loss
    #[serde(rename = "unrealizedPNL")]
    pub unrealized_pnl: Decimal,

    /// Mark price
    #[serde(rename = "markPrice")]
    pub mark_price: Decimal,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,

    /// Position cost
    #[serde(rename = "positionCost")]
    pub position_cost: Decimal,

    /// Exercise time (expiry date)
    #[serde(rename = "expiryDate")]
    pub expiry_date: u64,

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
    /// Get current position information
    ///
    /// Returns current position information for option contracts.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/trade/Option-Position-Information)
    /// Method: GET /eapi/v1/position
    /// Weight: 5
    /// Requires: API key and signature
    pub async fn get_position(&self, params: PositionRequest) -> RestResult<Vec<Position>> {
        shared::send_signed_request(
            self,
            "/eapi/v1/position",
            reqwest::Method::GET,
            params,
            5,
            false,
        )
        .await
    }
}
