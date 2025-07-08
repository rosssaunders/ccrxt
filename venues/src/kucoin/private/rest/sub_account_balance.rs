use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::{ResponseHeaders, RestResponse, Result};

const SUB_ACCOUNT_BALANCE_ENDPOINT: &str = "/api/v1/sub-accounts/{subUserId}";

/// Request for getting sub-account balance (if main account)
#[derive(Debug, Clone, Serialize)]
pub struct GetSubAccountBalanceRequest {
    /// Sub-account user ID
    #[serde(rename = "subUserId")]
    pub sub_user_id: String,

    /// Include sub-account details flag (optional)
    #[serde(rename = "includeBaseAmount")]
    pub include_base_amount: Option<bool>,
}

/// Account information
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    /// Account ID
    pub id: String,

    /// Currency
    pub currency: String,

    /// Account type (main, trade, etc.)
    #[serde(rename = "type")]
    pub account_type: String,

    /// Balance
    pub balance: String,

    /// Available balance
    pub available: String,

    /// Held balance
    pub holds: String,
}

/// Sub-account balance information
#[derive(Debug, Clone, Deserialize)]
pub struct SubAccountBalance {
    /// Sub-account user ID
    #[serde(rename = "subUserId")]
    pub sub_user_id: String,

    /// Sub-account name
    #[serde(rename = "subName")]
    pub sub_name: String,

    /// Main accounts
    #[serde(rename = "mainAccounts")]
    pub main_accounts: Vec<Account>,

    /// Trade accounts
    #[serde(rename = "tradeAccounts")]
    pub trade_accounts: Vec<Account>,

    /// Margin accounts
    #[serde(rename = "marginAccounts")]
    pub margin_accounts: Vec<Account>,
}

impl RestClient {
    /// Get sub-account balance (for main accounts)
    ///
    /// Reference: https://docs.kucoin.com/#get-sub-account-balance
    pub async fn get_sub_account_balance(
        &self,
        request: GetSubAccountBalanceRequest,
    ) -> Result<(SubAccountBalance, ResponseHeaders)> {
        let mut params = HashMap::new();
        if let Some(include_base_amount) = request.include_base_amount {
            params.insert(
                "includeBaseAmount".to_string(),
                include_base_amount.to_string(),
            );
        }
        let endpoint = SUB_ACCOUNT_BALANCE_ENDPOINT.replace("{subUserId}", &request.sub_user_id);
        let (response, headers): (RestResponse<SubAccountBalance>, ResponseHeaders) =
            self.get(&endpoint, Some(params)).await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_account_balance_request_creation() {
        let request = GetSubAccountBalanceRequest {
            sub_user_id: "test_sub_user".to_string(),
            include_base_amount: Some(true),
        };
        assert_eq!(request.sub_user_id, "test_sub_user");
        assert_eq!(request.include_base_amount, Some(true));
    }

    #[test]
    fn test_sub_account_balance_request_minimal() {
        let request = GetSubAccountBalanceRequest {
            sub_user_id: "test_sub_user".to_string(),
            include_base_amount: None,
        };
        assert_eq!(request.sub_user_id, "test_sub_user");
        assert!(request.include_base_amount.is_none());
    }
}
