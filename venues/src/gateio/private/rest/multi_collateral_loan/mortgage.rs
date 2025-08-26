use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const MORTGAGE_ENDPOINT: &str = "/loan/multi_collateral/mortgage";

/// Collateral management request (add or withdraw)
#[derive(Debug, Clone, Serialize)]
pub struct MortgageRequest {
    /// Loan order ID
    pub order_id: String,

    /// Operation type: "append" to add collateral, "redeem" to withdraw
    #[serde(rename = "type")]
    pub operation_type: String,

    /// List of collateral currencies and amounts
    pub collaterals: Vec<CollateralItem>,
}

/// Individual collateral item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralItem {
    /// Currency code
    pub currency: String,

    /// Amount to add or withdraw
    pub amount: String,
}

/// Mortgage operation response
#[derive(Debug, Clone, Deserialize)]
pub struct MortgageResponse {
    /// Operation success status
    pub success: bool,

    /// Updated collateral information (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaterals: Option<Vec<CollateralItem>>,
}

impl RestClient {
    /// Manage Multi-Currency Collateral
    ///
    /// Add or withdraw collateral from an existing multi-collateral loan order.
    /// Supports both "append" operations to add more collateral and "redeem" operations
    /// to withdraw excess collateral while maintaining required loan-to-value ratios.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#manage-multi-currency-collateral)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Mortgage management request with order ID, operation type, and collateral details
    ///
    /// # Returns
    /// Management response with operation success status and updated collateral information
    pub async fn manage_mortgage(&self, req: MortgageRequest) -> RestResult<MortgageResponse> {
        self.send_post_request(MORTGAGE_ENDPOINT, Some(&req)).await
    }
}
