use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::coinbaseexchange::{EndpointType, RestResult};

const ACCOUNTS_ENDPOINT: &str = "accounts";

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
    /// Request page before (newer) this pagination id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Request page after (older) this pagination id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Number of results per request. Maximum 1000 (default 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Cursor for the page before (newer page)
    pub before: Option<String>,

    /// Cursor for the page after (older page)
    pub after: Option<String>,
}

impl RestClient {
    /// Get account balances for the authenticated user
    ///
    /// Returns a list of all accounts and their balances for the authenticated user.
    /// This endpoint requires the "view" permission.
    ///
    /// [docs](https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getaccounts)
    ///
    /// Rate limit: 15 requests/second (private endpoint)
    ///
    /// # Arguments
    /// * `request` - Request parameters including optional pagination
    ///
    /// # Returns
    /// Account balance information for all accounts with pagination info
    pub async fn get_account_balances(
        &self,
        request: GetAccountBalancesRequest,
    ) -> RestResult<GetAccountBalancesResponse> {
        let (accounts, pagination) = self
            .send_get_request_with_pagination(ACCOUNTS_ENDPOINT, request, EndpointType::Private)
            .await?;

        Ok(GetAccountBalancesResponse {
            accounts,
            pagination,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

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

        let account: AccountBalance =
            serde_json::from_str(json).expect("Failed to deserialize account balance");
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
            before: Some("before123".to_string()),
            after: Some("after123".to_string()),
            limit: Some(50),
        };

        let query_string =
            serde_urlencoded::to_string(&request).expect("Failed to serialize request");
        assert!(query_string.contains("before=before123"));
        assert!(query_string.contains("after=after123"));
        assert!(query_string.contains("limit=50"));
    }

    #[test]
    fn test_empty_request_serialization() {
        let request = GetAccountBalancesRequest::default();
        let query_string =
            serde_urlencoded::to_string(&request).expect("Failed to serialize request");
        assert!(query_string.is_empty());
    }

    #[test]
    fn test_pagination_info_structure() {
        let pagination = PaginationInfo {
            before: Some("before_cursor".to_string()),
            after: Some("after_cursor".to_string()),
        };

        assert_eq!(pagination.before, Some("before_cursor".to_string()));
        assert_eq!(pagination.after, Some("after_cursor".to_string()));
    }

    #[test]
    fn test_partial_pagination_params() {
        // Test request with only 'before' parameter
        let before_only_request = GetAccountBalancesRequest {
            before: Some("before123".to_string()),
            after: None,
            limit: None,
        };

        let query_string =
            serde_urlencoded::to_string(&before_only_request).expect("Failed to serialize request");
        assert!(query_string.contains("before=before123"));
        assert!(!query_string.contains("after="));
        assert!(!query_string.contains("limit="));

        // Test request with only 'after' parameter
        let after_only_request = GetAccountBalancesRequest {
            before: None,
            after: Some("after456".to_string()),
            limit: Some(100),
        };

        let query_string =
            serde_urlencoded::to_string(&after_only_request).expect("Failed to serialize request");
        assert!(!query_string.contains("before="));
        assert!(query_string.contains("after=after456"));
        assert!(query_string.contains("limit=100"));
    }
}
