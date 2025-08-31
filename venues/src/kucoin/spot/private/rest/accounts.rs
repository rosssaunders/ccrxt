use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

const ACCOUNTS_ENDPOINT: &str = "/api/v1/accounts";

/// Request for getting all accounts
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccountsRequest {
    /// Currency code filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Account type filter (optional)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
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
    /// [docs](https://docs.kucoin.com/#list-accounts)
    pub async fn get_accounts(
        &self,
        request: GetAccountsRequest,
    ) -> Result<(Vec<Account>, ResponseHeaders)> {
        let (response, headers): (RestResponse<Vec<Account>>, ResponseHeaders) =
            self.get_with_request(ACCOUNTS_ENDPOINT, &request).await?;

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
