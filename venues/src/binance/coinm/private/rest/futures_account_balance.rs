// Futures Account Balance (USER_DATA) endpoint implementation for GET /dapi/v1/balance
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

/// Request parameters for getting futures account balance (GET /dapi/v1/balance).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetFuturesAccountBalanceRequest {
    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Individual balance entry.
#[derive(Debug, Clone, Deserialize)]
pub struct FuturesAccountBalanceEntry {
    /// Unique account code.
    #[serde(rename = "accountAlias")]
    pub account_alias: String,

    /// Asset name.
    pub asset: String,

    /// Total balance.
    pub balance: String,

    /// Withdraw available amount.
    #[serde(rename = "withdrawAvailable")]
    pub withdraw_available: String,

    /// Cross wallet balance.
    #[serde(rename = "crossWalletBalance")]
    pub cross_wallet_balance: String,

    /// Cross unrealized PnL.
    #[serde(rename = "crossUnPnl")]
    pub cross_un_pnl: String,

    /// Available balance.
    #[serde(rename = "availableBalance")]
    pub available_balance: String,

    /// Update time.
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

/// Response for getting futures account balance (GET /dapi/v1/balance).
pub type GetFuturesAccountBalanceResponse = Vec<FuturesAccountBalanceEntry>;

impl RestClient {
    /// Gets futures account balance (USER_DATA) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance>
    /// GET /dapi/v1/balance
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Check futures account balance.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetFuturesAccountBalanceRequest`])
    ///
    /// # Returns
    /// A [`GetFuturesAccountBalanceResponse`] - array of account balance entries.
    pub async fn get_futures_account_balance(
        &self,
        params: GetFuturesAccountBalanceRequest,
    ) -> RestResult<GetFuturesAccountBalanceResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/dapi/v1/balance",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
