use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const ACCOUNT_LEDGERS_ENDPOINT: &str = "/api/v1/accounts/ledgers";

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

impl RestClient {
    /// Get account ledgers (transaction history)
    ///
    /// Reference: https://docs.kucoin.com/#get-account-ledgers
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
            self.get(ACCOUNT_LEDGERS_ENDPOINT, Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_ledgers_request_default() {
        let request = GetAccountLedgersRequest::default();
        assert!(request.currency.is_none());
        assert!(request.direction.is_none());
    }
}
