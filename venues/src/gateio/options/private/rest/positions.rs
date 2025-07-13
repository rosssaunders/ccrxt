use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options positions
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsPositionsRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Options position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsPosition {
    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Position size
    pub size: String,

    /// Average entry price
    pub entry_price: String,

    /// Mark price
    pub mark_price: String,

    /// Mark IV (implied volatility)
    pub mark_iv: String,

    /// Realized PnL
    pub realised_pnl: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Pending orders
    pub pending_orders: i32,

    /// Close order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_order: Option<serde_json::Value>,

    /// Delta
    pub delta: String,

    /// Gamma
    pub gamma: String,

    /// Vega
    pub vega: String,

    /// Theta
    pub theta: String,
}

impl RestClient {
    /// Get options positions
    ///
    /// This endpoint returns all options positions for the authenticated user.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The options positions request parameters
    ///
    /// # Returns
    /// List of options positions
    pub async fn get_options_positions(
        &self,
        params: OptionsPositionsRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsPosition>> {
        self.get_with_query("/options/positions", &params).await
    }

    /// Get a specific options position
    ///
    /// This endpoint returns details for a specific options position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Specific options position details
    pub async fn get_options_position(
        &self,
        contract: &str,
    ) -> crate::gateio::options::Result<OptionsPosition> {
        let endpoint = format!("/options/positions/{}", contract);
        self.get(&endpoint).await
    }
}
