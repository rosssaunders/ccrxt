use crate::bitget::enums::{ProductType, HoldSide, MarginMode, OrderSide};
use rest::Client;
use serde::{Deserialize, Serialize};

/// Change Leverage
/// 
/// Frequency limit: 5 times/1s (uid)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Margin coin
    pub margin_coin: String,
    /// Leverage
    pub leverage: String,
    /// Position direction (required for isolated hedge mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_side: Option<HoldSide>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageResponse {
    /// Trading pair name
    pub symbol: String,
    /// Margin coin
    pub margin_coin: String,
    /// Leverage of long positions
    pub long_leverage: String,
    /// Leverage of short positions
    pub short_leverage: String,
    /// Leverage of 'crossed' margin mode
    pub cross_margin_leverage: String,
    /// Margin mode
    pub margin_mode: MarginMode,
}

/// Change Margin Mode
/// 
/// Frequency limit: 5 times/1s (uid)
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Margin coin
    pub margin_coin: String,
    /// Margin mode
    pub margin_mode: MarginMode,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeResponse {
    /// Trading pair name
    pub symbol: String,
    /// Margin coin
    pub margin_coin: String,
    /// Leverage of long positions
    pub long_leverage: String,
    /// Leverage of short positions
    pub short_leverage: String,
    /// Margin mode
    pub margin_mode: MarginMode,
}

/// Adjust Position Margin
/// 
/// Rate limit: 5 req/sec/UID
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginRequest {
    /// Trading pair
    pub symbol: String,
    /// Product type
    pub product_type: ProductType,
    /// Margin coin
    pub margin_coin: String,
    /// Position direction
    pub hold_side: HoldSide,
    /// Margin amount (positive means increase, negative means decrease)
    pub amount: String,
}

pub async fn set_leverage(
    client: &Client,
    params: &SetLeverageRequest,
) -> Result<SetLeverageResponse, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/account/set-leverage";
    client.post_signed(endpoint, Some(params)).await.map_err(Into::into)
}

pub async fn set_margin_mode(
    client: &Client,
    params: &SetMarginModeRequest,
) -> Result<SetMarginModeResponse, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/account/set-margin-mode";
    client.post_signed(endpoint, Some(params)).await.map_err(Into::into)
}

pub async fn set_margin(
    client: &Client,
    params: &SetMarginRequest,
) -> Result<String, crate::bitget::Errors> {
    let endpoint = "/api/v2/mix/account/set-margin";
    client.post_signed(endpoint, Some(params)).await.map_err(Into::into)
}
