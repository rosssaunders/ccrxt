use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const ALL_ACCOUNT_BALANCE_ENDPOINT: &str = "/openApi/account/v1/allAccountBalance";

/// Request parameters for querying asset overview.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllAccountBalanceRequest {
    /// Account type (optional) - if left blank, all assets of the account will be checked by default
    /// spot: spot (fund account), stdFutures: standard futures account, coinMPerp: coin base account,
    /// USDTMPerp: U base account, copyTrading: copy trading account, grid: grid account,
    /// eran: wealth account, c2c: c2c account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,

    /// Request valid time window value, in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request timestamp in milliseconds since epoch (required)
    pub timestamp: i64,
}

/// Account balance information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    /// Account type - spot: spot (fund account), stdFutures: standard futures account,
    /// coinMPerp: coin base account, USDTMPerp: U base account, copyTrading: copy trading account,
    /// grid: grid account, eran: wealth account, c2c: c2c account
    pub account_type: String,

    /// Equivalent to USDT amount
    pub usdt_balance: String,
}

/// Response containing all account balances.
pub type GetAllAccountBalanceResponse = Vec<AccountBalance>;

impl RestClient {
    /// Asset overview
    ///
    /// Retrieves the asset overview for all account types or a specific account type.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/account-api.html#Asset%20overview)
    ///
    /// Rate limit: UID 5/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The query assets request parameters
    ///
    /// # Returns
    /// A result containing the account balances or an error
    pub async fn get_all_account_balance(
        &self,
        request: GetAllAccountBalanceRequest,
    ) -> RestResult<GetAllAccountBalanceResponse> {
        self.send_get_signed_request(
            ALL_ACCOUNT_BALANCE_ENDPOINT,
            &request,
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
            account_type: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_get_all_account_balance_request_serialization_with_recv_window() {
        let request = GetAllAccountBalanceRequest {
            account_type: Some("spot".to_string()),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("accountType=spot"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_get_all_account_balance_response_deserialization() {
        let json = r#"[
            {
                "accountType": "spot",
                "usdtBalance": "1500.50"
            },
            {
                "accountType": "stdFutures", 
                "usdtBalance": "2000.25"
            },
            {
                "accountType": "USDTMPerp",
                "usdtBalance": "750.75"
            }
        ]"#;

        let response: GetAllAccountBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 3);

        let spot_balance = response.first().expect("Missing spot balance in response");
        assert_eq!(spot_balance.account_type, "spot");
        assert_eq!(spot_balance.usdt_balance, "1500.50");

        let futures_balance = response
            .get(1)
            .expect("Missing futures balance in response");
        assert_eq!(futures_balance.account_type, "stdFutures");
        assert_eq!(futures_balance.usdt_balance, "2000.25");

        let perp_balance = response
            .get(2)
            .expect("Missing perpetual balance in response");
        assert_eq!(perp_balance.account_type, "USDTMPerp");
        assert_eq!(perp_balance.usdt_balance, "750.75");
    }

    #[test]
    fn test_account_balance_deserialization() {
        let json = r#"{
            "accountType": "copyTrading",
            "usdtBalance": "500.25"
        }"#;

        let balance: AccountBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.account_type, "copyTrading");
        assert_eq!(balance.usdt_balance, "500.25");
    }

    #[test]
    fn test_default_request() {
        let request = GetAllAccountBalanceRequest::default();
        assert!(request.account_type.is_none());
        assert!(request.recv_window.is_none());
        assert_eq!(request.timestamp, 0);
    }
}
