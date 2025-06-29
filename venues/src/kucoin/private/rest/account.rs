use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

/// Request for getting account balance
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccountBalanceRequest {
    /// Currency code (optional, if not provided returns all balances)
    pub currency: Option<String>,
    /// Account type (optional)
    #[serde(rename = "type")]
    pub account_type: Option<String>,
}

/// Account balance information
#[derive(Debug, Clone, Deserialize)]
pub struct AccountBalance {
    /// Currency code
    pub currency: String,
    /// Total balance
    pub balance: String,
    /// Available balance
    pub available: String,
    /// Balance held (in orders)
    pub holds: String,
}

/// Request for getting all accounts
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccountsRequest {
    /// Currency code filter (optional)
    pub currency: Option<String>,
    /// Account type filter (optional)
    #[serde(rename = "type")]
    pub account_type: Option<String>,
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

/// Request for getting account ledgers (transaction history)
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccountLedgersRequest {
    /// Currency filter (optional)
    pub currency: Option<String>,
    /// Direction (optional): in, out
    pub direction: Option<String>,
    /// Business type filter (optional)
    #[serde(rename = "bizType")]
    pub business_type: Option<String>,
    /// Start time (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,
    /// End time (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
}

/// Account ledger entry
#[derive(Debug, Clone, Deserialize)]
pub struct AccountLedger {
    /// Ledger ID
    pub id: String,
    /// Currency
    pub currency: String,
    /// Amount (positive for income, negative for outcome)
    pub amount: String,
    /// Fee
    pub fee: String,
    /// Balance after this transaction
    pub balance: String,
    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Business type
    #[serde(rename = "bizType")]
    pub business_type: String,
    /// Direction (in/out)
    pub direction: String,
    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// Context (additional information)
    pub context: Option<serde_json::Value>,
}

/// Response wrapper for paginated ledger data
#[derive(Debug, Clone, Deserialize)]
pub struct AccountLedgersResponse {
    /// Current page
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    /// Page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// Total number of records
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: i32,
    /// Ledger items
    pub items: Vec<AccountLedger>,
}

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
    /// Get account balance for a specific currency or all currencies
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetAccountBalanceRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetAccountBalanceRequest {
    ///         currency: Some("BTC".to_string()),
    ///         account_type: Some("trade".to_string()),
    ///     };
    ///     let (balances, _headers) = client.get_account_balance(request).await?;
    ///     for balance in balances {
    ///         println!("Currency: {}, Available: {}", balance.currency, balance.available);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_account_balance(
        &self,
        request: GetAccountBalanceRequest,
    ) -> Result<(Vec<AccountBalance>, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(account_type) = request.account_type {
            params.insert("type".to_string(), account_type);
        }

        let (response, headers): (RestResponse<Vec<AccountBalance>>, ResponseHeaders) =
            self.get("/api/v1/accounts", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get all accounts
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetAccountsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetAccountsRequest::default();
    ///     let (accounts, _headers) = client.get_accounts(request).await?;
    ///     println!("Found {} accounts", accounts.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_accounts(
        &self,
        request: GetAccountsRequest,
    ) -> Result<(Vec<Account>, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(account_type) = request.account_type {
            params.insert("type".to_string(), account_type);
        }

        let (response, headers): (RestResponse<Vec<Account>>, ResponseHeaders) =
            self.get("/api/v1/accounts", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get account ledgers (transaction history)
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetAccountLedgersRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetAccountLedgersRequest {
    ///         currency: Some("BTC".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let (ledgers, _headers) = client.get_account_ledgers(request).await?;
    ///     println!("Found {} ledger entries", ledgers.items.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_account_ledgers(
        &self,
        request: GetAccountLedgersRequest,
    ) -> Result<(AccountLedgersResponse, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(direction) = request.direction {
            params.insert("direction".to_string(), direction);
        }
        if let Some(business_type) = request.business_type {
            params.insert("bizType".to_string(), business_type);
        }
        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }

        let (response, headers): (RestResponse<AccountLedgersResponse>, ResponseHeaders) =
            self.get("/api/v1/accounts/ledgers", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get sub-account balance (for main accounts)
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetSubAccountBalanceRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetSubAccountBalanceRequest {
    ///         sub_user_id: "sub_account_id".to_string(),
    ///         include_base_amount: Some(true),
    ///     };
    ///     let (balance, _headers) = client.get_sub_account_balance(request).await?;
    ///     println!("Sub-account: {}", balance.sub_name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_sub_account_balance(
        &self,
        request: GetSubAccountBalanceRequest,
    ) -> Result<(SubAccountBalance, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("subUserId".to_string(), request.sub_user_id);

        if let Some(include_base_amount) = request.include_base_amount {
            params.insert(
                "includeBaseAmount".to_string(),
                include_base_amount.to_string(),
            );
        }

        let (response, headers): (RestResponse<SubAccountBalance>, ResponseHeaders) =
            self.get("/api/v1/sub-accounts", Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_balance_request_creation() {
        let request = GetAccountBalanceRequest {
            currency: Some("BTC".to_string()),
            account_type: Some("trade".to_string()),
        };
        assert_eq!(request.currency, Some("BTC".to_string()));
        assert_eq!(request.account_type, Some("trade".to_string()));
    }

    #[test]
    fn test_accounts_request_default() {
        let request = GetAccountsRequest::default();
        assert!(request.currency.is_none());
        assert!(request.account_type.is_none());
    }

    #[test]
    fn test_ledgers_request_creation() {
        let request = GetAccountLedgersRequest {
            currency: Some("USDT".to_string()),
            direction: Some("in".to_string()),
            ..Default::default()
        };
        assert_eq!(request.currency, Some("USDT".to_string()));
        assert_eq!(request.direction, Some("in".to_string()));
    }
}
