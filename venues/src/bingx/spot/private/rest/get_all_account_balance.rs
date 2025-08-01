use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult, enums::AccountType};

const ALL_ACCOUNT_BALANCE_ENDPOINT: &str = "/openApi/spot/v1/account/balance";

/// Request to get all account balance overview
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllAccountBalanceRequest {
    /// Account type (optional - if blank, all assets will be checked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,

    /// Request valid time window value, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp for this request
    pub timestamp: i64,
}

/// Account balance overview item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalanceOverview {
    /// Account type
    pub account_type: AccountType,

    /// Equivalent to USDT amount
    pub usdt_balance: String,
}

/// Response from getting all account balance overview
#[derive(Debug, Clone, Deserialize)]
pub struct GetAllAccountBalanceResponse {
    /// List of account balance overviews
    pub accounts: Vec<AccountBalanceOverview>,
}

impl RestClient {
    /// Get all account balance overview
    ///
    /// Retrieves the balance overview for all account types or a specific account type.
    /// Rate limit: 5/s by UID
    ///
    /// # Arguments
    /// * `request` - The get all account balance request (account type is optional)
    ///
    /// # Returns
    /// A result containing the account balance overview or an error
    pub async fn get_all_account_balance(
        &self,
        request: &GetAllAccountBalanceRequest,
    ) -> RestResult<GetAllAccountBalanceResponse> {
        self.send_request(
            ALL_ACCOUNT_BALANCE_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
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
    fn test_get_all_account_balance_request_serialization_with_account_type() {
        let request = GetAllAccountBalanceRequest {
            account_type: Some(AccountType::Spot),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("accountType=spot"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_account_type_serialization() {
        assert_eq!(
            serde_json::to_string(&AccountType::Spot).unwrap(),
            "\"spot\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::StdFutures).unwrap(),
            "\"stdFutures\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::CoinMPerp).unwrap(),
            "\"coinMPerp\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::UsdtMPerp).unwrap(),
            "\"USDTMPerp\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::CopyTrading).unwrap(),
            "\"copyTrading\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::Grid).unwrap(),
            "\"grid\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::Earn).unwrap(),
            "\"earn\""
        );
        assert_eq!(serde_json::to_string(&AccountType::C2C).unwrap(), "\"c2c\"");
    }

    #[test]
    fn test_get_all_account_balance_response_deserialization() {
        let json = r#"{
            "accounts": [
                {
                    "accountType": "spot",
                    "usdtBalance": "1000.50"
                },
                {
                    "accountType": "stdFutures",
                    "usdtBalance": "500.25"
                },
                {
                    "accountType": "coinMPerp",
                    "usdtBalance": "750.75"
                }
            ]
        }"#;

        let response: GetAllAccountBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.accounts.len(), 3);

        let spot_account = response
            .accounts
            .first()
            .expect("Missing spot account in response");
        assert!(matches!(spot_account.account_type, AccountType::Spot));
        assert_eq!(spot_account.usdt_balance, "1000.50");

        let futures_account = response
            .accounts
            .get(1)
            .expect("Missing stdFutures account in response");
        assert!(matches!(
            futures_account.account_type,
            AccountType::StdFutures
        ));
        assert_eq!(futures_account.usdt_balance, "500.25");

        let coinm_account = response
            .accounts
            .get(2)
            .expect("Missing coinMPerp account in response");
        assert!(matches!(coinm_account.account_type, AccountType::CoinMPerp));
        assert_eq!(coinm_account.usdt_balance, "750.75");
    }

    #[test]
    fn test_account_balance_overview_deserialization() {
        let json = r#"{
            "accountType": "spot",
            "usdtBalance": "1000.50"
        }"#;

        let overview: AccountBalanceOverview = serde_json::from_str(json).unwrap();
        assert!(matches!(overview.account_type, AccountType::Spot));
        assert_eq!(overview.usdt_balance, "1000.50");
    }
}
