use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

/// Account type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    /// Spot (fund account)
    #[serde(rename = "spot")]
    Spot,
    /// Standard futures account
    #[serde(rename = "stdFutures")]
    StdFutures,
    /// Coin base account
    #[serde(rename = "coinMPerp")]
    CoinMPerp,
    /// U base account
    #[serde(rename = "USDTMPerp")]
    UsdtmPerp,
    /// Copy trading account
    #[serde(rename = "copyTrading")]
    CopyTrading,
    /// Grid account
    #[serde(rename = "grid")]
    Grid,
    /// Wealth account
    #[serde(rename = "eran")]
    Eran,
    /// C2C account
    #[serde(rename = "c2c")]
    C2c,
}

/// Request to get all account balance overview
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllAccountBalanceRequest {
    /// Account type (optional - if blank, all assets will be checked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,

    /// Request valid time window value, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
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
#[derive(Debug, Clone)]
pub struct GetAllAccountBalanceResponse {
    /// List of account balance overviews
    pub accounts: Vec<AccountBalanceOverview>,
}

// Custom deserialization since the response is a direct array
impl<'de> Deserialize<'de> for GetAllAccountBalanceResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let accounts = Vec::<AccountBalanceOverview>::deserialize(deserializer)?;
        Ok(GetAllAccountBalanceResponse { accounts })
    }
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
    ///
    /// # Example
    /// ```rust,no_run
    /// use venues::bingx::{PrivateRestClient, GetAllAccountBalanceRequest, AccountType};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client: PrivateRestClient = unimplemented!();
    ///     
    ///     // Get all account balances
    ///     let request = GetAllAccountBalanceRequest::default();
    ///     let all_balances = client.get_all_account_balance(&request).await?;
    ///     println!("Total accounts: {}", all_balances.accounts.len());
    ///     
    ///     // Get specific account type balance
    ///     let request = GetAllAccountBalanceRequest {
    ///         account_type: Some(AccountType::Spot),
    ///         recv_window: None,
    ///     };
    ///     let spot_balance = client.get_all_account_balance(&request).await?;
    ///     println!("Spot balance overview: {:?}", spot_balance);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_all_account_balance(&self, request: &GetAllAccountBalanceRequest) -> RestResult<GetAllAccountBalanceResponse> {
        self.send_request(
            "/openApi/account/v1/allAccountBalance",
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
    fn test_get_all_account_balance_request_serialization_default() {
        let request = GetAllAccountBalanceRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        // Should be empty when default
        assert!(serialized.is_empty() || serialized == "");
    }

    #[test]
    fn test_get_all_account_balance_request_serialization_with_account_type() {
        let request = GetAllAccountBalanceRequest {
            account_type: Some(AccountType::Spot),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("accountType=spot"));
        assert!(serialized.contains("recvWindow=5000"));
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
            serde_json::to_string(&AccountType::UsdtmPerp).unwrap(),
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
            serde_json::to_string(&AccountType::Eran).unwrap(),
            "\"eran\""
        );
        assert_eq!(serde_json::to_string(&AccountType::C2c).unwrap(), "\"c2c\"");
    }

    #[test]
    fn test_get_all_account_balance_response_deserialization() {
        let json = r#"[
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
        ]"#;

        let response: GetAllAccountBalanceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.accounts.len(), 3);

        let spot_account = &response.accounts[0];
        assert!(matches!(spot_account.account_type, AccountType::Spot));
        assert_eq!(spot_account.usdt_balance, "1000.50");

        let futures_account = &response.accounts[1];
        assert!(matches!(
            futures_account.account_type,
            AccountType::StdFutures
        ));
        assert_eq!(futures_account.usdt_balance, "500.25");

        let coinm_account = &response.accounts[2];
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
