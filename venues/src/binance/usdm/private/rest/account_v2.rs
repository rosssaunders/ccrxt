use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::PositionSide;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Request parameters for the Account Information V2 endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountV2Request {
    /// Request timestamp in milliseconds since epoch.
    pub timestamp: u64,

    /// The number of milliseconds after timestamp the request is valid for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Asset information for a single asset in Account Information V2 response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetV2 {
    /// Asset name (e.g., "USDT").
    pub asset: String,

    /// Wallet balance for the asset.
    pub wallet_balance: String,

    /// Unrealized profit and loss for the asset.
    pub unrealized_profit: String,

    /// Margin balance for the asset.
    pub margin_balance: String,

    /// Maintenance margin required for the asset.
    pub maint_margin: String,

    /// Initial margin required for the asset.
    pub initial_margin: String,

    /// Position initial margin required for the asset.
    pub position_initial_margin: String,

    /// Open order initial margin required for the asset.
    pub open_order_initial_margin: String,

    /// Cross wallet balance for the asset.
    pub cross_wallet_balance: String,

    /// Cross unrealized profit and loss for the asset.
    pub cross_un_pnl: String,

    /// Available balance for the asset.
    pub available_balance: String,

    /// Maximum amount for transfer out for the asset.
    pub max_withdraw_amount: String,

    /// Whether the asset can be used as margin in Multi-Assets mode.
    pub margin_available: bool,

    /// Last update time for the asset (milliseconds since epoch).
    pub update_time: u64,
}

/// Position information for a single symbol in Account Information V2 response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionV2 {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// Initial margin required for the position.
    pub initial_margin: String,

    /// Maintenance margin required for the position.
    pub maint_margin: String,

    /// Unrealized profit and loss for the position.
    pub unrealized_profit: String,

    /// Position initial margin required for the position.
    pub position_initial_margin: String,

    /// Open order initial margin required for the position.
    pub open_order_initial_margin: String,

    /// Current leverage for the position.
    pub leverage: String,

    /// Whether the position is isolated.
    pub isolated: bool,

    /// Average entry price for the position.
    pub entry_price: String,

    /// Break-even price for the position.
    pub break_even_price: String,

    /// Maximum available notional value for the position.
    pub max_notional: String,

    /// Bid notional value (may be ignored).
    pub bid_notional: String,

    /// Ask notional value (may be ignored).
    pub ask_notional: String,

    /// Position side (see PositionSide enum).
    pub position_side: PositionSide,

    /// Position amount.
    pub position_amt: String,

    /// Last update time for the position (milliseconds since epoch).
    pub update_time: u64,
}

/// Response for Account Information V2 endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountV2Response {
    /// Account commission tier.
    pub fee_tier: u32,

    /// Whether fee discount is enabled.
    pub fee_burn: bool,

    /// Whether the account can trade.
    pub can_trade: bool,

    /// Whether the account can deposit assets.
    pub can_deposit: bool,

    /// Whether the account can withdraw assets.
    pub can_withdraw: bool,

    /// Last update time (reserved, may be ignored).
    pub update_time: u64,

    /// Whether multi-assets margin mode is enabled.
    pub multi_assets_margin: bool,

    /// Trade group ID.
    pub trade_group_id: Option<i64>,

    /// Total initial margin required with current mark price (USD value).
    pub total_initial_margin: String,

    /// Total maintenance margin required (USD value).
    pub total_maint_margin: String,

    /// Total wallet balance (USD value).
    pub total_wallet_balance: String,

    /// Total unrealized profit (USD value).
    pub total_unrealized_profit: String,

    /// Total margin balance (USD value).
    pub total_margin_balance: String,

    /// Total position initial margin (USD value).
    pub total_position_initial_margin: String,

    /// Total open order initial margin (USD value).
    pub total_open_order_initial_margin: String,

    /// Total cross wallet balance (USD value).
    pub total_cross_wallet_balance: String,

    /// Total cross unrealized profit (USD value).
    pub total_cross_un_pnl: String,

    /// Available balance (USD value).
    pub available_balance: String,

    /// Maximum amount for transfer out (USD value).
    pub max_withdraw_amount: String,

    /// List of asset information.
    pub assets: Vec<AssetV2>,

    /// List of position information.
    pub positions: Vec<PositionV2>,
}

