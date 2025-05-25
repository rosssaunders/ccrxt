use serde::Deserialize;
use super::types::BinanceCoinMResult;
use super::private_rest::BinanceCoinMPrivateRest;
use super::enums::{PositionSide, MarginType};

/// Represents the overall account information.
#[derive(Debug, Deserialize)]
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
    /// Detailed information about each asset in the account.
    pub assets: Vec<Asset>,
    /// Detailed information about each position held by the user.
    pub positions: Vec<Position>,
}

/// Represents details for a specific asset within the account.
#[derive(Debug, Deserialize)]
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
    /// The timestamp of the last update for this asset.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}



/// Represents details for a specific position held by the user.
#[derive(Debug, Deserialize)]
pub struct Position {
    /// The trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,
    /// The quantity of the position. Positive for long, negative for short.
    #[serde(rename = "positionAmt")]
    pub position_amt: String,
    /// The initial margin required for this position.
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    /// The maintenance margin required for this position.
    #[serde(rename = "maintMargin")]
    pub maint_margin: String,
    /// The unrealized profit or loss for this position.
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: String,
    /// The margin required for positions of this asset.
    #[serde(rename = "positionInitialMargin")]
    pub position_initial_margin: String,
    /// The initial margin required for open orders of this asset.
    #[serde(rename = "openOrderInitialMargin")]
    pub open_order_initial_margin: String,
    /// The leverage used for this position.
    pub leverage: String,
    /// Whether the position is isolated margin (false = cross, true = isolated).
    pub isolated: bool,
    /// The margin type for this position (Cross or Isolated).
    #[serde(rename = "marginType")]
    pub margin_type: MarginType,
    /// The side of the position (Both, Long, Short).
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,
    /// The average entry price of the position.
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    /// The break-even price of the position.
    #[serde(rename = "breakEvenPrice")]
    pub break_even_price: String,
    /// The maximum quantity of base asset allowed for this position.
    #[serde(rename = "maxQty")]
    pub max_qty: String,
    /// The notional value of the position (optional, only present in some positions).
    #[serde(rename = "notionalValue")]
    pub notional_value: Option<String>,
    /// The timestamp of the last update for this position.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

impl BinanceCoinMPrivateRest {
    /// Fetches the user's account information, including assets and positions.
    /// See: https://binance-docs.github.io/apidocs/delivery/en/#account-information-user_data
    /// Corresponds to endpoint GET /dapi/v1/account.
    /// Requires API key authentication.
    /// Weight: 5
    pub async fn get_account(&self) -> BinanceCoinMResult<AccountResponse> {
        self.send_signed_request(
            "/dapi/v1/account",
            reqwest::Method::GET,
            None, // No additional query parameters needed
        ).await
    }
}