use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MARGIN_ACCOUNT_BOOK_ENDPOINT: &str = "/margin/account_book";

/// Request parameters for querying margin account balance change history
#[derive(Debug, Clone, Serialize)]
pub struct MarginAccountBookRequest {
    /// Specific currency
    pub currency: String,

    /// Trading pair
    pub currency_pair: String,

    /// Account change type
    #[serde(rename = "type")]
    pub change_type: String,

    /// Start timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Records per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Margin account balance change record
#[derive(Debug, Clone, Deserialize)]
pub struct MarginAccountBookEntry {
    /// Record ID
    pub id: String,

    /// Transaction timestamp
    pub time: i64,

    /// Currency code
    pub currency: String,

    /// Balance change amount
    pub change: String,

    /// Balance after change
    pub balance: String,

    /// Transaction type
    #[serde(rename = "type")]
    pub change_type: String,

    /// Account type
    pub account: String,

    /// Additional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
}

impl RestClient {
    /// Margin Account Balance Change History
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#margin-account-book)
    pub async fn get_margin_account_book(
        &self,
        req: MarginAccountBookRequest,
    ) -> RestResult<Vec<MarginAccountBookEntry>> {
        self.send_get_request(MARGIN_ACCOUNT_BOOK_ENDPOINT, Some(&req))
            .await
    }
}
