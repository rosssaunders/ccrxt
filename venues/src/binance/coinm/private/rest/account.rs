use crate::binance::coinm::enums::{MarginType, PositionSide};
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::RestResult;
use serde::{Deserialize, Serialize};

/// Request parameters for fetching account information.
#[derive(Debug, Serialize)]
pub struct AccountRequest {
    /// The timestamp of the request in milliseconds.
    /// This is a mandatory parameter.
    pub timestamp: u64,

    /// The receive window for the request in milliseconds.
    /// Optional parameter that specifies how long the request is valid for.
    /// If not provided, the server will use a default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Represents the overall account information.
#[derive(Debug, Deserialize)]
pub struct AccountResponse {
    /// User's fee tier.
    /// Range: 0 to 9, where higher numbers indicate lower fees.
    #[serde(rename = "feeTier")]
    pub fee_tier: i32,

    /// Whether the user can trade.
    /// True if the account is enabled for trading, false if trading is disabled.
    #[serde(rename = "canTrade")]
    pub can_trade: bool,

    /// Whether the user can make deposits.
    /// True if the account is enabled for deposits, false if deposits are disabled.
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,

    /// Whether the user can make withdrawals.
    /// True if the account is enabled for withdrawals, false if withdrawals are disabled.
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,

    /// The timestamp of the last account update.
    /// Format: Unix timestamp in milliseconds.
    #[serde(rename = "updateTime")]
    pub update_time: i64,

    /// Detailed information about each asset in the account.
    /// List of assets and their balances, margins, and other details.
    pub assets: Vec<Asset>,

    /// Detailed information about each position held by the user.
    /// List of open positions with their details.
    pub positions: Vec<Position>,
}

/// Represents details for a specific asset within the account.
#[derive(Debug, Deserialize)]
pub struct Asset {
    /// The symbol of the asset (e.g., "BTC", "ETH").
    /// The asset identifier used by the exchange.
    pub asset: String,

    /// The balance of this asset in the wallet.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,

    /// The unrealized profit or loss for this asset.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Positive for profit, negative for loss.
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: String,

    /// The margin balance for this asset.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Calculated as walletBalance + unrealizedProfit.
    #[serde(rename = "marginBalance")]
    pub margin_balance: String,

    /// The maintenance margin required for this asset.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Minimum margin required to maintain positions.
    #[serde(rename = "maintMargin")]
    pub maint_margin: String,

    /// The initial margin required for this asset.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Total initial margin across all positions.
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,

    /// The initial margin required for positions of this asset.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Initial margin allocated to positions.
    #[serde(rename = "positionInitialMargin")]
    pub position_initial_margin: String,

    /// The initial margin required for open orders of this asset.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Initial margin allocated to open orders.
    #[serde(rename = "openOrderInitialMargin")]
    pub open_order_initial_margin: String,

    /// The wallet balance for this asset in cross margin mode.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "crossWalletBalance")]
    pub cross_wallet_balance: String,

    /// The unrealized profit or loss for this asset in cross margin mode.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Positive for profit, negative for loss.
    #[serde(rename = "crossUnPnl")]
    pub cross_un_pnl: String,

    /// The available balance of this asset for placing new orders.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Calculated as marginBalance - initialMargin.
    #[serde(rename = "availableBalance")]
    pub available_balance: String,

    /// The maximum amount of this asset that can be withdrawn.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: String,

    /// The timestamp of the last update for this asset.
    /// Format: Unix timestamp in milliseconds.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

/// Represents details for a specific position held by the user.
#[derive(Debug, Deserialize)]
pub struct Position {
    /// The trading symbol (e.g., "BTCUSD_PERP").
    /// Format: "{ASSET}USD_PERP" for perpetual contracts, "{ASSET}USD_{EXPIRY}" for delivery contracts.
    pub symbol: String,

    /// The quantity of the position. Positive for long, negative for short.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "positionAmt")]
    pub position_amt: String,

    /// The initial margin required for this position.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,

    /// The maintenance margin required for this position.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Minimum margin required to maintain this position.
    #[serde(rename = "maintMargin")]
    pub maint_margin: String,

    /// The unrealized profit or loss for this position.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Positive for profit, negative for loss.
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: String,

    /// The margin required for positions of this asset.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "positionInitialMargin")]
    pub position_initial_margin: String,

    /// The initial margin required for open orders of this asset.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "openOrderInitialMargin")]
    pub open_order_initial_margin: String,

    /// The leverage used for this position.
    /// Format: Decimal string (e.g., "20" for 20x leverage).
    pub leverage: String,

    /// Whether the position is isolated margin (false = cross, true = isolated).
    /// True if using isolated margin, false if using cross margin.
    pub isolated: bool,

    /// The margin type for this position (Cross or Isolated).
    /// Only present when isolated is true.
    #[serde(rename = "marginType")]
    #[serde(default)]
    pub margin_type: Option<MarginType>,

    /// The side of the position (Both, Long, Short).
    /// Indicates the direction of the position.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// The average entry price of the position.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "entryPrice")]
    pub entry_price: String,

    /// The break-even price of the position.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "breakEvenPrice")]
    pub break_even_price: String,

    /// The maximum quantity of base asset allowed for this position.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "maxQty")]
    pub max_qty: String,

    /// The notional value of the position.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Only present in some positions.
    #[serde(rename = "notionalValue")]
    #[serde(default)]
    pub notional_value: Option<String>,

    /// The isolated wallet for this position.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Only present for isolated positions.
    #[serde(rename = "isolatedWallet")]
    #[serde(default)]
    pub isolated_wallet: Option<String>,

    /// The timestamp of the last update for this position.
    /// Format: Unix timestamp in milliseconds.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

impl RestClient {
    /// Fetches the user's account information, including assets and positions.
    /// See: https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Account-Information
    /// Corresponds to endpoint GET /dapi/v1/account.
    /// Requires API key authentication.
    /// Weight: 5
    pub async fn get_account(
        &self,
        request: AccountRequest,
    ) -> RestResult<AccountResponse> {
        self.send_signed_request("/dapi/v1/account", reqwest::Method::GET, request, 5, false)
            .await
    }
}
