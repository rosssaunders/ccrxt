use serde::{Deserialize, Serialize};
use super::api_errors::BinanceCoinMResult;
use super::private_rest::BinanceCoinMPrivateRest;
use super::types::BinanceResponse;
use super::common::request::send_request;
use super::enums::PositionSide;

/// Represents the overall account information.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    /// User's fee tier.
    #[serde(rename = "feeTier")]
    pub fee_tier: i32,
    /// Whether the user can trade.
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    /// Whether the user can make deposits.
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    /// Whether the user can make withdrawals.
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    /// The timestamp of the last account update.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// Total initial margin required for all positions in USDT.
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: String,
    /// Total maintenance margin required for all positions in USDT.
    #[serde(rename = "totalMaintMargin")]
    pub total_maint_margin: String,
    /// Total wallet balance in USDT.
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    /// Total unrealized profit for all positions in USDT.
    #[serde(rename = "totalUnrealizedProfit")]
    pub total_unrealized_profit: String,
    /// Total margin balance in USDT.
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    /// Total initial margin required for all positions in USDT (same as totalInitialMargin).
    #[serde(rename = "totalPositionInitialMargin")]
    pub total_position_initial_margin: String,
    /// Total initial margin required for all open orders in USDT.
    #[serde(rename = "totalOpenOrderInitialMargin")]
    pub total_open_order_initial_margin: String,
    /// Total wallet balance for cross margin accounts in USDT.
    #[serde(rename = "totalCrossWalletBalance")]
    pub total_cross_wallet_balance: String,
    /// Available balance for new orders in USDT.
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    /// Maximum amount that can be withdrawn in USDT.
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: String,
    /// Detailed information about each asset in the account.
    pub assets: Vec<Asset>,
    /// Detailed information about each position held by the user.
    pub positions: Vec<Position>,
}

/// Represents details for a specific asset within the account.
#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    /// The symbol of the asset (e.g., "BTC", "ETH").
    pub asset: String,
    /// The balance of this asset in the wallet.
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,
    /// The unrealized profit or loss for this asset.
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: String,
    /// The margin balance for this asset.
    #[serde(rename = "marginBalance")]
    pub margin_balance: String,
    /// The maintenance margin required for this asset.
    #[serde(rename = "maintMargin")]
    pub maint_margin: String,
    /// The initial margin required for this asset.
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    /// The initial margin required for positions of this asset.
    #[serde(rename = "positionInitialMargin")]
    pub position_initial_margin: String,
    /// The initial margin required for open orders of this asset.
    #[serde(rename = "openOrderInitialMargin")]
    pub open_order_initial_margin: String,
    /// The wallet balance for this asset in cross margin mode.
    #[serde(rename = "crossWalletBalance")]
    pub cross_wallet_balance: String,
    /// The unrealized profit or loss for this asset in cross margin mode.
    #[serde(rename = "crossUnPnl")]
    pub cross_un_pnl: String,
    /// The available balance of this asset for placing new orders.
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    /// The maximum amount of this asset that can be withdrawn.
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: String,
    /// Whether margin is available for this asset.
    #[serde(rename = "marginAvailable")]
    pub margin_available: bool,
    /// The timestamp of the last update for this asset.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

/// Represents the margin type for a position (Cross or Isolated).
/// Note: Ideally, this should be in the main enums.rs file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountMarginType {
    Cross,
    Isolated,
}

/// Represents details for a specific position held by the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    /// The trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,
    /// The quantity of the position. Positive for long, negative for short.
    #[serde(rename = "positionAmt")]
    pub position_amt: String,
    /// The average entry price of the position.
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    /// The current mark price of the underlying asset.
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    /// The unrealized profit or loss for this position.
    #[serde(rename = "unRealizedProfit")]
    pub unrealized_profit: String,
    /// The estimated liquidation price for this position.
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: String,
    /// The leverage used for this position.
    pub leverage: String,
    /// The maximum notional value allowed for this position.
    #[serde(rename = "maxNotionalValue")]
    pub max_notional_value: String,
    /// The margin type used for this position (Cross or Isolated).
    #[serde(rename = "marginType")]
    pub margin_type: AccountMarginType,
    /// The isolated margin allocated to this position (if marginType is Isolated).
    #[serde(rename = "isolatedMargin")]
    pub isolated_margin: String,
    /// Whether auto-margin addition is enabled ("true" or "false"). Parsed as bool.
    #[serde(rename = "isAutoAddMargin", deserialize_with = "deserialize_string_as_bool")]
    pub is_auto_add_margin: bool,
    /// The side of the position (Both, Long, Short).
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,
    /// The notional value of the position (Mark Price * |Position Amount|).
    pub notional: String,
    /// The isolated wallet balance associated with this position.
    #[serde(rename = "isolatedWallet")]
    pub isolated_wallet: String,
    /// The timestamp of the last update for this position.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

// Helper function to deserialize "true" or "false" strings to bool
fn deserialize_string_as_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    match s.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&s),
            &"true or false",
        )),
    }
}

impl BinanceCoinMPrivateRest {
    /// Fetches the user's account information, including assets and positions.
    /// Corresponds to endpoint GET /dapi/v1/account.
    /// Requires API key authentication.
    /// Weight: 5
    pub async fn get_account(&self) -> BinanceCoinMResult<BinanceResponse<AccountResponse>> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query_string = format!("timestamp={}", timestamp);
        
        // Get the decrypted API key
        let api_key = self.get_api_key()?;

        // Sign the request using the decrypted secret (handled within sign_request)
        let signature = self.sign_request(&query_string)?;
        
        let endpoint = format!("/dapi/v1/account?{}&signature={}", query_string, signature);
        
        send_request(
            &self.client,
            &self.base_url,
            &endpoint,
            reqwest::Method::GET,
            None,
            // Expose the decrypted secret key for the header
            Some(api_key.expose_secret()),
            // Use the specific weight for this endpoint
            || self.rate_limiter.check_weight_limit("account", 5) 
        ).await
    }
} 