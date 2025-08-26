use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/flash_swap/orders";

/// Create flash swap order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Preview result ID
    pub preview_id: String,

    /// Currency to sell
    pub sell_currency: String,

    /// Amount to sell
    pub sell_amount: String,

    /// Currency to buy
    pub buy_currency: String,

    /// Amount to buy
    pub buy_amount: String,
}

/// Query flash swap orders
#[derive(Debug, Clone, Serialize)]
pub struct ListOrdersRequest {
    /// Order status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Filter by sell currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_currency: Option<String>,

    /// Filter by buy currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_currency: Option<String>,

    /// Sort order (true for desc, false for asc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

/// Flash swap order response
#[derive(Debug, Clone, Deserialize)]
pub struct FlashSwapOrder {
    /// Order ID
    pub id: String,

    /// Order status
    pub status: String,

    /// Currency to sell
    pub sell_currency: String,

    /// Amount to sell
    pub sell_amount: String,

    /// Currency to buy
    pub buy_currency: String,

    /// Amount to buy
    pub buy_amount: String,

    /// Exchange rate
    pub price: String,

    /// Fee amount
    pub fee: String,

    /// Creation time
    pub create_time: i64,

    /// Update time
    pub update_time: i64,
}

impl RestClient {
    /// Create a flash swap order
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#create-flash-swap-order)
    pub async fn create_flash_swap_order(
        &self,
        req: CreateOrderRequest,
    ) -> RestResult<FlashSwapOrder> {
        self.send_post_request(ORDERS_ENDPOINT, Some(&req)).await
    }

    /// Query flash swap orders
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#list-flash-swap-orders)
    pub async fn get_flash_swap_orders(
        &self,
        req: Option<ListOrdersRequest>,
    ) -> RestResult<Vec<FlashSwapOrder>> {
        self.send_get_request(ORDERS_ENDPOINT, req.as_ref()).await
    }

    /// Get details of a specific flash swap order
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#get-flash-swap-order)
    pub async fn get_flash_swap_order(&self, order_id: &str) -> RestResult<FlashSwapOrder> {
        let endpoint = format!("{}/{}", ORDERS_ENDPOINT, order_id);
        self.send_get_request(&endpoint, None::<&()>).await
    }
}
