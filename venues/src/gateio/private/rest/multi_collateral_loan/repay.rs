use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const REPAY_ENDPOINT: &str = "/loan/multi_collateral/repay";

/// Request to repay multi-collateral order
#[derive(Debug, Clone, Serialize)]
pub struct RepayRequest {
    /// Specific loan order ID
    pub order_id: String,

    /// List of repayment details
    pub repay_items: Vec<RepayItem>,
}

/// Individual repayment item
#[derive(Debug, Clone, Serialize)]
pub struct RepayItem {
    /// Currency to repay
    pub currency: String,

    /// Amount to repay
    pub amount: String,
}

/// Repay response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepayResponse {
    pub success: bool,

    pub remaining: Option<String>,
}

impl RestClient {
    /// Repay Multi-Currency Collateral Loan
    ///
    /// Allows partial or full repayment of borrowed amounts in a multi-collateral loan order.
    /// Supports repaying multiple currencies in a single request with flexible repayment amounts.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#repay-multi-currency-collateral-loan)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Repayment request containing order ID and list of currencies/amounts to repay
    ///
    /// # Returns
    /// Repayment response with success status and optional remaining balance information
    pub async fn repay_multi_collateral(&self, req: RepayRequest) -> RestResult<RepayResponse> {
        self.send_post_request(REPAY_ENDPOINT, Some(&req)).await
    }
}
