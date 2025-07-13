use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_ACCOUNT_BOOK_ENDPOINT: &str = "/delivery/{}/account_book";

/// Request parameters for delivery account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryAccountBookRequest {
    /// Settlement currency
    pub settle: String,

    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Account book type filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Delivery account book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAccountBookEntry {
    /// Entry ID
    pub id: i64,

    /// Change time
    pub time: f64,

    /// Currency
    pub currency: String,

    /// Change amount
    pub change: String,

    /// Balance after change
    pub balance: String,

    /// Change type
    #[serde(rename = "type")]
    pub entry_type: String,

    /// Change text
    pub text: String,
}

impl RestClient {
    /// Query delivery account book
    ///
    /// Retrieves detailed account transaction history for delivery trading.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The account book request parameters
    ///
    /// # Returns
    /// List of account book entries
    pub async fn get_delivery_account_book(
        &self,
        params: DeliveryAccountBookRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryAccountBookEntry>> {
        let endpoint = DELIVERY_ACCOUNT_BOOK_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}
