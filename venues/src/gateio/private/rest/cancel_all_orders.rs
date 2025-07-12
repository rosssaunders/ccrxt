use serde::Serialize;

use super::RestClient;
use crate::gateio::{OrderSide, private::rest::create_order::Order};

/// Cancel all orders request
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrdersRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Order side (optional, cancel all sides if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,

    /// Account type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

impl RestClient {
    /// Cancel all orders
    ///
    /// This endpoint cancels all open orders for a currency pair.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#cancel-all-open-orders-in-specified-currency-pair>
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> crate::gateio::Result<Vec<Order>> {
        self.delete_with_query("/spot/orders", &request).await
    }

    /// Cancel all orders for a currency pair
    pub async fn cancel_all_orders_for_pair(
        &self,
        currency_pair: &str,
    ) -> crate::gateio::Result<Vec<Order>> {
        let request = CancelAllOrdersRequest {
            currency_pair: currency_pair.to_string(),
            side: None,
            account: None,
        };
        self.cancel_all_orders(request).await
    }
}
