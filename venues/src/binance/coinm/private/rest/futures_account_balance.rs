// Futures Account Balance (USER_DATA) endpoint implementation for GET /dapi/v1/balance
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Futures-Account-Balance>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

const BALANCE_ENDPOINT: &str = "/dapi/v1/balance";

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
            BALANCE_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_futures_account_balance_request_serialization() {
        let request = GetFuturesAccountBalanceRequest {
            recv_window: None,
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timestamp=1625097600000");
    }

    #[test]
    fn test_get_futures_account_balance_request_serialization_with_recv_window() {
        let request = GetFuturesAccountBalanceRequest {
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_futures_account_balance_response_deserialization() {
        let json = r#"[
            {
                "accountAlias": "SgsR",
                "asset": "BTC",
                "balance": "0.00250000",
                "withdrawAvailable": "0.00250000",
                "crossWalletBalance": "0.00241969",
                "crossUnPnl": "0.00000000",
                "availableBalance": "0.00241969",
                "updateTime": 1625097600000
            },
            {
                "accountAlias": "SgsR",
                "asset": "ETH",
                "balance": "0.10000000",
                "withdrawAvailable": "0.10000000",
                "crossWalletBalance": "0.09950000",
                "crossUnPnl": "-0.00050000",
                "availableBalance": "0.09900000",
                "updateTime": 1625097600001
            }
        ]"#;
        let response: GetFuturesAccountBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        
        let btc_balance = &response[0];
        assert_eq!(btc_balance.account_alias, "SgsR");
        assert_eq!(btc_balance.asset, "BTC");
        assert_eq!(btc_balance.balance, "0.00250000");
        assert_eq!(btc_balance.withdraw_available, "0.00250000");
        assert_eq!(btc_balance.cross_wallet_balance, "0.00241969");
        assert_eq!(btc_balance.cross_un_pnl, "0.00000000");
        assert_eq!(btc_balance.available_balance, "0.00241969");
        assert_eq!(btc_balance.update_time, 1625097600000);
        
        let eth_balance = &response[1];
        assert_eq!(eth_balance.asset, "ETH");
        assert_eq!(eth_balance.balance, "0.10000000");
        assert_eq!(eth_balance.cross_un_pnl, "-0.00050000");
    }

    #[test]
    fn test_futures_account_balance_response_deserialization_empty() {
        let json = r#"[]"#;
        let response: GetFuturesAccountBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
