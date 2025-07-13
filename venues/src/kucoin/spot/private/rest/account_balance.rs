use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const ACCOUNT_BALANCE_ENDPOINT: &str = "/api/v1/accounts";

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

impl RestClient {
    /// Get account balance for a specific currency or all currencies
    ///
    /// Reference: https://docs.kucoin.com/#list-accounts
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
            self.get(ACCOUNT_BALANCE_ENDPOINT, Some(params)).await?;

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
    fn test_account_balance_request_default() {
        let request = GetAccountBalanceRequest::default();
        assert!(request.currency.is_none());
        assert!(request.account_type.is_none());
    }
}
