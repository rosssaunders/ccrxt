use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const DUAL_ORDERS_ENDPOINT: &str = "/earn/dual/orders";

/// Request parameters for Dual Investment order list.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DualOrdersRequest {
    /// Start settlement time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End settlement time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Page number. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records returned in a single list. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Represents a single Dual Investment order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DualOrder {
    /// Order ID
    pub id: i32,

    /// Product ID
    pub plan_id: i32,

    /// Units
    pub copies: String,

    /// Investment Quantity
    pub invest_amount: String,

    /// Settlement Quantity
    pub settlement_amount: String,

    /// Creation time
    pub create_time: i32,

    /// Completed Time
    pub complete_time: i32,

    /// Status: INIT, SETTLEMENT_SUCCESS, SETTLEMENT_PROCESSING, CANCELED, FAILED
    pub status: String,

    /// Investment Token
    pub invest_currency: String,

    /// Strike Token
    pub exercise_currency: String,

    /// Settlement currency
    pub settlement_currency: String,

    /// Strike price
    pub exercise_price: String,

    /// Settlement price
    pub settlement_price: String,

    /// Settlement time
    pub delivery_time: i32,

    /// Annual Yield
    pub apy_display: String,

    /// Settlement Annual Yield
    pub apy_settlement: String,

    /// Custom order information
    pub text: String,
}

impl RestClient {
    /// Dual Investment order list endpoint
    ///
    /// Returns a list of dual investment orders.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#dual-investment-order-list)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The order list request parameters
    ///
    /// # Returns
    /// List of dual investment orders
    pub async fn dual_orders(&self, request: DualOrdersRequest) -> RestResult<Vec<DualOrder>> {
        self.send_get_request::<Vec<DualOrder>, _>(DUAL_ORDERS_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_orders_request_serialization() {
        let req = DualOrdersRequest {
            from: Some(1),
            to: Some(2),
            page: Some(1),
            limit: Some(10),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("from"));
        assert!(json.contains("to"));
        assert!(json.contains("page"));
        assert!(json.contains("limit"));
    }

    #[test]
    fn test_dual_order_deserialization() {
        let json = r#"{
            "id": 373,
            "plan_id": 176,
            "copies": "1.0000000000",
            "invest_amount": "0.0000000000",
            "settlement_amount": "0.0000000000",
            "create_time": 1697685172,
            "complete_time": 1697685172,
            "status": "CANCELED",
            "invest_currency": "USDT",
            "exercise_currency": "BTC",
            "settlement_currency": "",
            "exercise_price": "24500.0000000000",
            "settlement_price": "0.0000000000",
            "delivery_time": 1697685172,
            "apy_display": "0.6800000000",
            "apy_settlement": "0.0000000000",
            "text": "t-custom-text"
        }"#;
        let order: DualOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, 373);
        assert_eq!(order.status, "CANCELED");
        assert_eq!(order.text, "t-custom-text");
    }
}
