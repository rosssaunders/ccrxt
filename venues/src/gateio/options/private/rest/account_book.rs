use serde::{Deserialize, Serialize};

use super::RestClient;

/// Options account book entry
#[derive(Debug, Clone, Deserialize)]
pub struct OptionsAccountBookEntry {
    /// Entry ID
    pub id: String,

    /// Time of the entry
    pub time: f64,

    /// Change amount
    pub change: String,

    /// Balance after the change
    pub balance: String,

    /// Entry type
    #[serde(rename = "type")]
    pub entry_type: String,

    /// Text description
    pub text: String,
}

/// Request to retrieve options account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsAccountBookRequest {
    /// Maximum number of record items to be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// You can set this to the last result ID to retrieve the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Start timestamp (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End timestamp (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Filter by entry type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
}

impl RestClient {
    /// Get options account book
    ///
    /// This endpoint returns the options account book with balance change records.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The account book request parameters
    ///
    /// # Returns
    /// List of account book entries
    pub async fn get_options_account_book(
        &self,
        request: OptionsAccountBookRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsAccountBookEntry>> {
        self.get_with_query("/options/account_book", &request)
            .await
    }
}
