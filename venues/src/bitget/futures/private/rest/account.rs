use crate::bitget::{BitgetRestClient, ProductType, MarginCoin, MarginMode};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Position mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionMode {
    /// Hedge mode (allow both long and short positions)
    #[serde(rename = "hedge_mode")]
    HedgeMode,
    /// One way mode (net position)
    #[serde(rename = "one_way_mode")]
    OneWayMode,
}

impl std::fmt::Display for PositionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionMode::HedgeMode => write!(f, "hedge_mode"),
            PositionMode::OneWayMode => write!(f, "one_way_mode"),
        }
    }
}

/// Asset mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetMode {
    /// Union asset mode
    #[serde(rename = "union")]
    Union,
    /// Single asset mode
    #[serde(rename = "single")]
    Single,
}

impl std::fmt::Display for AssetMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetMode::Union => write!(f, "union"),
            AssetMode::Single => write!(f, "single"),
        }
    }
}

/// Account information for a specific margin coin
#[derive(Debug, Clone, Deserialize)]
pub struct FuturesAccount {
    /// Margin coin
    #[serde(rename = "marginCoin")]
    pub margin_coin: String,
    
    /// Locked balance
    pub locked: String,
    
    /// Available balance
    pub available: String,
    
    /// Maximum available for crossed margin
    #[serde(rename = "crossedMaxAvailable")]
    pub crossed_max_available: String,
    
    /// Maximum available for isolated margin
    #[serde(rename = "isolatedMaxAvailable")]
    pub isolated_max_available: String,
    
    /// Maximum transferable amount
    #[serde(rename = "maxTransferOut")]
    pub max_transfer_out: String,
    
    /// Account equity
    #[serde(rename = "accountEquity")]
    pub account_equity: String,
    
    /// USDT equity
    #[serde(rename = "usdtEquity")]
    pub usdt_equity: String,
    
    /// BTC equity
    #[serde(rename = "btcEquity")]
    pub btc_equity: String,
    
    /// Crossed margin risk rate
    #[serde(rename = "crossedRiskRate")]
    pub crossed_risk_rate: String,
    
    /// Crossed margin leverage
    #[serde(rename = "crossedMarginLeverage")]
    pub crossed_margin_leverage: String,
    
    /// Isolated long leverage
    #[serde(rename = "isolatedLongLever")]
    pub isolated_long_lever: String,
    
    /// Isolated short leverage
    #[serde(rename = "isolatedShortLever")]
    pub isolated_short_lever: String,
    
    /// Margin mode
    #[serde(rename = "marginMode")]
    pub margin_mode: MarginMode,
    
    /// Position mode
    #[serde(rename = "posMode")]
    pub pos_mode: PositionMode,
    
    /// Unrealized P&L
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: String,
    
    /// Coupon amount
    pub coupon: String,
    
    /// Crossed unrealized P&L
    #[serde(rename = "crossedUnrealizedPL")]
    pub crossed_unrealized_pl: String,
    
    /// Isolated unrealized P&L
    #[serde(rename = "isolatedUnrealizedPL")]
    pub isolated_unrealized_pl: String,
    
    /// Asset mode
    #[serde(rename = "assetMode")]
    pub asset_mode: AssetMode,
}

/// Request for getting single account information
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountRequest {
    /// Trading pair
    pub symbol: String,
    
    /// Product type
    #[serde(rename = "productType")]
    pub product_type: ProductType,
    
    /// Margin coin
    #[serde(rename = "marginCoin")]
    pub margin_coin: MarginCoin,
}

/// Response for getting single account information
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountResponse {
    /// Account information
    pub data: FuturesAccount,
}

impl GetAccountRequest {
    /// Create a new account request
    pub fn new(
        symbol: impl Into<String>,
        product_type: ProductType,
        margin_coin: MarginCoin,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            product_type,
            margin_coin,
        }
    }
}

impl BitgetRequest for GetAccountRequest {
    type Response = GetAccountResponse;

    fn path(&self) -> String {
        "/api/v2/mix/account/account".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_request() {
        let request = GetAccountRequest::new(
            "BTCUSDT",
            ProductType::UsdtFutures,
            MarginCoin::Usdt,
        );

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.product_type, ProductType::UsdtFutures);
        assert_eq!(request.margin_coin, MarginCoin::Usdt);
        assert_eq!(request.path(), "/api/v2/mix/account/account");
        assert_eq!(request.method(), "GET".to_string());
        assert!(request.need_signature());
    }

    #[test]
    fn test_position_mode_display() {
        assert_eq!(PositionMode::HedgeMode.to_string(), "hedge_mode");
        assert_eq!(PositionMode::OneWayMode.to_string(), "one_way_mode");
    }

    #[test]
    fn test_asset_mode_display() {
        assert_eq!(AssetMode::Union.to_string(), "union");
        assert_eq!(AssetMode::Single.to_string(), "single");
    }
}
