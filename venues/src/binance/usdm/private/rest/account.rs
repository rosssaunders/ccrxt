//! Account information, balance, and config endpoints for Binance USDM REST API.

use std::borrow::Cow;

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{private::rest::client::RestClient, signing::sign_query};

#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum AccountError {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("Account error: {0}")]
    Account(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<AccountErrorResponse> for AccountError {
    fn from(e: AccountErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => AccountError::InvalidKey(e.msg),
            -1003 => AccountError::RateLimit(e.msg),
            _ => AccountError::Other(e.msg),
        }
    }
}

pub type AccountResult<T> = std::result::Result<T, AccountError>;

/// Request for getting account information.
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountInfoRequest {
    #[serde(skip_serializing)]
    pub api_key: SecretString,
    #[serde(skip_serializing)]
    pub api_secret: SecretString,
}

/// Response for account information.
#[derive(Debug, Clone, Deserialize)]
pub struct AccountInfo {
    pub fee_tier: u8,
    pub can_trade: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub update_time: u64,
    pub total_initial_margin: Cow<'static, str>,
    pub total_maint_margin: Cow<'static, str>,
    pub total_wallet_balance: Cow<'static, str>,
    pub total_unrealized_profit: Cow<'static, str>,
    pub total_margin_balance: Cow<'static, str>,
    pub total_position_initial_margin: Cow<'static, str>,
    pub total_open_order_initial_margin: Cow<'static, str>,
    pub total_cross_wallet_balance: Cow<'static, str>,
    pub total_cross_un_pnl: Cow<'static, str>,
    pub available_balance: Cow<'static, str>,
    pub max_withdraw_amount: Cow<'static, str>,
}

impl RestClient {
    /// Get account information (GET /fapi/v2/account)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#account-information-v2-user_data)
    pub async fn get_account_info(
        &self,
        params: GetAccountInfoRequest,
    ) -> AccountResult<AccountInfo> {
        use tracing::debug;

        use crate::binance::usdm::request::execute_request;
        let endpoint = "/fapi/v2/account";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Build query string
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;
        let query_pairs = format!("timestamp={timestamp}&recvWindow={recv_window}");

        // 2. Sign
        let signature = sign_query(&query_pairs, &params.api_secret);
        let full_query = format!("{}&signature={signature}", query_pairs);

        // 3. Headers
        let headers = vec![("X-MBX-APIKEY", params.api_key.expose_secret().to_string())];

        // 4. Rate limiting
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| AccountError::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending get account info request");

        // 5. Execute
        let full_url = format!("{}?{}", url, full_query);
        let resp =
            execute_request::<AccountInfo>(&self.client, &full_url, method, Some(headers), None)
                .await
                .map_err(|e| match e {
                    crate::binance::usdm::Errors::ApiError(api_err) => {
                        AccountError::Other(format!("API error: {api_err}"))
                    }
                    crate::binance::usdm::Errors::HttpError(http_err) => {
                        AccountError::Other(format!("HTTP error: {http_err}"))
                    }
                    crate::binance::usdm::Errors::Error(msg) => AccountError::Other(msg),
                    crate::binance::usdm::Errors::InvalidApiKey() => {
                        AccountError::InvalidKey("Invalid API key or signature".to_string())
                    }
                })?;

        Ok(resp.data)
    }
}
