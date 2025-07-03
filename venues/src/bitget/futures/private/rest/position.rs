use crate::bitget::enums::{ProductType, HoldSide, MarginMode, PositionMode};
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Single Position
/// 
/// Rate limit: 10 requests/sec/UID
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SinglePositionRequest {
    /// Product type
    pub product_type: ProductType,
    /// Trading pair
    pub symbol: String,
    /// Margin coin
    pub margin_coin: String,
}

/// Get All Positions
/// 
/// Rate limit: 5 requests/sec/UID
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPositionsRequest {
    /// Product type
    pub product_type: ProductType,
    /// Margin coin (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_coin: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    /// Trading pair name
    pub symbol: String,
    /// Margin coin
    pub margin_coin: String,
    /// Position direction
    pub hold_side: HoldSide,
    /// Amount to be filled of the current order (base coin)
    pub open_delegate_size: String,
    /// Margin amount (margin coin)
    pub margin_size: String,
    /// Available amount for positions (base currency)
    pub available: String,
    /// Frozen amount in the position (base currency)
    pub locked: String,
    /// Total amount of all positions (available amount + locked amount)
    pub total: String,
    /// Leverage
    pub leverage: String,
    /// Realized PnL(exclude funding fee and transaction fee)
    pub achieved_profits: String,
    /// Average entry price
    pub open_price_avg: String,
    /// Margin mode
    pub margin_mode: MarginMode,
    /// Position mode
    pub pos_mode: PositionMode,
    /// Unrealized PnL
    pub unrealized_pl: String,
    /// Estimated liquidation price
    pub liquidation_price: String,
    /// Tiered maintenance margin rate
    pub keep_margin_rate: String,
    /// Mark price
    pub mark_price: String,
    /// Maintenance margin rate (MMR)
    pub margin_ratio: String,
    /// Position breakeven price
    pub break_even_price: String,
    /// Funding fee
    pub total_fee: String,
    /// Deducted transaction fees
    pub deducted_fee: String,
    /// Take profit price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Take profit order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit_id: Option<String>,
    /// Stop loss order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss_id: Option<String>,
    /// Asset mode
    pub asset_mode: String,
    /// Auto Margin
    pub auto_margin: String,
    /// Creation time
    pub c_time: String,
    /// Last updated time
    pub u_time: String,
}

pub async fn single_position(
    client: &Client,
    params: &SinglePositionRequest,
) -> Result<Vec<PositionData>, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/position/single-position";
    client.get_signed(endpoint, Some(params)).await.map_err(Into::into)
}

pub async fn all_positions(
    client: &Client,
    params: &AllPositionsRequest,
) -> Result<Vec<PositionData>, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/position/all-position";
    client.get_signed(endpoint, Some(params)).await.map_err(Into::into)
}
