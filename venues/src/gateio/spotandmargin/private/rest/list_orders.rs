use serde::Serialize;

use super::RestClient;
use crate::gateio::spotandmargin::{OrderSide, OrderStatus, private::rest::create_order::Order};

/// List orders request
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListOrdersRequest {
    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Order status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Number of records per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Start timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Order side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
}

impl RestClient {
    /// List orders
    ///
    /// This endpoint returns a list of orders based on the provided filters.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-orders>
    pub async fn list_orders(
        &self,
        request: ListOrdersRequest,
    ) -> crate::gateio::spotandmargin::Result<Vec<Order>> {
        self.get_with_query("/spot/orders", &request).await
    }
}
