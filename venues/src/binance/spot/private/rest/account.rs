use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

/// Request parameters for getting account information
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountRequest {
    /// Omit zero balances in the response
    #[serde(rename = "omitZeroBalances", skip_serializing_if = "Option::is_none")]
    pub omit_zero_balances: Option<bool>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account information response
#[derive(Debug, Clone, Deserialize)]
pub struct AccountResponse {
    /// Maker commission rate (bips)
    #[serde(rename = "makerCommission")]
    pub maker_commission: u32,

    /// Taker commission rate (bips)
    #[serde(rename = "takerCommission")]
    pub taker_commission: u32,

    /// Buyer commission rate (bips)
    #[serde(rename = "buyerCommission")]
    pub buyer_commission: u32,

    /// Seller commission rate (bips)
    #[serde(rename = "sellerCommission")]
    pub seller_commission: u32,

    /// Commission rates
    #[serde(rename = "commissionRates")]
    pub commission_rates: CommissionRates,

    /// Can trade
    #[serde(rename = "canTrade")]
    pub can_trade: bool,

    /// Can withdraw
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,

    /// Can deposit
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,

    /// Buy BNB enabled
    #[serde(rename = "brokered")]
    pub brokered: bool,

    /// Require self trade prevention
    #[serde(rename = "requireSelfTradePrevention")]
    pub require_self_trade_prevention: bool,

    /// Prevent SOR
    #[serde(rename = "preventSor")]
    pub prevent_sor: bool,

    /// Update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: String,

    /// Account balances
    #[serde(rename = "balances")]
    pub balances: Vec<Balance>,

    /// Account permissions
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,

    /// UID
    #[serde(rename = "uid")]
    pub uid: u64,
}

/// Commission rates information
#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRates {
    /// Maker commission rate
    #[serde(rename = "maker")]
    pub maker: Decimal,

    /// Taker commission rate
    #[serde(rename = "taker")]
    pub taker: Decimal,

    /// Buyer commission rate
    #[serde(rename = "buyer")]
    pub buyer: Decimal,

    /// Seller commission rate
    #[serde(rename = "seller")]
    pub seller: Decimal,
}

/// Account balance information
#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    /// Asset name
    #[serde(rename = "asset")]
    pub asset: String,

    /// Free balance
    #[serde(rename = "free")]
    pub free: Decimal,

    /// Locked balance
    #[serde(rename = "locked")]
    pub locked: Decimal,
}

impl RestClient {
    /// Get current account information
    ///
    /// Get current account information.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#account-information--user_data)
    /// Method: GET /api/v3/account
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_account(&self, params: Option<AccountRequest>) -> RestResult<AccountResponse> {
        let query_string = if let Some(p) = params {
            if p.omit_zero_balances.is_some() || p.recv_window.is_some() {
                Some(serde_urlencoded::to_string(&p).map_err(|e| {
                    crate::binance::spot::Errors::Error(format!("URL encoding error: {e}"))
                })?)
            } else {
                None
            }
        } else {
            None
        };

        self.send_request(
            "/api/v3/account",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            20,
            false,
        )
        .await
    }
}
