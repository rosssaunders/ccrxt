use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{
        OrderSide, PriceMatch, RestResult,
        private::rest::{client::RestClient, modify_order::ModifyOrderResponse},
    },
    shared,
};

const BATCH_ORDERS_ENDPOINT: &str = "/dapi/v1/batchOrders";

/// Single order parameters for batch modify operation.
#[derive(Debug, Clone, Serialize)]
pub struct BatchModifyOrderItem {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Order side (BUY or SELL).
    pub side: OrderSide,

    /// Order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// New order quantity. Either quantity or price must be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// New order price. Either quantity or price must be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Price match mode. Only available for LIMIT/STOP/TAKE_PROFIT orders.
    /// Cannot be passed together with price.
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,
}

/// Request parameters for modifying multiple orders (PUT /dapi/v1/batchOrders).
#[derive(Debug, Clone, Serialize, Default)]
pub struct ModifyMultipleOrdersRequest {
    /// List of orders to modify. Maximum 5 orders.
    #[serde(rename = "batchOrders")]
    pub batch_orders: Vec<BatchModifyOrderItem>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Error response for a failed order modification in a batch.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchModifyOrderError {
    /// Error code.
    pub code: i32,

    /// Error message.
    pub msg: String,
}

/// Response item for batch modify orders - either success or error.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BatchModifyOrderResponseItem {
    /// Successful order modification.
    Success(ModifyOrderResponse),
    /// Failed order modification.
    Error(BatchModifyOrderError),
}

/// Response for modifying multiple orders (PUT /dapi/v1/batchOrders).
pub type ModifyMultipleOrdersResponse = Vec<BatchModifyOrderResponseItem>;

