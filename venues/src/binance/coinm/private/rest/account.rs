use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    RestResult,
    enums::{MarginType, PositionSide},
    private::rest::client::RestClient,
};

const ACCOUNT_ENDPOINT: &str = "/dapi/v1/account";

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
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Account-Information)
    ///
    /// Corresponds to endpoint GET /dapi/v1/account.
    /// Requires API key authentication.
    /// Weight: 5
    pub async fn get_account(&self, request: AccountRequest) -> RestResult<AccountResponse> {
        self.send_get_signed_request(ACCOUNT_ENDPOINT, request, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_request_serialization() {
        let request = AccountRequest {
            timestamp: 1625097600000,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timestamp=1625097600000");
    }

    #[test]
    fn test_account_request_serialization_with_recv_window() {
        let request = AccountRequest {
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recv_window=5000"));
    }

    #[test]
    fn test_account_response_deserialization() {
        let json = r#"{
            "feeTier": 0,
            "canTrade": true,
            "canDeposit": true,
            "canWithdraw": true,
            "updateTime": 1699948800000,
            "assets": [
                {
                    "asset": "BTC",
                    "walletBalance": "1.50000000",
                    "unrealizedProfit": "0.00000000",
                    "marginBalance": "1.50000000",
                    "maintMargin": "0.00000000",
                    "initialMargin": "0.00000000",
                    "positionInitialMargin": "0.00000000",
                    "openOrderInitialMargin": "0.00000000",
                    "crossWalletBalance": "1.50000000",
                    "crossUnPnl": "0.00000000",
                    "availableBalance": "1.50000000",
                    "maxWithdrawAmount": "1.50000000",
                    "updateTime": 1699948800000
                }
            ],
            "positions": [
                {
                    "symbol": "BTCUSD_PERP",
                    "positionAmt": "10.00000000",
                    "initialMargin": "0.00100000",
                    "maintMargin": "0.00050000",
                    "unrealizedProfit": "0.00010000",
                    "positionInitialMargin": "0.00100000",
                    "openOrderInitialMargin": "0.00000000",
                    "leverage": "20",
                    "isolated": false,
                    "positionSide": "LONG",
                    "entryPrice": "50000.00000000",
                    "breakEvenPrice": "50000.00000000",
                    "maxQty": "1000.00000000",
                    "updateTime": 1699948800000
                }
            ]
        }"#;

        let response: AccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.fee_tier, 0);
        assert!(response.can_trade);
        assert!(response.can_deposit);
        assert!(response.can_withdraw);
        assert_eq!(response.update_time, 1699948800000);
        assert_eq!(response.assets.len(), 1);
        assert_eq!(response.positions.len(), 1);
    }

    #[test]
    fn test_asset_deserialization() {
        let json = r#"{
            "asset": "ETH",
            "walletBalance": "10.50000000",
            "unrealizedProfit": "-0.50000000",
            "marginBalance": "10.00000000",
            "maintMargin": "0.10000000",
            "initialMargin": "0.20000000",
            "positionInitialMargin": "0.15000000",
            "openOrderInitialMargin": "0.05000000",
            "crossWalletBalance": "10.50000000",
            "crossUnPnl": "-0.50000000",
            "availableBalance": "9.80000000",
            "maxWithdrawAmount": "9.80000000",
            "updateTime": 1699948800000
        }"#;

        let asset: Asset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.asset, "ETH");
        assert_eq!(asset.wallet_balance, "10.50000000");
        assert_eq!(asset.unrealized_profit, "-0.50000000");
        assert_eq!(asset.margin_balance, "10.00000000");
        assert_eq!(asset.maint_margin, "0.10000000");
        assert_eq!(asset.initial_margin, "0.20000000");
        assert_eq!(asset.position_initial_margin, "0.15000000");
        assert_eq!(asset.open_order_initial_margin, "0.05000000");
        assert_eq!(asset.cross_wallet_balance, "10.50000000");
        assert_eq!(asset.cross_un_pnl, "-0.50000000");
        assert_eq!(asset.available_balance, "9.80000000");
        assert_eq!(asset.max_withdraw_amount, "9.80000000");
        assert_eq!(asset.update_time, 1699948800000);
    }

    #[test]
    fn test_position_deserialization_cross_margin() {
        let json = r#"{
            "symbol": "ETHUSD_PERP",
            "positionAmt": "-5.00000000",
            "initialMargin": "0.00050000",
            "maintMargin": "0.00025000",
            "unrealizedProfit": "-0.00005000",
            "positionInitialMargin": "0.00050000",
            "openOrderInitialMargin": "0.00000000",
            "leverage": "10",
            "isolated": false,
            "positionSide": "SHORT",
            "entryPrice": "3000.00000000",
            "breakEvenPrice": "3000.00000000",
            "maxQty": "500.00000000",
            "updateTime": 1699948800000
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.symbol, "ETHUSD_PERP");
        assert_eq!(position.position_amt, "-5.00000000");
        assert_eq!(position.initial_margin, "0.00050000");
        assert_eq!(position.maint_margin, "0.00025000");
        assert_eq!(position.unrealized_profit, "-0.00005000");
        assert_eq!(position.position_initial_margin, "0.00050000");
        assert_eq!(position.open_order_initial_margin, "0.00000000");
        assert_eq!(position.leverage, "10");
        assert!(!position.isolated);
        assert_eq!(position.position_side, PositionSide::Short);
        assert_eq!(position.entry_price, "3000.00000000");
        assert_eq!(position.break_even_price, "3000.00000000");
        assert_eq!(position.max_qty, "500.00000000");
        assert_eq!(position.update_time, 1699948800000);
        assert!(position.margin_type.is_none());
        assert!(position.notional_value.is_none());
        assert!(position.isolated_wallet.is_none());
    }

    #[test]
    fn test_position_deserialization_isolated_margin() {
        let json = r#"{
            "symbol": "SOLUSD_240329",
            "positionAmt": "100.00000000",
            "initialMargin": "0.01000000",
            "maintMargin": "0.00500000",
            "unrealizedProfit": "0.00100000",
            "positionInitialMargin": "0.01000000",
            "openOrderInitialMargin": "0.00000000",
            "leverage": "5",
            "isolated": true,
            "marginType": "ISOLATED",
            "positionSide": "BOTH",
            "entryPrice": "100.00000000",
            "breakEvenPrice": "100.00000000",
            "maxQty": "10000.00000000",
            "notionalValue": "10000.00000000",
            "isolatedWallet": "0.01100000",
            "updateTime": 1699948800000
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.symbol, "SOLUSD_240329");
        assert_eq!(position.position_amt, "100.00000000");
        assert!(position.isolated);
        assert_eq!(position.margin_type, Some(MarginType::Isolated));
        assert_eq!(position.position_side, PositionSide::Both);
        assert_eq!(position.notional_value, Some("10000.00000000".to_string()));
        assert_eq!(position.isolated_wallet, Some("0.01100000".to_string()));
    }
}
