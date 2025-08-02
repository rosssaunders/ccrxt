use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    CancelRestrictions, OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode,
    TimeInForce,
};

const CANCEL_ORDER_ENDPOINT: &str = "/api/v3/order";

/// Request parameters for cancelling an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// New client order ID for the cancel
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Cancel restrictions
    #[serde(rename = "cancelRestrictions", skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestrictions>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Cancel order response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Original client order ID
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Transaction time
    #[serde(rename = "transactTime")]
    pub transact_time: u64,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Original quantity
    #[serde(rename = "origQty")]
    pub orig_qty: Decimal,

    /// Executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: Decimal,

    /// Cumulative quote quantity
    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: Decimal,

    /// Order status
    #[serde(rename = "status")]
    pub status: OrderStatus,

    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Stop price
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,

    /// Iceberg quantity
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<Decimal>,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Cancel an active order
    ///
    /// Cancel an active order.
    /// Either orderId or origClientOrderId must be sent.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#cancel-order--trade)
    /// Method: DELETE /api/v3/order
    /// Weight: 1
    /// Security: TRADE
    pub async fn cancel_order(
        &self,
        params: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_delete_signed_request(CANCEL_ORDER_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_cancel_order_request_with_order_id_serialization() {
        let request = CancelOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(12345),
            orig_client_order_id: None,
            new_client_order_id: None,
            cancel_restrictions: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderId"], 12345);
        assert!(json.get("origClientOrderId").is_none());
        assert!(json.get("newClientOrderId").is_none());
        assert!(json.get("cancelRestrictions").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_cancel_order_request_with_orig_client_order_id_serialization() {
        let request = CancelOrderRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            orig_client_order_id: Some("my-order-123".to_string()),
            new_client_order_id: None,
            cancel_restrictions: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert!(json.get("orderId").is_none());
        assert_eq!(json["origClientOrderId"], "my-order-123");
        assert!(json.get("newClientOrderId").is_none());
        assert!(json.get("cancelRestrictions").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_cancel_order_request_with_all_fields_serialization() {
        let request = CancelOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(12345),
            orig_client_order_id: Some("original-123".to_string()),
            new_client_order_id: Some("cancel-456".to_string()),
            cancel_restrictions: Some(CancelRestrictions::OnlyNew),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderId"], 12345);
        assert_eq!(json["origClientOrderId"], "original-123");
        assert_eq!(json["newClientOrderId"], "cancel-456");
        assert_eq!(json["cancelRestrictions"], "ONLY_NEW");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_cancel_order_request_with_only_partially_filled_restriction_serialization() {
        let request = CancelOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(98765),
            orig_client_order_id: None,
            new_client_order_id: None,
            cancel_restrictions: Some(CancelRestrictions::OnlyPartiallyFilled),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderId"], 98765);
        assert_eq!(json["cancelRestrictions"], "ONLY_PARTIALLY_FILLED");
    }

    #[test]
    fn test_cancel_order_request_both_ids_serialization() {
        // Test when both order_id and orig_client_order_id are provided
        let request = CancelOrderRequest {
            symbol: "BNBUSDT".to_string(),
            order_id: Some(555),
            orig_client_order_id: Some("client-555".to_string()),
            new_client_order_id: Some("new-client-555".to_string()),
            cancel_restrictions: None,
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert_eq!(json["orderId"], 555);
        assert_eq!(json["origClientOrderId"], "client-555");
        assert_eq!(json["newClientOrderId"], "new-client-555");
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_cancel_order_response_basic_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "origClientOrderId": "myOrder1",
            "orderId": 4,
            "orderListId": -1,
            "clientOrderId": "cancelMyOrder1",
            "transactTime": 1684804350068,
            "price": "2.00000000",
            "origQty": "1.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orig_client_order_id, "myOrder1");
        assert_eq!(response.order_id, 4);
        assert_eq!(response.order_list_id, -1);
        assert_eq!(response.client_order_id, "cancelMyOrder1");
        assert_eq!(response.transact_time, 1684804350068);
        assert_eq!(response.price.to_string(), "2.00000000");
        assert_eq!(response.orig_qty.to_string(), "1.00000000");
        assert_eq!(response.executed_qty.to_string(), "0.00000000");
        assert_eq!(response.cummulative_quote_qty.to_string(), "0.00000000");
        assert_eq!(response.status, OrderStatus::Canceled);
        assert_eq!(response.time_in_force, TimeInForce::GTC);
        assert_eq!(response.order_type, OrderType::Limit);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
        assert!(response.stop_price.is_none());
        assert!(response.iceberg_qty.is_none());
    }

    #[test]
    fn test_cancel_order_response_with_stop_price_deserialization() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "origClientOrderId": "stopOrder123",
            "orderId": 789,
            "orderListId": -1,
            "clientOrderId": "cancelStop123",
            "transactTime": 1684804350068,
            "price": "3000.00000000",
            "origQty": "0.50000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "STOP_LOSS_LIMIT",
            "side": "SELL",
            "stopPrice": "2950.00000000",
            "selfTradePreventionMode": "EXPIRE_TAKER"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSDT");
        assert_eq!(response.order_type, OrderType::StopLossLimit);
        assert_eq!(response.stop_price.unwrap().to_string(), "2950.00000000");
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );
    }

    #[test]
    fn test_cancel_order_response_with_iceberg_qty_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "origClientOrderId": "icebergOrder1",
            "orderId": 999,
            "orderListId": -1,
            "clientOrderId": "cancelIceberg1",
            "transactTime": 1684804350068,
            "price": "50000.00000000",
            "origQty": "10.00000000",
            "executedQty": "2.00000000",
            "cummulativeQuoteQty": "100000.00000000",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "icebergQty": "1.00000000",
            "selfTradePreventionMode": "EXPIRE_BOTH"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.iceberg_qty.unwrap().to_string(), "1.00000000");
        assert_eq!(response.executed_qty.to_string(), "2.00000000");
        assert_eq!(
            response.cummulative_quote_qty.to_string(),
            "100000.00000000"
        );
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        );
    }

    #[test]
    fn test_cancel_order_response_partially_filled_deserialization() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "origClientOrderId": "partialOrder1",
            "orderId": 111,
            "orderListId": -1,
            "clientOrderId": "cancelPartial1",
            "transactTime": 1684804350068,
            "price": "300.00000000",
            "origQty": "5.00000000",
            "executedQty": "2.50000000",
            "cummulativeQuoteQty": "750.00000000",
            "status": "CANCELED",
            "timeInForce": "IOC",
            "type": "LIMIT",
            "side": "SELL",
            "selfTradePreventionMode": "EXPIRE_MAKER"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BNBUSDT");
        assert_eq!(response.time_in_force, TimeInForce::IOC);
        assert_eq!(response.executed_qty.to_string(), "2.50000000");
        assert_eq!(response.orig_qty.to_string(), "5.00000000");
        // Verify partial fill
        assert!(response.executed_qty > dec!(0));
        assert!(response.executed_qty < response.orig_qty);
    }

    #[test]
    fn test_cancel_order_response_market_order_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "origClientOrderId": "marketOrder1",
            "orderId": 222,
            "orderListId": -1,
            "clientOrderId": "cancelMarket1",
            "transactTime": 1684804350068,
            "price": "0.00000000",
            "origQty": "0.10000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "CANCELED",
            "timeInForce": "FOK",
            "type": "MARKET",
            "side": "BUY",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_type, OrderType::Market);
        assert_eq!(response.time_in_force, TimeInForce::FOK);
        assert_eq!(response.price.to_string(), "0.00000000");
    }

    #[test]
    fn test_cancel_order_response_all_order_types_deserialization() {
        // Test various order types
        let order_types = vec![
            ("LIMIT", OrderType::Limit),
            ("MARKET", OrderType::Market),
            ("STOP_LOSS", OrderType::StopLoss),
            ("STOP_LOSS_LIMIT", OrderType::StopLossLimit),
            ("TAKE_PROFIT", OrderType::TakeProfit),
            ("TAKE_PROFIT_LIMIT", OrderType::TakeProfitLimit),
            ("LIMIT_MAKER", OrderType::LimitMaker),
        ];

        for (type_str, expected_type) in order_types {
            let json = format!(
                r#"{{
                    "symbol": "BTCUSDT",
                    "origClientOrderId": "order1",
                    "orderId": 1,
                    "orderListId": -1,
                    "clientOrderId": "cancel1",
                    "transactTime": 1684804350068,
                    "price": "1.00000000",
                    "origQty": "1.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "CANCELED",
                    "timeInForce": "GTC",
                    "type": "{}",
                    "side": "BUY",
                    "selfTradePreventionMode": "NONE"
                }}"#,
                type_str
            );

            let response: CancelOrderResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.order_type, expected_type);
        }
    }

    #[test]
    fn test_cancel_order_response_all_self_trade_prevention_modes_deserialization() {
        let modes = vec![
            ("NONE", SelfTradePreventionMode::None),
            ("EXPIRE_TAKER", SelfTradePreventionMode::ExpireTaker),
            ("EXPIRE_MAKER", SelfTradePreventionMode::ExpireMaker),
            ("EXPIRE_BOTH", SelfTradePreventionMode::ExpireBoth),
        ];

        for (mode_str, expected_mode) in modes {
            let json = format!(
                r#"{{
                    "symbol": "BTCUSDT",
                    "origClientOrderId": "order1",
                    "orderId": 1,
                    "orderListId": -1,
                    "clientOrderId": "cancel1",
                    "transactTime": 1684804350068,
                    "price": "1.00000000",
                    "origQty": "1.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "CANCELED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "selfTradePreventionMode": "{}"
                }}"#,
                mode_str
            );

            let response: CancelOrderResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.self_trade_prevention_mode, expected_mode);
        }
    }

    #[test]
    fn test_cancel_restrictions_serialization() {
        // Test ONLY_NEW
        let request1 = CancelOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(123),
            orig_client_order_id: None,
            new_client_order_id: None,
            cancel_restrictions: Some(CancelRestrictions::OnlyNew),
            recv_window: None,
        };

        let json1 = serde_json::to_value(&request1).unwrap();
        assert_eq!(json1["cancelRestrictions"], "ONLY_NEW");

        // Test ONLY_PARTIALLY_FILLED
        let request2 = CancelOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(123),
            orig_client_order_id: None,
            new_client_order_id: None,
            cancel_restrictions: Some(CancelRestrictions::OnlyPartiallyFilled),
            recv_window: None,
        };

        let json2 = serde_json::to_value(&request2).unwrap();
        assert_eq!(json2["cancelRestrictions"], "ONLY_PARTIALLY_FILLED");
    }

    #[test]
    fn test_cancel_order_request_minimal_fields() {
        // Test with only required fields
        let request = CancelOrderRequest {
            symbol: "LTCUSDT".to_string(),
            order_id: Some(456),
            orig_client_order_id: None,
            new_client_order_id: None,
            cancel_restrictions: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        // Should only have symbol and orderId
        assert_eq!(json.as_object().unwrap().len(), 2);
        assert_eq!(json["symbol"], "LTCUSDT");
        assert_eq!(json["orderId"], 456);
    }

    #[test]
    fn test_cancel_order_response_precision_deserialization() {
        // Test high precision decimals
        let json = r#"{
            "symbol": "BTCUSDT",
            "origClientOrderId": "precision1",
            "orderId": 333,
            "orderListId": -1,
            "clientOrderId": "cancelPrecision1",
            "transactTime": 1684804350068,
            "price": "50123.45678901",
            "origQty": "0.12345678",
            "executedQty": "0.08765432",
            "cummulativeQuoteQty": "4392.87654321",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.price.to_string(), "50123.45678901");
        assert_eq!(response.orig_qty.to_string(), "0.12345678");
        assert_eq!(response.executed_qty.to_string(), "0.08765432");
        assert_eq!(response.cummulative_quote_qty.to_string(), "4392.87654321");
    }
}
