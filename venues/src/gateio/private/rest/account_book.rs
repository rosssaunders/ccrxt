use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for getting account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAccountBookRequest {
    /// Retrieve data of the specified currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Start timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of records per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Type of record
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub record_type: Option<String>,
}

/// Account book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBookEntry {
    /// Entry ID
    pub id: String,
    /// Unix timestamp
    pub time: i64,
    /// Currency
    pub currency: String,
    /// Change amount (positive for income, negative for expenditure)
    pub change: String,
    /// Balance after change
    pub balance: String,
    /// Entry type
    #[serde(rename = "type")]
    pub entry_type: String,
    /// Additional text
    pub text: Option<String>,
}

impl RestClient {
    /// Get account book
    ///
    /// This endpoint returns the account balance change history.
    /// You can filter by currency, time range, and record type.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-account-book>
    pub async fn get_account_book(
        &self,
        request: GetAccountBookRequest,
    ) -> crate::gateio::Result<Vec<AccountBookEntry>> {
        self.get_with_query("/spot/account_book", &request).await
    }
}
