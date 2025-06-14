use super::client::RestClient;
use crate::coinbase::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Account balance information from Coinbase Exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    /// Account ID
    pub id: String,
    /// Currency code (e.g., "BTC", "USD")
    pub currency: String,
    /// Current balance
    pub balance: String,
    /// Amount on hold
    pub hold: String,
    /// Available balance (balance - hold)
    pub available: String,
    /// Profile ID this account belongs to
    pub profile_id: String,
    /// Whether this is a trading account
    pub trading_enabled: bool,
}

/// Request parameters for getting account balances
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAccountBalancesRequest {
    /// Optional cursor for pagination
    pub cursor: Option<String>,
    /// Number of results per page (default 100, max 250)
    pub limit: Option<u32>,
}

/// Response for get account balances endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountBalancesResponse {
    /// List of account balances
    pub accounts: Vec<AccountBalance>,
    /// Pagination information
    pub pagination: Option<PaginationInfo>,
}

/// Pagination information for responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    /// Cursor for the next page
    pub after: Option<String>,
    /// Whether there are more results
    pub has_next: bool,
}

impl RestClient {
    /// Get account balances for the authenticated user
    ///
    /// Returns a list of all accounts and their balances for the authenticated user.
    /// This endpoint requires the "view" permission.
    ///
    /// See: https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getaccounts
    ///
    /// Rate limit: 15 requests/second (private endpoint)
    ///
    /// # Arguments
    /// * `request` - Request parameters including optional pagination
    ///
    /// # Returns
    /// Account balance information for all accounts
    pub async fn get_account_balances(
        &self,
        request: &GetAccountBalancesRequest,
    ) -> RestResult<GetAccountBalancesResponse> {
        let accounts: Vec<AccountBalance> = self
            .send_request(
                "accounts",
                reqwest::Method::GET,
                Some(request),
                EndpointType::Private,
            )
            .await?;

        // Note: The actual Coinbase API returns an array directly,
        // but we wrap it in a response structure for consistency
        // In a real implementation, you'd need to handle pagination headers
        Ok(GetAccountBalancesResponse {
            accounts,
            pagination: None, // TODO: Extract from response headers
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_account_balance_deserialization() {
        let json = r#"
        {
            "id": "71452118-efc7-4cc4-8780-a5e22d4baa53",
            "currency": "BTC",
            "balance": "1.100000000000",
            "hold": "0.100000000000",
            "available": "1.000000000000",
            "profile_id": "75da88c5-05bf-4f54-bc85-5c775bd68254",
            "trading_enabled": true
        }"#;

        let account: AccountBalance = serde_json::from_str(json).unwrap();
        assert_eq!(account.id, "71452118-efc7-4cc4-8780-a5e22d4baa53");
        assert_eq!(account.currency, "BTC");
        assert_eq!(account.balance, "1.100000000000");
        assert_eq!(account.hold, "0.100000000000");
        assert_eq!(account.available, "1.000000000000");
        assert_eq!(account.profile_id, "75da88c5-05bf-4f54-bc85-5c775bd68254");
        assert!(account.trading_enabled);
    }

    #[test]
    fn test_get_account_balances_request_serialization() {
        let request = GetAccountBalancesRequest {
            cursor: Some("cursor123".to_string()),
            limit: Some(50),
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.contains("cursor=cursor123"));
        assert!(query_string.contains("limit=50"));
    }

    #[test]
    fn test_empty_request_serialization() {
        let request = GetAccountBalancesRequest::default();
        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.is_empty());
    }
}