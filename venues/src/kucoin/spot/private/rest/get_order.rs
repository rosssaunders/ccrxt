use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{OrderSide, ResponseHeaders, RestResponse, Result};

const GET_ORDER_ENDPOINT: &str = "/api/v1/orders/{order_id}";

/// Request for getting order details
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    /// Order ID
    pub order_id: String,
}

/// Order information
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,

    /// Trading symbol
    pub symbol: String,

    /// Operation type (DEAL)
    #[serde(rename = "opType")]
    pub operation_type: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,

    /// Order side
    pub side: OrderSide,

    /// Order amount
    pub amount: String,

    /// Order funds
    pub funds: String,

    /// Dealt amount
    #[serde(rename = "dealAmount")]
    pub deal_amount: String,

    /// Dealt funds
    #[serde(rename = "dealFunds")]
    pub deal_funds: String,

    /// Fee
    pub fee: String,

    /// Fee currency
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,

    /// Self-trade prevention
    pub stp: String,

    /// Stop type
    pub stop: String,

    /// Stop triggered flag
    #[serde(rename = "stopTriggered")]
    pub stop_triggered: bool,

    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,

    /// Post only flag
    #[serde(rename = "postOnly")]
    pub post_only: bool,

    /// Hidden order flag
    pub hidden: bool,

    /// Iceberg order flag
    pub iceberg: bool,

    /// Visible size for iceberg orders
    #[serde(rename = "visibleSize")]
    pub visible_size: String,

    /// Cancel after time
    #[serde(rename = "cancelAfter")]
    pub cancel_after: i64,

    /// Channel
    pub channel: String,

    /// Client order ID
    #[serde(rename = "clientOid")]
    pub client_order_id: String,

    /// Remark
    pub remark: String,

    /// Tags
    pub tags: String,

    /// Is active flag
    #[serde(rename = "isActive")]
    pub is_active: bool,

    /// Cancel exist flag
    #[serde(rename = "cancelExist")]
    pub cancel_exist: bool,

    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,

    /// Trade type
    #[serde(rename = "tradeType")]
    pub trade_type: String,

    /// Price
    pub price: String,

    /// Size
    pub size: String,
}

impl RestClient {
    /// Get order details by order ID
    ///
    /// [docs](https://docs.kucoin.com/#get-single-order-info)
    pub async fn get_order(&self, request: GetOrderRequest) -> Result<(Order, ResponseHeaders)> {
        let endpoint = GET_ORDER_ENDPOINT.replace("{order_id}", &request.order_id);
        let (response, headers): (RestResponse<Order>, ResponseHeaders) =
            self.get_with_request(&endpoint, &()).await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_request_creation() {
        let request = GetOrderRequest {
            order_id: "test_order_id".to_string(),
        };
        assert_eq!(request.order_id, "test_order_id");
    }
}
