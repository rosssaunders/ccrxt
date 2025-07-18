use serde::{Deserialize, Serialize};

use super::RestClient;

/// Options trade record
#[derive(Debug, Clone, Deserialize)]
pub struct OptionsTrade {
    /// Trade ID
    pub id: String,

    /// Trade creation time
    pub create_time: f64,

    /// Order ID
    pub order_id: String,

    /// Contract name
    pub contract: String,

    /// Trade size
    pub size: i64,

    /// Trade price
    pub price: String,

    /// Underlying asset
    pub underlying: String,

    /// Trade role
    pub role: String,
}

/// Request to retrieve options trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsTradesRequest {
    /// Underlying asset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Maximum number of record items to be returned (1-1000)
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
}

impl RestClient {
    /// Get options trades
    ///
    /// This endpoint returns a list of options trades.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The trades request parameters
    ///
    /// # Returns
    /// List of trade records
    pub async fn get_options_trades(
        &self,
        request: OptionsTradesRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsTrade>> {
        self.get_with_query("/options/my_trades", &request).await
    }
}
