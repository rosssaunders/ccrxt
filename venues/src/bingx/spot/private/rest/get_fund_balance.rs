use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const FUND_BALANCE_ENDPOINT: &str = "/openApi/wallets/v1/capital/config/getall";

/// Request to get fund account balance
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundBalanceRequest {
    /// Timestamp of initiating the request, Unit: milliseconds
    /// This will be automatically set by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request, Unit: milliseconds
    pub timestamp: i64,
}

/// Response from the get fund balance endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetFundBalanceResponse {
    /// Asset list
    pub balances: Vec<FundBalance>,
}

/// Fund balance information for a specific asset
#[derive(Debug, Clone, Deserialize)]
pub struct FundBalance {
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
    /// Get fund account balance
    ///
    /// Retrieves the account balance for all assets in the fund account.
    /// Rate limit: 5/s by UID
    ///
    /// # Arguments
    /// * `request` - The get fund balance request (can be empty for default parameters)
    ///
    /// # Returns
    /// A result containing the fund account balances or an error
    pub async fn get_fund_balance(
        &self,
        request: &GetFundBalanceRequest,
    ) -> RestResult<GetFundBalanceResponse> {
        self.send_get_signed_request(FUND_BALANCE_ENDPOINT, request, EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fund_balance_request_serialization() {
        let request = GetFundBalanceRequest {
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1658748648396"));
    }

    #[test]
    fn test_get_fund_balance_request_minimal() {
        let request = GetFundBalanceRequest {
            recv_window: None,
            timestamp: 1658748648396,
        };
        assert!(request.recv_window.is_none());
        assert_eq!(request.timestamp, 1658748648396);
    }

    #[test]
    fn test_fund_balance_deserialization() {
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

        let response: GetFundBalanceResponse = serde_json::from_str(json).unwrap();
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