impl RestClient {
    /// Modifies multiple orders (TRADE) on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Multiple-Orders
    ///
    /// PUT /dapi/v1/batchOrders
    /// Weight: 5
    /// Requires API key and signature.
    ///
    /// Parameter rules are same as single order modification.
    /// Batch modify orders are processed concurrently, and the order of matching is not guaranteed.
    /// The order of returned contents for batch modify orders is the same as the order of the order list.
    /// One order can only be modified for less than 10000 times.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ModifyMultipleOrdersRequest`])
    ///
    /// # Returns
    /// A [`ModifyMultipleOrdersResponse`] - array of order results or errors.
    pub async fn modify_multiple_orders(
        &self,
        params: ModifyMultipleOrdersRequest,
    ) -> RestResult<ModifyMultipleOrdersResponse> {
        let weight = 5;
        shared::send_signed_request(
            self,
            BATCH_ORDERS_ENDPOINT,
            reqwest::Method::PUT,
            params,
            weight,
            true,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_modify_order_item_serialization_with_order_id() {
        let item = BatchModifyOrderItem {
            symbol: "BTCUSD_PERP".to_string(),
            side: OrderSide::Buy,
            order_id: Some(123456789),
            orig_client_order_id: None,
            quantity: Some("0.1".to_string()),
            price: Some("50000".to_string()),
            price_match: None,
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains(r#""symbol":"BTCUSD_PERP""#));
        assert!(json.contains(r#""side":"BUY""#));
        assert!(json.contains(r#""orderId":123456789"#));
        assert!(json.contains(r#""quantity":"0.1""#));
        assert!(json.contains(r#""price":"50000""#));
        assert!(!json.contains("origClientOrderId"));
        assert!(!json.contains("priceMatch"));
    }

    #[test]
    fn test_batch_modify_order_item_serialization_with_client_order_id() {
        let item = BatchModifyOrderItem {
            symbol: "ETHUSD_PERP".to_string(),
            side: OrderSide::Sell,
            order_id: None,
            orig_client_order_id: Some("my_order_123".to_string()),
            quantity: Some("1.5".to_string()),
            price: None,
            price_match: Some(PriceMatch::Opponent),
        };
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains(r#""symbol":"ETHUSD_PERP""#));
        assert!(json.contains(r#""side":"SELL""#));
        assert!(json.contains(r#""origClientOrderId":"my_order_123""#));
        assert!(json.contains(r#""quantity":"1.5""#));
        assert!(json.contains(r#""priceMatch":"OPPONENT""#));
        assert!(!json.contains("orderId"));
        assert!(!json.contains(r#""price""#));
    }

    #[test]
    fn test_modify_multiple_orders_request_serialization() {
        let request = ModifyMultipleOrdersRequest {
            batch_orders: vec![
                BatchModifyOrderItem {
                    symbol: "BTCUSD_PERP".to_string(),
                    side: OrderSide::Buy,
                    order_id: Some(123456789),
                    orig_client_order_id: None,
                    quantity: Some("0.1".to_string()),
                    price: Some("50000".to_string()),
                    price_match: None,
                },
                BatchModifyOrderItem {
                    symbol: "ETHUSD_PERP".to_string(),
                    side: OrderSide::Sell,
                    order_id: None,
                    orig_client_order_id: Some("my_order_456".to_string()),
                    quantity: None,
                    price: Some("3000".to_string()),
                    price_match: None,
                },
            ],
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""batchOrders""#));
        assert!(json.contains(r#""recvWindow":5000"#));
        assert!(json.contains(r#""timestamp":1625097600000"#));
        assert!(json.contains(r#""BTCUSD_PERP""#));
        assert!(json.contains(r#""ETHUSD_PERP""#));
    }

    #[test]
    fn test_modify_multiple_orders_response_deserialization_success() {
        let json = r#"[
            {
                "orderId": 123456789,
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "status": "PARTIALLY_FILLED",
                "clientOrderId": "testOrder",
                "price": "50000",
                "avgPrice": "0",
                "origQty": "0.1",
                "executedQty": "0.05",
                "cumBase": "0.00001",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "reduceOnly": false,
                "closePosition": false,
                "side": "BUY",
                "positionSide": "BOTH",
                "stopPrice": "0",
                "workingType": "CONTRACT_PRICE",
                "priceProtect": false,
                "origType": "LIMIT",
                "priceMatch": "NONE",
                "selfTradePreventionMode": "NONE",
                "goodTillDate": 0,
                "updateTime": 1625097600001
            }
        ]"#;
        let response: ModifyMultipleOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        match &response[0] {
            BatchModifyOrderResponseItem::Success(order) => {
                assert_eq!(order.order_id, 123456789);
                assert_eq!(order.symbol, "BTCUSD_PERP");
                assert_eq!(order.price, "50000");
            }
            BatchModifyOrderResponseItem::Error(_) => panic!("Expected success, got error"),
        }
    }

    #[test]
    fn test_modify_multiple_orders_response_deserialization_mixed() {
        let json = r#"[
            {
                "orderId": 123456789,
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "status": "NEW",
                "clientOrderId": "testOrder1",
                "price": "50000",
                "avgPrice": "0",
                "origQty": "0.1",
                "executedQty": "0",
                "cumBase": "0",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "reduceOnly": false,
                "closePosition": false,
                "side": "BUY",
                "positionSide": "BOTH",
                "stopPrice": "0",
                "workingType": "CONTRACT_PRICE",
                "priceProtect": false,
                "origType": "LIMIT",
                "priceMatch": "NONE",
                "selfTradePreventionMode": "NONE",
                "goodTillDate": 0,
                "updateTime": 1625097600001
            },
            {
                "code": -2011,
                "msg": "Unknown order sent."
            }
        ]"#;
        let response: ModifyMultipleOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        match &response[0] {
            BatchModifyOrderResponseItem::Success(order) => {
                assert_eq!(order.order_id, 123456789);
            }
            BatchModifyOrderResponseItem::Error(_) => panic!("Expected success for first item"),
        }

        match &response[1] {
            BatchModifyOrderResponseItem::Error(error) => {
                assert_eq!(error.code, -2011);
                assert_eq!(error.msg, "Unknown order sent.");
            }
            BatchModifyOrderResponseItem::Success(_) => panic!("Expected error for second item"),
        }
    }
}
