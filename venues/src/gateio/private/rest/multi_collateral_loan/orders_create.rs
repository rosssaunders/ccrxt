use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/loan/multi_collateral/orders";

/// Create a multi-collateral loan order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Order type: "current" or "fixed"
    pub order_type: String,

    /// Borrow currency
    pub borrow_currency: String,

    /// Borrow amount
    pub borrow_amount: String,

    /// Collateral currencies and amounts
    pub collateral_currencies: Vec<CollateralPart>,

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
    /// Place Multi-Currency Collateral Order
    ///
    /// Creates a new multi-currency collateral loan order with flexible collateral options.
    /// Supports both current rate (variable) and fixed rate loan types with multiple collateral currencies.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#place-multi-currency-collateral-order)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `req` - Order creation request with order type, borrow currency/amount, and collateral details
    ///
    /// # Returns
    /// Order response containing the created order ID, borrow details, and initial status
    pub async fn create_multi_collateral_order(
        &self,
        req: CreateOrderRequest,
    ) -> RestResult<OrderResponse> {
        self.send_post_request(ORDERS_ENDPOINT, Some(&req)).await
    }
}
