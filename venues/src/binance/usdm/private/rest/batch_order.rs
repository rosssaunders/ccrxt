use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{
    RestResult,
    enums::{OrderSide, OrderType, PositionSide, TimeInForce, WorkingType},
};

const BATCH_ORDERS_ENDPOINT: &str = "/fapi/v1/batchOrders";

/// Individual order in a batch request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderItem {
    /// Trading symbol
    pub symbol: String,

    /// Order side
    pub side: OrderSide,

    /// Position side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Stop price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Close position flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,

    /// Reduce only flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Working type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,

    /// Price protection flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<bool>,
}

/// Request parameters for placing batch orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderRequest {
    /// List of orders to place (max 5)
    pub batch_orders: Vec<BatchOrderItem>,
}

/// Response for a single order in the batch.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: String,

    /// Transaction time (timestamp in milliseconds).
    pub transact_time: u64,

    /// Order price.
    pub price: String,

    /// Original order quantity.
    pub orig_qty: String,

    /// Executed quantity.
    pub executed_qty: String,

    /// Cumulative quote quantity.
    pub cum_quote: String,

    /// Order status.
    pub status: String,

    /// Time in force.
    pub time_in_force: String,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: String,

    /// Order side.
    pub side: String,

    /// Position side.
    pub position_side: String,

    /// Working type.
    pub working_type: String,
}

impl UsdmClient {
    /// Place multiple orders (POST /fapi/v1/batchOrders)
    ///
    /// Places multiple orders in a single batch for USDM futures.
    ///
    /// [docs]: https://binance-docs.github.io/apidocs/futures/en/#place-multiple-orders-trade
    ///
    /// Rate limit: 5 weight
    ///
    /// # Arguments
    /// * `request` - The batch order request parameters
    ///
    /// # Returns
    /// Vector of order responses, one for each order in the batch
    pub async fn place_batch_orders(
        &self,
        request: BatchOrderRequest,
    ) -> RestResult<Vec<BatchOrderResponse>> {
        self.send_signed_request(BATCH_ORDERS_ENDPOINT, Method::POST, request, 5, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_order_item_serialization() {
        let item = BatchOrderItem {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            position_side: Some(PositionSide::Long),
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some("0.1".to_string()),
            price: Some("50000".to_string()),
            new_client_order_id: Some("test123".to_string()),
            stop_price: None,
            close_position: None,
            reduce_only: Some(false),
            working_type: Some(WorkingType::ContractPrice),
            price_protect: None,
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains(r#""symbol":"BTCUSDT""#));
        assert!(json.contains(r#""side":"BUY""#));
        assert!(json.contains(r#""type":"LIMIT""#));
        assert!(json.contains(r#""quantity":"0.1""#));
        assert!(json.contains(r#""price":"50000""#));
    }

    #[test]
    fn test_batch_order_request_serialization() {
        let request = BatchOrderRequest {
            batch_orders: vec![BatchOrderItem {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                position_side: Some(PositionSide::Long),
                order_type: OrderType::Limit,
                time_in_force: Some(TimeInForce::GTC),
                quantity: Some("0.1".to_string()),
                price: Some("50000".to_string()),
                new_client_order_id: Some("order1".to_string()),
                stop_price: None,
                close_position: None,
                reduce_only: Some(false),
                working_type: Some(WorkingType::ContractPrice),
                price_protect: None,
            }],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""batchOrders":[{"#));
        assert!(json.contains(r#""symbol":"BTCUSDT""#));
    }

    #[test]
    fn test_batch_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 123456789,
            "clientOrderId": "test123",
            "transactTime": 1625184000000,
            "price": "50000.00",
            "origQty": "0.100",
            "executedQty": "0.000",
            "cumQuote": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "positionSide": "LONG",
            "workingType": "CONTRACT_PRICE"
        }"#;

        let response: BatchOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.client_order_id, "test123");
        assert_eq!(response.price, "50000.00");
        assert_eq!(response.status, "NEW");
    }
}
