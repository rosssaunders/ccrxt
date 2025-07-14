use serde::{Deserialize, Serialize};

use super::RestClient;

/// Options settlement record
#[derive(Debug, Clone, Deserialize)]
pub struct OptionsSettlement {
    /// Settlement time
    pub time: i64,

    /// Underlying asset
    pub underlying: String,

    /// Contract
    pub contract: String,

    /// Settlement profit
    pub profit: String,

    /// Fee
    pub fee: String,

    /// Settlement price
    pub settle_price: String,

    /// Strike price
    pub strike_price: String,

    /// Size
    pub size: i64,
}

/// Request to retrieve options settlements
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsSettlementsRequest {
    /// Underlying asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Number of results per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Start time (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

impl RestClient {
    /// Get options settlements
    ///
    /// This endpoint returns a list of options settlement records.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The settlement request parameters
    ///
    /// # Returns
    /// List of settlement records
    pub async fn get_options_settlements(
        &self,
        request: OptionsSettlementsRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsSettlement>> {
        self.get_with_query("/options/settlements", &request).await
    }
}
