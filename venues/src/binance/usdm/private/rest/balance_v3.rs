use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const BALANCE_V3_ENDPOINT: &str = "/fapi/v3/balance";

/// Request parameters for the Futures Account Balance V3 endpoint.
///
/// Retrieves the current account balance for all assets, including cross wallet balance, unrealized PnL, and margin availability.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceV3Request {
    /// Request timestamp in milliseconds since epoch. Must be the current server time. Required.
    pub timestamp: u64,

    /// Number of milliseconds after timestamp the request is valid for. Optional. Valid range: 1-60000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Account balance V3 response item for a single asset.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceV3Response {
    /// Unique account code (e.g., "SgsR").
    pub account_alias: String,

    /// Asset name (e.g., "USDT", "BTC").
    pub asset: String,

    /// Wallet balance for the asset.
    pub balance: String,

    /// Crossed wallet balance for the asset.
    pub cross_wallet_balance: String,

    /// Unrealized profit of crossed positions for the asset.
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
    /// Futures Account Balance V3
    ///
    /// Retrieves the current account balance for all assets, including cross wallet balance, unrealized PnL, and margin availability.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Futures-Account-Balance-V3)
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters for the endpoint
    ///
    /// # Returns
    /// Returns a list of balance information for all assets.
    pub async fn get_balance_v3(
        &self,
        params: GetBalanceV3Request,
    ) -> RestResult<Vec<BalanceV3Response>> {
        self.send_get_signed_request(BALANCE_V3_ENDPOINT, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_balance_v3_request_serialization() {
        let request = GetBalanceV3Request {
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_balance_v3_request_serialization_minimal() {
        let request = GetBalanceV3Request {
            timestamp: 1625097600000,
            recv_window: None,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_balance_v3_response_deserialization() {
        let json = r#"[{
            "accountAlias": "SgsR",
            "asset": "USDT",
            "balance": "122607.35137903",
            "crossWalletBalance": "23.72469206",
            "crossUnPnl": "0.00000000",
            "availableBalance": "23.72469206",
            "maxWithdrawAmount": "23.72469206",
            "marginAvailable": true,
            "updateTime": 1617939110373
        }]"#;
        let response: Vec<BalanceV3Response> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        let item = &response[0];
        assert_eq!(item.account_alias, "SgsR");
        assert_eq!(item.asset, "USDT");
        assert_eq!(item.balance, "122607.35137903");
        assert_eq!(item.cross_wallet_balance, "23.72469206");
        assert_eq!(item.cross_un_pnl, "0.00000000");
        assert_eq!(item.available_balance, "23.72469206");
        assert_eq!(item.max_withdraw_amount, "23.72469206");
        assert!(item.margin_available);
        assert_eq!(item.update_time, 1617939110373);
    }

    #[test]
    fn test_balance_v3_response_deserialization_multiple_assets() {
        let json = r#"[
            {
                "accountAlias": "SgsR",
                "asset": "USDT",
                "balance": "1000.00000000",
                "crossWalletBalance": "1000.00000000",
                "crossUnPnl": "0.00000000",
                "availableBalance": "1000.00000000",
                "maxWithdrawAmount": "1000.00000000",
                "marginAvailable": true,
                "updateTime": 1625097600000
            },
            {
                "accountAlias": "SgsR", 
                "asset": "BTC",
                "balance": "0.50000000",
                "crossWalletBalance": "0.50000000",
                "crossUnPnl": "10.50000000",
                "availableBalance": "0.60500000",
                "maxWithdrawAmount": "0.60500000",
                "marginAvailable": false,
                "updateTime": 1625097600001
            }
        ]"#;
        let response: Vec<BalanceV3Response> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        // USDT balance
        assert_eq!(response[0].asset, "USDT");
        assert!(response[0].margin_available);
        // BTC balance
        assert_eq!(response[1].asset, "BTC");
        assert_eq!(response[1].cross_un_pnl, "10.50000000");
        assert!(!response[1].margin_available);
        assert_eq!(response[1].update_time, 1625097600001);
    }

    #[test]
    fn test_balance_v3_response_deserialization_empty() {
        let json = r#"[]"#;
        let response: Vec<BalanceV3Response> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
