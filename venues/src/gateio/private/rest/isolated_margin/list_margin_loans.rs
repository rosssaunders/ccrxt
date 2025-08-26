use serde::Serialize;

use super::{RestClient, create_loan::Loan};

const MARGIN_LOANS_ENDPOINT: &str = "/margin/uni/loans";

/// Request parameters for listing margin loans with comprehensive filtering options.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListLoansRequest {
    /// Loan status filter (e.g., "open", "finished", "cancelled").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Side filter for loan operation type ("lend" for lending, "borrow" for borrowing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,

    /// Currency filter for specific asset loans (e.g., "BTC", "ETH", "USDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Trading pair filter for loans tied to specific markets (e.g., "BTC_USDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Sort field for result ordering (e.g., "create_time", "amount", "rate").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Reverse sort order flag for descending order when true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_sort: Option<bool>,

    /// Page number for pagination (1-based indexing, default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return per page (1-100, default: 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl RestClient {
    /// Margin Loans
    ///
    /// Retrieve a list of margin loans for lending or borrowing.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#list-margin-loans)
    pub async fn spot_list_margin_loans(
        &self,
        params: ListLoansRequest,
    ) -> RestResult<Vec<Loan>> {
        self.get_with_query(MARGIN_LOANS_ENDPOINT, &params).await
    }
}
