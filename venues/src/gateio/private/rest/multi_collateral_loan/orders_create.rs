use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/loan/multi_collateral/orders";

/// Create a multi-collateral loan order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Borrow currency
    pub borrow_currency: String,

    /// Borrow amount
    pub borrow_amount: String,

    /// Collateral currencies and amounts
    pub collaterals: Vec<CollateralPart>,

    /// Optional client order id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// One collateral component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralPart {
    pub currency: String,

    pub amount: String,
}

/// Multi-collateral order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: String,

    pub borrow_currency: String,

    pub borrow_amount: String,

    pub status: String,
}

impl RestClient {
    /// Create a new multi-collateral loan order
    pub async fn create_multi_collateral_order(
        &self,
        req: CreateOrderRequest,
    ) -> RestResult<OrderResponse> {
        self.send_post_request(ORDERS_ENDPOINT, Some(&req)).await
    }
}
