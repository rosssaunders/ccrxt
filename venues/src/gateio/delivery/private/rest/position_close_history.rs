use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_POSITION_CLOSE_ENDPOINT: &str = "/delivery/{}/position_close";

/// Request parameters for delivery position close history
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryPositionCloseHistoryRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

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

    /// Order side filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
}

/// Delivery position close history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPositionClose {
    /// Position close time
    pub time: f64,

    /// PnL
    pub pnl: String,

    /// Position side
    pub side: String,

    /// Contract name
    pub contract: String,

    /// Text
    pub text: String,

    /// Maximum position size during the period
    pub max_size: i64,
}

impl RestClient {
    /// List delivery position close history
    ///
    /// Retrieves history of closed delivery positions.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The position close history request parameters
    ///
    /// # Returns
    /// List of position close history entries
    pub async fn get_delivery_position_close_history(
        &self,
        params: DeliveryPositionCloseHistoryRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryPositionClose>> {
        let endpoint = DELIVERY_POSITION_CLOSE_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}
