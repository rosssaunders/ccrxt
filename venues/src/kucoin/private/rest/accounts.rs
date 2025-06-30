use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

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

impl RestClient {
    /// Get all accounts
    ///
    /// Reference: https://docs.kucoin.com/#list-accounts
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounts_request_default() {
        let request = GetAccountsRequest::default();
        assert!(request.currency.is_none());
        assert!(request.account_type.is_none());
    }

    #[test]
    fn test_accounts_request_creation() {
        let request = GetAccountsRequest {
            currency: Some("BTC".to_string()),
            account_type: Some("trade".to_string()),
        };
        assert_eq!(request.currency, Some("BTC".to_string()));
        assert_eq!(request.account_type, Some("trade".to_string()));
    }
}
