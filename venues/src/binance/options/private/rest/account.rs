//! Account endpoints for Binance Options Private API

use serde::{Deserialize, Serialize};

/// Request parameters for account information
#[derive(Debug, Clone, Serialize)]
pub struct AccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

impl AccountRequest {
    pub fn new() -> Self {
        Self { recv_window: None }
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

impl Default for AccountRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Asset information in account response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountAsset {
    /// Asset type
    pub asset: String,
    /// Account balance
    #[serde(rename = "marginBalance")]
    pub margin_balance: String,
    /// Account equity
    pub equity: String,
    /// Available funds
    pub available: String,
    /// Locked balance for order and position
    pub locked: String,
    /// Unrealized profit/loss
    #[serde(rename = "unrealizedPNL")]
    pub unrealized_pnl: String,
}

/// Greek values in account response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountGreek {
    /// Option underlying
    pub underlying: String,
    /// Account delta
    pub delta: String,
    /// Account gamma
    pub gamma: String,
    /// Account theta
    pub theta: String,
    /// Account vega
    pub vega: String,
}

/// Account information response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountResponse {
    /// Asset information
    pub asset: Vec<AccountAsset>,
    /// Greek values
    pub greek: Vec<AccountGreek>,
    /// Time
    pub time: u64,
    /// Account risk level
    #[serde(rename = "riskLevel")]
    pub risk_level: String,
}

use crate::binance::options::{PrivateRestClient, RestResult};

impl PrivateRestClient {
    /// Get current account information
    ///
    /// # Arguments
    /// * `request` - Account request parameters (optional recv_window)
    ///
    /// # Returns
    /// Account information including assets and Greeks
    ///
    /// # Weight
    /// 3
    pub async fn get_account(&self, request: AccountRequest) -> RestResult<AccountResponse> {
        self.send_signed_request(
            "/eapi/v1/account",
            reqwest::Method::GET,
            request,
            3, // weight
            false, // not an order
        )
        .await
    }


}