use crate::bitget::enums::ProductType;
use rest::Client;
use serde::{Deserialize, Serialize};

/// Get Contract Config
/// 
/// Frequency limit: 20 times/1s (IP)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractConfigRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractConfigData {
    /// Trading pair
    pub symbol: String,
    /// Base coin
    pub base_coin: String,
    /// Quote coin
    pub quote_coin: String,
    /// Buy limit price ratio
    pub buy_limit_price_ratio: String,
    /// Sell limit price ratio
    pub sell_limit_price_ratio: String,
    /// Fee rate group
    pub fee_rate_group: String,
    /// Trading fee taker rate
    pub taker_fee_rate: String,
    /// Trading fee maker rate
    pub maker_fee_rate: String,
    /// Open cost
    pub open_cost: String,
    /// Support margin coins
    pub support_margin_coins: Vec<String>,
    /// Min trade number
    pub min_trade_num: String,
    /// Price endStep
    pub price_end_step: String,
    /// Volume place
    pub volume_place: String,
    /// Price place
    pub price_place: String,
    /// Size multiplier
    pub size_multiplier: String,
    /// Symbol status
    pub symbol_status: String,
    /// Off shelf time
    pub off_shelf_time: String,
    /// Limit open time
    pub limit_open_time: String,
    /// Delivery time
    pub delivery_time: String,
    /// Delivery start time
    pub delivery_start_time: String,
    /// Launch time
    pub launch_time: String,
    /// Fund interval
    pub fund_interval: String,
    /// Min leverage
    pub min_leverage: String,
    /// Max leverage
    pub max_leverage: String,
    /// Position limit
    pub position_limit: String,
    /// Maintain time
    pub maintain_time: String,
}

pub async fn contract_config(
    client: &Client,
    params: &ContractConfigRequest,
) -> Result<Vec<ContractConfigData>, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/market/contracts";
    client.get(endpoint, Some(params)).await.map_err(Into::into)
}
