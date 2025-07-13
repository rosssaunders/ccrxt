use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_LIQUIDATES_ENDPOINT: &str = "/delivery/{}/liquidates";

/// Request parameters for delivery liquidation history
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryLiquidationHistoryRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Delivery liquidation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLiquidation {
    /// Liquidation time
    pub time: f64,

    /// Contract name
    pub contract: String,

    /// Liquidation size
    pub size: i64,

    /// Liquidation price
    pub price: String,

    /// Left position size after liquidation
    pub left: i64,

    /// Leverage
    pub leverage: String,

    /// Margin
    pub margin: String,

    /// Entry price
    pub entry_price: String,

    /// Liquidation fee
    pub liq_price: String,

    /// Mark price
    pub mark_price: String,
}

impl RestClient {
    /// List delivery liquidation history
    ///
    /// Retrieves the user's liquidation history for delivery contracts.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The liquidation history request parameters
    ///
    /// # Returns
    /// List of liquidation history entries
    pub async fn get_delivery_liquidation_history(
        &self,
        params: DeliveryLiquidationHistoryRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryLiquidation>> {
        let endpoint = DELIVERY_LIQUIDATES_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}
