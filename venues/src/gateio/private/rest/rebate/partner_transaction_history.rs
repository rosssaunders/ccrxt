use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const PARTNER_TRANSACTION_HISTORY_ENDPOINT: &str = "/rebate/partner/transaction_history";

/// Partner transaction history request parameters
#[derive(Debug, Clone, Serialize, Default)]
pub struct PartnerTransactionHistoryRequest {
    /// Trading pair filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// User ID filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Start time (defaults to 7 days prior if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (defaults to current time if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Partner transaction history response
#[derive(Debug, Clone, Deserialize)]
pub struct PartnerTransactionHistoryResponse {
    /// Total number of records
    pub total: i64,

    /// List of transaction records
    pub list: Vec<PartnerTransaction>,
}

/// Individual partner transaction record
#[derive(Debug, Clone, Deserialize)]
pub struct PartnerTransaction {
    /// Transaction ID
    pub id: String,

    /// User ID
    pub user_id: i64,

    /// Trading pair
    pub currency_pair: String,

    /// Transaction type
    #[serde(rename = "type")]
    pub transaction_type: String,

    /// Transaction amount
    pub amount: String,

    /// Fee amount
    pub fee: String,

    /// Transaction time
    pub create_time: i64,
}

impl RestClient {
    /// Partner Obtains Transaction History of Recommended Users
    ///
    /// Query time range is limited to 30 days
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#partner-obtains-transaction-history-of-recommended-users)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Transaction history request parameters with optional filters
    ///
    /// # Returns
    /// Transaction history response with total count and transaction list
    pub async fn get_partner_transaction_history(
        &self,
        req: Option<PartnerTransactionHistoryRequest>,
    ) -> RestResult<PartnerTransactionHistoryResponse> {
        self.send_get_request(PARTNER_TRANSACTION_HISTORY_ENDPOINT, req.as_ref())
            .await
    }
}
