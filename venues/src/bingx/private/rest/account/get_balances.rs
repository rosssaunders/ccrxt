use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const BALANCES_ENDPOINT: &str = "/openApi/spot/v1/account/balance";

/// Request to get account balances
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalancesRequest {
    /// Request valid time window value, Unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request, Unit: milliseconds (required)
    pub timestamp: i64,
}

/// Response from the get balances endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetBalancesResponse {
    /// Asset list
    pub balances: Vec<Balance>,
}

/// Balance information for a specific asset
#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    /// Asset name (e.g., "BTC", "USDT")
    pub asset: String,

    /// Display name for the asset
    #[serde(rename = "displayName")]
    pub display_name: String,

    /// Available asset amount
    pub free: String,

    /// Frozen/locked asset amount
    pub locked: String,
}

impl RestClient {
    /// Query Assets
    ///
    /// Retrieves the account balance for all assets in the spot/fund account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/account-api.html#Query%20Assets)
    ///
    /// Rate limit: UID 5/s & IP rate limit group 3
    ///
    /// # Arguments
    /// * `request` - The get balances request (can be empty for default parameters)
    ///
    /// # Returns
    /// A result containing the account balances or an error
    pub async fn get_balances(
        &self,
        request: &GetBalancesRequest,
    ) -> RestResult<GetBalancesResponse> {
        self.send_get_signed_request(BALANCES_ENDPOINT, request, EndpointType::Account)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_balances_request_serialization() {
        let request = GetBalancesRequest {
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_get_balances_request_minimal() {
        let request = GetBalancesRequest {
            recv_window: None,
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1640995200000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_balance_deserialization() {
        let json = r#"{
            "balances": [
                {
                    "asset": "BTC",
                    "displayName": "Bitcoin",
                    "free": "1.00000000",
                    "locked": "0.00000000"
                },
                {
                    "asset": "USDT",
                    "displayName": "Tether USD",
                    "free": "1000.50000000",
                    "locked": "50.00000000"
                }
            ]
        }"#;

        let response: GetBalancesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.balances.len(), 2);

        let btc_balance = &response.balances[0];
        assert_eq!(btc_balance.asset, "BTC");
        assert_eq!(btc_balance.display_name, "Bitcoin");
        assert_eq!(btc_balance.free, "1.00000000");
        assert_eq!(btc_balance.locked, "0.00000000");

        let usdt_balance = &response.balances[1];
        assert_eq!(usdt_balance.asset, "USDT");
        assert_eq!(usdt_balance.display_name, "Tether USD");
        assert_eq!(usdt_balance.free, "1000.50000000");
        assert_eq!(usdt_balance.locked, "50.00000000");
    }
}
