use serde::{Deserialize, Serialize};

use super::RestClient;

/// Options position close history entry
#[derive(Debug, Clone, Deserialize)]
pub struct OptionsPositionCloseHistory {
    /// Entry time
    pub time: f64,

    /// Profit and loss
    pub pnl: String,

    /// Position side
    pub side: String,

    /// Contract name
    pub contract: String,

    /// Text description
    pub text: String,
}

/// Request to retrieve options position close history
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsPositionCloseHistoryRequest {
    /// Underlying asset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Position side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,

    /// Maximum number of record items to be returned (1-1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// You can set this to the last result time to retrieve records after that time
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
    /// Get options position close history
    ///
    /// This endpoint returns the position close history for options trading.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The position close history request parameters
    ///
    /// # Returns
    /// List of position close history entries
    pub async fn get_options_position_close_history(
        &self,
        request: OptionsPositionCloseHistoryRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsPositionCloseHistory>> {
        self.get_with_query("/options/position_close", &request)
            .await
    }
}
