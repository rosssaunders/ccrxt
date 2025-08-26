use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const AGENCY_TRANSACTION_HISTORY_ENDPOINT: &str = "/rebate/agency/transaction_history";

/// Broker transaction history request parameters
#[derive(Debug, Clone, Serialize, Default)]
pub struct AgencyTransactionHistoryRequest {
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

/// Agency transaction history response
#[derive(Debug, Clone, Deserialize)]
pub struct AgencyTransactionHistoryResponse {
    /// Total number of records
    pub total: i64,

    /// List of transaction records
    pub list: Vec<AgencyTransaction>,
}

/// Individual transaction record
#[derive(Debug, Clone, Deserialize)]
pub struct AgencyTransaction {
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
    /// Broker Obtains Transaction History of Recommended Users
    ///
    /// Query time range is limited to 30 days
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#broker-obtains-transaction-history-of-recommended-users)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Transaction history request parameters with optional filters
    ///
    /// # Returns
    /// Transaction history response with total count and transaction list
    pub async fn get_agency_transaction_history(
        &self,
        req: Option<AgencyTransactionHistoryRequest>,
    ) -> RestResult<AgencyTransactionHistoryResponse> {
        self.send_get_request(AGENCY_TRANSACTION_HISTORY_ENDPOINT, req.as_ref())
            .await
    }
}
