use super::{RestClient, RestResult};
use serde::{Deserialize, Serialize};

const STRUCTURED_ORDERS_ENDPOINT: &str = "/earn/structured/orders";

/// Request parameters for Structured Product Order List.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructuredOrdersRequest {
    /// Start timestamp. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// Termination Timestamp. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Page number. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records returned in a single list. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Represents a single Structured Product order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StructuredOrder {
    /// Order ID
    pub id: i32,

    /// Product ID
    pub pid: String,

    /// Locked coin
    pub lock_coin: String,

    /// Locked amount
    pub amount: String,

    /// Status: SUCCESS, FAILED, DONE
    pub status: String,

    /// Income
    pub income: String,

    /// Created time
    pub create_time: i32,
}

impl RestClient {
    /// Structured Product Order List endpoint
    ///
    /// Returns a list of structured product orders.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#structured-product-order-list)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The order list request parameters
    ///
    /// # Returns
    /// List of structured product orders
    pub async fn structured_orders(
        &self,
        request: StructuredOrdersRequest,
    ) -> RestResult<Vec<StructuredOrder>> {
        self.send_get_request::<Vec<StructuredOrder>, _>(STRUCTURED_ORDERS_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_orders_request_serialization() {
        let req = StructuredOrdersRequest {
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
    fn test_structured_order_deserialization() {
        let json = r#"{
            "id": 35,
            "pid": "3691",
            "lock_coin": "ETH",
            "amount": "20",
            "status": "SUCCESS",
            "income": "0.000000",
            "create_time": 1697685172
        }"#;
        let order: StructuredOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, 35);
        assert_eq!(order.status, "SUCCESS");
        assert_eq!(order.lock_coin, "ETH");
    }
}
