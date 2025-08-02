use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const GET_MARGIN_ACCOUNT_ENDPOINT: &str = "/eapi/v1/marginAccount";

/// Request parameters for querying margin account information
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarginAccountRequest {
    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Margin account information
#[derive(Debug, Clone, Deserialize)]
pub struct MarginAccount {
    /// Total margin balance
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: Decimal,

    /// Total initial margin
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: Decimal,

    /// Total maintenance margin
    #[serde(rename = "totalMaintenanceMargin")]
    pub total_maintenance_margin: Decimal,

    /// Total wallet balance
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: Decimal,

    /// Total unrealized profit
    #[serde(rename = "totalUnrealizedProfit")]
    pub total_unrealized_profit: Decimal,

    /// Total position value
    #[serde(rename = "totalPositionValue")]
    pub total_position_value: Decimal,

    /// Available balance
    #[serde(rename = "availableBalance")]
    pub available_balance: Decimal,

    /// Maximum withdraw amount
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: Decimal,

    /// Assets information
    #[serde(rename = "assets")]
    pub assets: Vec<MarginAsset>,
}

/// Margin asset information
#[derive(Debug, Clone, Deserialize)]
pub struct MarginAsset {
    /// Asset name
    #[serde(rename = "asset")]
    pub asset: String,

    /// Margin balance
    #[serde(rename = "marginBalance")]
    pub margin_balance: Decimal,

    /// Initial margin
    #[serde(rename = "initialMargin")]
    pub initial_margin: Decimal,

    /// Maintenance margin
    #[serde(rename = "maintMargin")]
    pub maint_margin: Decimal,

    /// Wallet balance
    #[serde(rename = "walletBalance")]
    pub wallet_balance: Decimal,

    /// Unrealized profit
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: Decimal,

    /// Position value
    #[serde(rename = "positionValue")]
    pub position_value: Decimal,

    /// Available balance
    #[serde(rename = "availableBalance")]
    pub available_balance: Decimal,
}

impl RestClient {
    /// Get option margin account information
    ///
    /// Returns margin account information including balance, margins, and asset details.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/account/Option-Margin-Account-Information)
    /// Method: GET /eapi/v1/marginAccount
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn get_margin_account(
        &self,
        params: MarginAccountRequest,
    ) -> RestResult<MarginAccount> {
        self.send_get_signed_request(
            GET_MARGIN_ACCOUNT_ENDPOINT,
            params,
            1,
            false,
        )
        .await
    }
}
