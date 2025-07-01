use serde::Serialize;

use super::RestClient;
use crate::gateio::private::rest::create_order::Order;

/// Order amendment request
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderRequest {
    /// New order amount (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// New order price (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Amendment text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

impl RestClient {
    /// Amend an order
    /// 
    /// This endpoint modifies the price and/or amount of an existing order.
    pub async fn amend_order(
        &self, 
        order_id: &str, 
        currency_pair: &str,
        amendment: AmendOrderRequest
    ) -> crate::gateio::Result<Order> {
        let endpoint = format!("/spot/orders/{}", order_id);
        #[allow(clippy::unwrap_used)]
        let mut body = serde_json::to_value(&amendment).unwrap();
        #[allow(clippy::indexing_slicing)]
        {
            body["currency_pair"] = serde_json::Value::String(currency_pair.to_string());
        }
        
        self.patch(&endpoint, &body).await
    }
}
