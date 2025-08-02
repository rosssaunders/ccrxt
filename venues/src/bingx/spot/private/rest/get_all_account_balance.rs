use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const ALL_ACCOUNT_BALANCE_ENDPOINT: &str = "/openApi/spot/v1/account/balance";

/// Request parameters for querying spot account assets.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllAccountBalanceRequest {
    /// Request valid time window value, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp in milliseconds since epoch.
    pub timestamp: i64,
}

/// Individual asset balance information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalance {
    /// Asset name (e.g., "BTC", "USDT").
    pub asset: String,

    /// Display name for the asset.
    pub display_name: String,

    /// Available (free) balance for the asset.
    pub free: String,

    /// Locked (frozen) balance for the asset.
    pub locked: String,
}

/// Response containing spot account asset balances.
#[derive(Debug, Clone, Deserialize)]
pub struct GetAllAccountBalanceResponse {
    /// List of asset balances in the spot account.
    pub balances: Vec<AssetBalance>,
}

impl RestClient {
    /// Query Assets
    ///
    /// Retrieves the asset balances for the spot account.
    ///
    /// [docs]: https://bingx-api.github.io/docs/#/en-us/spot/account-api.html#Query%20Assets
    ///
    /// Rate limit: 5/s by UID & rate limitation by IP in group Number: 3
    ///
    /// # Arguments
    /// * `request` - The query assets request parameters
    ///
    /// # Returns
    /// A result containing the asset balances or an error
    pub async fn get_all_account_balance(
        &self,
        request: GetAllAccountBalanceRequest,
    ) -> RestResult<GetAllAccountBalanceResponse> {
        self.send_request(
            ALL_ACCOUNT_BALANCE_ENDPOINT,
            reqwest::Method::GET,
            Some(&request),
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_account_balance_request_serialization_with_timestamp() {
        let request = GetAllAccountBalanceRequest {
            recv_window: None,
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_get_all_account_balance_request_serialization_with_recv_window() {
        let request = GetAllAccountBalanceRequest {
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_get_all_account_balance_response_deserialization() {
        let json = r#"{
            "balances": [
                {
                    "asset": "BTC",
                    "displayName": "Bitcoin",
                    "free": "1.5",
                    "locked": "0.0"
                },
                {
                    "asset": "USDT",
                    "displayName": "Tether USD",
                    "free": "1000.50",
                    "locked": "100.25"
                },
                {
                    "asset": "ETH",
                    "displayName": "Ethereum",
                    "free": "5.75",
                    "locked": "1.0"
                }
            ]
        }"#;

        let response: GetAllAccountBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.balances.len(), 3);

        let btc_balance = response
            .balances
            .first()
            .expect("Missing BTC balance in response");
        assert_eq!(btc_balance.asset, "BTC");
        assert_eq!(btc_balance.display_name, "Bitcoin");
        assert_eq!(btc_balance.free, "1.5");
        assert_eq!(btc_balance.locked, "0.0");

        let usdt_balance = response
            .balances
            .get(1)
            .expect("Missing USDT balance in response");
        assert_eq!(usdt_balance.asset, "USDT");
        assert_eq!(usdt_balance.display_name, "Tether USD");
        assert_eq!(usdt_balance.free, "1000.50");
        assert_eq!(usdt_balance.locked, "100.25");

        let eth_balance = response
            .balances
            .get(2)
            .expect("Missing ETH balance in response");
        assert_eq!(eth_balance.asset, "ETH");
        assert_eq!(eth_balance.display_name, "Ethereum");
        assert_eq!(eth_balance.free, "5.75");
        assert_eq!(eth_balance.locked, "1.0");
    }

    #[test]
    fn test_asset_balance_deserialization() {
        let json = r#"{
            "asset": "BTC",
            "displayName": "Bitcoin",
            "free": "1.5",
            "locked": "0.0"
        }"#;

        let balance: AssetBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.asset, "BTC");
        assert_eq!(balance.display_name, "Bitcoin");
        assert_eq!(balance.free, "1.5");
        assert_eq!(balance.locked, "0.0");
    }

    #[test]
    fn test_default_request() {
        let request = GetAllAccountBalanceRequest::default();
        assert!(request.recv_window.is_none());
        assert_eq!(request.timestamp, 0);
    }
}
