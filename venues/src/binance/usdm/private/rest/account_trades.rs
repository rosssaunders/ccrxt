//! Account trades endpoint for Binance USDM REST API.

use std::borrow::Cow;

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_urlencoded;
use thiserror::Error;

use crate::binance::usdm::{enums::*, private::rest::client::RestClient, signing::sign_query};

#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum AccountTradesError {
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    #[error("Account trades error: {0}")]
    AccountTrades(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountTradesErrorResponse {
    pub code: i64,
    pub msg: String,
}

impl From<AccountTradesErrorResponse> for AccountTradesError {
    fn from(e: AccountTradesErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => AccountTradesError::InvalidKey(e.msg),
            -1003 => AccountTradesError::RateLimit(e.msg),
            _ => AccountTradesError::Other(e.msg),
        }
    }
}

pub type AccountTradesResult<T> = std::result::Result<T, AccountTradesError>;

/// Request for getting account trades.
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountTradesRequest {
    #[serde(skip_serializing)]
    pub api_key: SecretString,
    #[serde(skip_serializing)]
    pub api_secret: SecretString,
    pub symbol: Cow<'static, str>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>,
    pub limit: Option<u32>,
}

/// Response for a single account trade.
#[derive(Debug, Clone, Deserialize)]
pub struct AccountTrade {
    pub id: u64,
    pub order_id: u64,
    pub symbol: Cow<'static, str>,
    pub price: Cow<'static, str>,
    pub qty: Cow<'static, str>,
    pub commission: Cow<'static, str>,
    pub commission_asset: Cow<'static, str>,
    pub time: u64,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub buyer: bool,
    pub maker: bool,
}

impl RestClient {
    /// Get account trade list (GET /fapi/v1/userTrades)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#account-trade-list-user_data)
    pub async fn get_account_trades(
        &self,
        params: GetAccountTradesRequest,
    ) -> AccountTradesResult<Vec<AccountTrade>> {
        use tracing::debug;

        use crate::binance::usdm::request::execute_request;
        let endpoint = "/fapi/v1/userTrades";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 1. Serialize params to query string (excluding api_key/api_secret)
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| AccountTradesError::Other(format!("Failed to serialize params: {e}")))?;
        if !query_pairs.is_empty() {
            query_pairs.push('&');
        }
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;
        query_pairs.push_str(&format!("timestamp={timestamp}&recvWindow={recv_window}"));

        // 2. Sign
        let signature = sign_query(&query_pairs, &params.api_secret);
        query_pairs.push_str(&format!("&signature={signature}"));

        // 3. Headers
        let headers = vec![("X-MBX-APIKEY", params.api_key.expose_secret().to_string())];

        // 4. Rate limiting
        self.rate_limiter
            .acquire_request(5)
            .await
            .map_err(|e| AccountTradesError::Other(format!("Rate limiting error: {e}")))?;
        debug!(endpoint = endpoint, "Sending get account trades request");

        // 5. Execute
        let full_url = format!("{}?{}", url, query_pairs);
        let resp = execute_request::<Vec<AccountTrade>>(
            &self.client,
            &full_url,
            method,
            Some(headers),
            None,
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                AccountTradesError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                AccountTradesError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => AccountTradesError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                AccountTradesError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}
