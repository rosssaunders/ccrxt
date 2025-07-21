use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const BALANCE_V2_ENDPOINT: &str = "/fapi/v2/balance";

/// Request parameters for the Balance V2 endpoint.
///
/// Retrieves the current account balance for all assets with basic information
/// including cross wallet balance and unrealized PnL.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceV2Request {
    /// Request timestamp in milliseconds since epoch.
    /// Must be the current server time.
    pub timestamp: u64,

    /// Optional receive window (milliseconds). If not set, default is used by API.
    /// Valid range: 1-60000. Used to specify the number of milliseconds after timestamp the request is valid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account balance V2 response item for a single asset.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceV2Response {
    /// Account alias.
    pub account_alias: String,

    /// Asset name (e.g., "USDT", "BTC").
    pub asset: String,

    /// Account balance for the asset.
    pub balance: String,

    /// Cross wallet balance for the asset.
    pub cross_wallet_balance: String,

    /// Cross unrealized PnL for the asset.
    pub cross_un_pnl: String,

    /// Available balance for the asset.
    pub available_balance: String,

    /// Maximum amount for transfer out for the asset.
    pub max_withdraw_amount: String,

    /// Whether the asset can be used as margin in Multi-Assets mode.
    pub margin_available: bool,

    /// Last update time (timestamp in milliseconds).
    pub update_time: u64,
}

impl UsdmClient {
    /// Future Account Balance V2 (GET /fapi/v2/balance)
    ///
    /// Retrieves the current account balance for all assets with basic information
    /// including cross wallet balance and unrealized PnL.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Futures-Account-Balance-V2
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// Vec<BalanceV2Response> - List of balance information for all assets
    pub async fn get_balance_v2(
        &self,
        params: GetBalanceV2Request,
    ) -> RestResult<Vec<BalanceV2Response>> {
        self.send_signed_request(BALANCE_V2_ENDPOINT, Method::GET, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_balance_v2_request_serialization() {
        let request = GetBalanceV2Request {
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_balance_v2_request_serialization_minimal() {
        let request = GetBalanceV2Request {
            timestamp: 1625097600000,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_balance_v2_response_deserialization() {
        let json = r#"[{
            "accountAlias": "futures",
            "asset": "USDT",
            "balance": "1000.00000000",
            "crossWalletBalance": "1000.00000000",
            "crossUnPnl": "0.00000000",
            "availableBalance": "1000.00000000",
            "maxWithdrawAmount": "1000.00000000",
            "marginAvailable": true,
            "updateTime": 1625097600000
        }]"#;

        let response: Vec<BalanceV2Response> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].asset, "USDT");
        assert_eq!(response[0].balance, "1000.00000000");
        assert_eq!(response[0].account_alias, "futures");
        assert!(response[0].margin_available);
        assert_eq!(response[0].update_time, 1625097600000);
    }
}
