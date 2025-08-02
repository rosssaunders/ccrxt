use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::PositionSide;

/// Endpoint path for Account Information V3.
const ACCOUNT_INFO_ENDPOINT: &str = "/fapi/v3/account";

/// Request parameters for the Account Information V3 endpoint.
///
/// See [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Account-Information-V3
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountV3Request {
    /// Request timestamp in milliseconds since epoch.
    /// Must be the current server time.
    pub timestamp: u64,

    /// The number of milliseconds after timestamp the request is valid for. Optional.
    /// If omitted, default is 5000ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Asset information for a single asset in Account Information V3 response.
///
/// All fields are returned as strings per Binance API. See [docs].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetV3 {
    /// Asset name (e.g., "USDT").
    /// See API docs for supported assets.
    pub asset: String,

    /// Wallet balance for the asset.
    /// String value representing the balance.
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

    /// Whether the asset can be used as margin in Multi-Assets mode. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_available: Option<bool>,

    /// Last update time for the asset (milliseconds since epoch).
    pub update_time: u64,
}

/// Position information for a single symbol in Account Information V3 response.
///
/// See API docs for details on position modes and returned fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionV3 {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// Position side (see PositionSide enum).
    /// "BOTH" for One-way mode, "LONG"/"SHORT" for Hedge mode.
    pub position_side: PositionSide,

    /// Position amount as string.
    pub position_amt: String,

    /// Unrealized profit and loss for the position.
    pub unrealized_profit: String,

    /// Isolated margin for the position.
    pub isolated_margin: String,

    /// Notional value for the position.
    pub notional: String,

    /// Isolated wallet for the position.
    pub isolated_wallet: String,

    /// Initial margin required for the position.
    pub initial_margin: String,

    /// Maintenance margin required for the position.
    pub maint_margin: String,

    /// Last update time for the position (milliseconds since epoch).
    pub update_time: u64,
}

/// Response for Account Information V3 endpoint.
///
/// See [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Account-Information-V3
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountV3Response {
    /// Total initial margin required with current mark price (USD value).
    /// String value, see API docs for details.
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
    pub assets: Vec<AssetV3>,

    /// List of position information.
    pub positions: Vec<PositionV3>,
}

impl UsdmClient {
    /// Account Information V3
    ///
    /// Retrieves current account information for a Binance USDM futures account, including balances, positions, and trading permissions.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Account-Information-V3
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// AccountV3Response - The account information response
    pub async fn get_account_v3(
        &self,
        params: GetAccountV3Request,
    ) -> RestResult<AccountV3Response> {
        self.send_get_signed_request(
            ACCOUNT_INFO_ENDPOINT,
            params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_v3_request_serialization() {
        let req = GetAccountV3Request {
            timestamp: 1234567890,
            recv_window: Some(5000),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("timestamp"));
        assert!(json.contains("recvWindow"));
    }

    #[test]
    fn test_asset_v3_deserialization() {
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
            "updateTime": 1625474304765
        }"#;
        let asset: AssetV3 = serde_json::from_str(json).unwrap();
        assert_eq!(asset.asset, "USDT");
        assert_eq!(asset.wallet_balance, "100.0");
        assert_eq!(asset.update_time, 1625474304765);
    }

    #[test]
    fn test_position_v3_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "positionSide": "BOTH",
            "positionAmt": "1.000",
            "unrealizedProfit": "0.00000000",
            "isolatedMargin": "0.00000000",
            "notional": "0",
            "isolatedWallet": "0",
            "initialMargin": "0",
            "maintMargin": "0",
            "updateTime": 0
        }"#;
        let pos: PositionV3 = serde_json::from_str(json).unwrap();
        assert_eq!(pos.symbol, "BTCUSDT");
        assert_eq!(pos.position_side.to_string(), "BOTH");
        assert_eq!(pos.position_amt, "1.000");
    }

    #[test]
    fn test_account_v3_response_deserialization() {
        let json = r#"{
            "totalInitialMargin": "0.00000000",
            "totalMaintMargin": "0.00000000",
            "totalWalletBalance": "103.12345678",
            "totalUnrealizedProfit": "0.00000000",
            "totalMarginBalance": "103.12345678",
            "totalPositionInitialMargin": "0.00000000",
            "totalOpenOrderInitialMargin": "0.00000000",
            "totalCrossWalletBalance": "103.12345678",
            "totalCrossUnPnl": "0.00000000",
            "availableBalance": "103.12345678",
            "maxWithdrawAmount": "103.12345678",
            "assets": [],
            "positions": []
        }"#;
        let resp: AccountV3Response = serde_json::from_str(json).unwrap();
        assert_eq!(resp.total_wallet_balance, "103.12345678");
        assert_eq!(resp.assets.len(), 0);
        assert_eq!(resp.positions.len(), 0);
    }
}
