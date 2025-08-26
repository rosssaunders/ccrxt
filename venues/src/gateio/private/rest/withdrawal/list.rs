use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const WITHDRAWALS_ENDPOINT: &str = "/withdrawals";

/// Request parameters for querying withdrawal history
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetWithdrawalsRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time filter (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Page size limit (max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Withdrawal record information
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawalRecord {
    /// Withdrawal record ID
    pub id: String,

    /// Withdrawal currency
    pub currency: String,

    /// Withdrawal amount
    pub amount: String,

    /// Destination address
    pub address: String,

    /// Transaction hash (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,

    /// Blockchain network
    pub chain: String,

    /// Current withdrawal status
    pub status: String,

    /// Withdrawal fee
    pub fee: String,

    /// Creation timestamp
    pub create_time: i64,

    /// Memo or tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

impl RestClient {
    /// Get Withdrawal Records
    ///
    /// Retrieve withdrawal history with filtering and pagination support.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-withdrawal-records)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Optional request parameters for filtering and pagination
    ///
    /// # Returns
    /// List of withdrawal records with transaction details
    pub async fn get_withdrawals(
        &self,
        req: Option<GetWithdrawalsRequest>,
    ) -> RestResult<Vec<WithdrawalRecord>> {
        self.send_get_request(WITHDRAWALS_ENDPOINT, req.as_ref())
            .await
    }
}