/// Endpoint path for Account Information V2.
const ACCOUNT_V2_ENDPOINT: &str = "/fapi/v2/account";

impl UsdmClient {
    /// Account Information V2 (GET /fapi/v2/account)
    ///
    /// Retrieves current account information for a Binance USDM futures account, including balances, positions, and trading permissions.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Account-Information-V2
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// AccountV2Response - The account information response
    pub async fn get_account_v2(
        &self,
        params: GetAccountV2Request,
    ) -> RestResult<AccountV2Response> {
        self.send_signed_request(ACCOUNT_V2_ENDPOINT, Method::GET, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_v2_request_serialization() {
        let req = GetAccountV2Request {
            timestamp: 1234567890,
            recv_window: Some(5000),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("timestamp"));
        assert!(json.contains("recvWindow"));
    }

    #[test]
    fn test_asset_v2_deserialization() {
        let json = r#"{
            "asset": "USDT",
            "walletBalance": "100.0",
            "unrealizedProfit": "0.0",
            "marginBalance": "100.0",
            "maintMargin": "0.0",
            "initialMargin": "0.0",
            "positionInitialMargin": "0.0",
            "openOrderInitialMargin": "0.0",
            "crossWalletBalance": "100.0",
            "crossUnPnl": "0.0",
            "availableBalance": "100.0",
            "maxWithdrawAmount": "100.0",
            "marginAvailable": true,
            "updateTime": 1625474304765
        }"#;
        let asset: AssetV2 = serde_json::from_str(json).unwrap();
        assert_eq!(asset.asset, "USDT");
        assert!(asset.margin_available);
        assert_eq!(asset.update_time, 1625474304765);
    }

    #[test]
    fn test_position_v2_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "initialMargin": "0.0",
            "maintMargin": "0.0",
            "unrealizedProfit": "0.0",
            "positionInitialMargin": "0.0",
            "openOrderInitialMargin": "0.0",
            "leverage": "100",
            "isolated": true,
            "entryPrice": "0.0",
            "breakEvenPrice": "0.0",
            "maxNotional": "250000",
            "bidNotional": "0",
            "askNotional": "0",
            "positionSide": "BOTH",
            "positionAmt": "0",
            "updateTime": 0
        }"#;
        let pos: PositionV2 = serde_json::from_str(json).unwrap();
        assert_eq!(pos.symbol, "BTCUSDT");
        assert_eq!(pos.position_side.to_string(), "BOTH");
        assert!(pos.isolated);
    }

    #[test]
    fn test_account_v2_response_deserialization() {
        let json = r#"{
            "feeTier": 0,
            "feeBurn": true,
            "canTrade": true,
            "canDeposit": true,
            "canWithdraw": true,
            "updateTime": 0,
            "multiAssetsMargin": false,
            "tradeGroupId": -1,
            "totalInitialMargin": "0.00000000",
            "totalMaintMargin": "0.00000000",
            "totalWalletBalance": "23.72469206",
            "totalUnrealizedProfit": "0.00000000",
            "totalMarginBalance": "23.72469206",
            "totalPositionInitialMargin": "0.00000000",
            "totalOpenOrderInitialMargin": "0.00000000",
            "totalCrossWalletBalance": "23.72469206",
            "totalCrossUnPnl": "0.00000000",
            "availableBalance": "23.72469206",
            "maxWithdrawAmount": "23.72469206",
            "assets": [],
            "positions": []
        }"#;
        let resp: AccountV2Response = serde_json::from_str(json).unwrap();
        assert_eq!(resp.fee_tier, 0);
        assert!(resp.fee_burn);
        assert!(resp.can_trade);
        assert_eq!(resp.assets.len(), 0);
        assert_eq!(resp.positions.len(), 0);
    }
}
