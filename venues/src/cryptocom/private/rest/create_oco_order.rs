use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Request parameters for creating an OCO order list (One-Cancels-the-Other).
///
/// This endpoint allows users to place two orders at the same time (a limit order and a stop order).
/// When either order is executed, the other is automatically canceled. Only available for Spot and Perpetual/Futures pairs.
///
/// See: https://exchange-docs.crypto.com/derivatives/index.html#private-create-order-list
#[derive(Debug, Clone, Serialize)]
pub struct CreateOcoOrderRequest {
    /// Contingency type. Must be "OCO" for this endpoint.
    pub contingency_type: String,

    /// List of exactly two orders (limit and stop order).
    pub order_list: Vec<OcoOrderItem>,
}

/// Individual order in an OCO order list.
#[derive(Debug, Clone, Serialize)]
pub struct OcoOrderItem {
    /// Instrument name (e.g., BTCUSD-PERP)
    pub instrument_name: String,

    /// Order side (BUY or SELL)
    pub side: crate::cryptocom::enums::OrderSide,

    /// Order type (LIMIT or STOP_LOSS)
    #[serde(rename = "type")]
    pub order_type: crate::cryptocom::enums::OrderType,

    /// Order price (required for LIMIT orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Order quantity
    pub quantity: String,

    /// Reference price for STOP_LOSS orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_price: Option<String>,
}

/// Response for creating OCO orders
#[derive(Debug, Clone, Deserialize)]
pub struct CreateOcoOrderResponse {
    /// List ID for the OCO order
    pub list_id: String,
}

impl RestClient {
    /// Create an OCO order list (One-Cancels-the-Other)
    ///
    /// Creates a pair of orders (limit and stop) that are linked so that the execution of one cancels the other.
    ///
    /// See: https://exchange-docs.crypto.com/derivatives/index.html#private-create-order-list
    ///
    /// Rate limit: 10 requests per second per user
    ///
    /// # Arguments
    /// * `request` - The OCO order list creation request
    ///
    /// # Returns
    /// The response contains the OCO list ID.
    pub async fn create_oco_order_list(&self, request: CreateOcoOrderRequest) -> RestResult<CreateOcoOrderResponse> {
        let params = serde_json::to_value(&request)?;
        // ...existing code for sending the request and handling the response...
        unimplemented!("Implement the HTTP request/response logic here");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptocom::enums::{OrderSide, OrderType};

    #[test]
    fn test_oco_order_request_structure() {
        let req = CreateOcoOrderRequest {
            contingency_type: "OCO".to_string(),
            order_list: vec![
                OcoOrderItem {
                    instrument_name: "BTCUSD-PERP".to_string(),
                    side: OrderSide::SELL,
                    order_type: OrderType::LIMIT,
                    price: Some("23000".to_string()),
                    quantity: "0.1".to_string(),
                    ref_price: None,
                },
                OcoOrderItem {
                    instrument_name: "BTCUSD-PERP".to_string(),
                    side: OrderSide::SELL,
                    order_type: OrderType::STOP_LOSS,
                    price: None,
                    quantity: "0.1".to_string(),
                    ref_price: Some("19000".to_string()),
                },
            ],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"contingency_type\":\"OCO\""));
        assert!(json.contains("\"order_list\""));
    }

    #[test]
    fn test_oco_order_response_structure() {
        let json = r#"{ "list_id": "6498090546073120100" }"#;
        let resp: CreateOcoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.list_id, "6498090546073120100");
    }
}
