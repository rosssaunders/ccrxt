use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::{OptionsRiskLevel, RestResult};
use crate::binance::shared;

use super::client::RestClient;

/// Request parameters for the account information endpoint
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountRequest {
    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Account asset information
#[derive(Debug, Clone, Deserialize)]
pub struct AccountAsset {
    /// Asset symbol (e.g., "USDT")
    #[serde(rename = "asset")]
    pub asset: String,

    /// Account balance
    #[serde(rename = "marginBalance")]
    pub margin_balance: Decimal,

    /// Account equity
    #[serde(rename = "equity")]
    pub equity: Decimal,

    /// Available funds
    #[serde(rename = "available")]
    pub available: Decimal,

    /// Locked balance for orders and positions
    #[serde(rename = "locked")]
    pub locked: Decimal,

    /// Unrealized profit/loss
    #[serde(rename = "unrealizedPNL")]
    pub unrealized_pnl: Decimal,
}

/// Account Greeks information for an underlying
#[derive(Debug, Clone, Deserialize)]
pub struct AccountGreeks {
    /// Option underlying (e.g., "BTCUSDT")
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Account delta
    #[serde(rename = "delta")]
    pub delta: Decimal,

    /// Account gamma
    #[serde(rename = "gamma")]
    pub gamma: Decimal,

    /// Account theta
    #[serde(rename = "theta")]
    pub theta: Decimal,

    /// Account vega
    #[serde(rename = "vega")]
    pub vega: Decimal,
}

/// Account information response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountResponse {
    /// Asset information array
    #[serde(rename = "asset")]
    pub assets: Vec<AccountAsset>,

    /// Greeks information array
    #[serde(rename = "greek")]
    pub greeks: Vec<AccountGreeks>,

    /// Response timestamp
    #[serde(rename = "time")]
    pub time: u64,

    /// Account risk level
    #[serde(rename = "riskLevel")]
    pub risk_level: OptionsRiskLevel,
}

impl RestClient {
    /// Get current account information
    ///
    /// Returns account balance, equity, available funds, Greeks, and risk level.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/account)
    /// Method: GET /eapi/v1/account
    /// Weight: 3
    /// Requires: API key and signature
    pub async fn get_account_info(&self, params: AccountRequest) -> RestResult<AccountResponse> {
        shared::send_signed_request(
            self,
            "/eapi/v1/account",
            reqwest::Method::GET,
            params,
            3,
            false,
        )
        .await
    }
}
