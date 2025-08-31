use rust_decimal::Decimal;
use serde::Serialize;

use crate::binance::spot::{
    CancelReplaceMode, CancelRestrictions, OrderRateLimitExceededMode, OrderResponseType,
    OrderSide, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
    private_client::RestClient,
};

const CANCEL_REPLACE_ORDER_ENDPOINT: &str = "/api/v3/order/cancelReplace";

/// Request parameters for cancel replace order
#[derive(Debug, Clone, Serialize)]
pub struct CancelReplaceOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order side (BUY or SELL)
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Cancel replace mode
    #[serde(rename = "cancelReplaceMode")]
    pub cancel_replace_mode: CancelReplaceMode,

    /// Time in force
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,

    /// Quote order quantity
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<Decimal>,

    /// Order price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Cancel order ID
    #[serde(rename = "cancelOrderId", skip_serializing_if = "Option::is_none")]
    pub cancel_order_id: Option<u64>,

    /// Cancel original client order ID
    #[serde(
        rename = "cancelOrigClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel_orig_client_order_id: Option<String>,

    /// New cancel client order ID
    #[serde(
        rename = "cancelNewClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel_new_client_order_id: Option<String>,

    /// New client order ID
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Strategy ID
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u32>,

    /// Strategy type
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u32>,

    /// Stop price
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,

    /// Trailing delta
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<u32>,

    /// Iceberg quantity
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<Decimal>,

    /// Response type
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self-trade prevention mode
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Cancel restrictions
    #[serde(rename = "cancelRestrictions", skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestrictions>,

    /// Order rate limit exceeded mode
    #[serde(
        rename = "orderRateLimitExceededMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_rate_limit_exceeded_mode: Option<OrderRateLimitExceededMode>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

impl RestClient {
    /// Cancel and replace order
    ///
    /// Cancel an existing order and place a new order on the same symbol.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#cancel-and-replace-order--trade)
    ///
    /// Method: POST /api/v3/order/cancelReplace
    /// Weight: 1
    /// Security: TRADE
    pub async fn cancel_replace_order(
        &self,
        params: CancelReplaceOrderRequest,
    ) -> RestResult<serde_json::Value> {
        self.send_post_signed_request(CANCEL_REPLACE_ORDER_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_cancel_replace_order_request_minimal_serialization() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            cancel_replace_mode: CancelReplaceMode::StopOnFailure,
            time_in_force: None,
            quantity: None,
            quote_order_qty: None,
            price: None,
            cancel_order_id: None,
            cancel_orig_client_order_id: None,
            cancel_new_client_order_id: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            cancel_restrictions: None,
            order_rate_limit_exceeded_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["cancelReplaceMode"], "STOP_ON_FAILURE");

        // Verify optional fields are not present
        assert!(json.get("timeInForce").is_none());
        assert!(json.get("quantity").is_none());
        assert!(json.get("quoteOrderQty").is_none());
        assert!(json.get("price").is_none());
        assert!(json.get("cancelOrderId").is_none());
        assert!(json.get("cancelOrigClientOrderId").is_none());
        assert!(json.get("cancelNewClientOrderId").is_none());
        assert!(json.get("newClientOrderId").is_none());
        assert!(json.get("strategyId").is_none());
        assert!(json.get("strategyType").is_none());
        assert!(json.get("stopPrice").is_none());
        assert!(json.get("trailingDelta").is_none());
        assert!(json.get("icebergQty").is_none());
        assert!(json.get("newOrderRespType").is_none());
        assert!(json.get("selfTradePreventionMode").is_none());
        assert!(json.get("cancelRestrictions").is_none());
        assert!(json.get("orderRateLimitExceededMode").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_cancel_replace_order_request_full_serialization() {
        let request = CancelReplaceOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            cancel_replace_mode: CancelReplaceMode::AllowFailure,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(1.5)),
            quote_order_qty: Some(dec!(1500)),
            price: Some(dec!(1000.50)),
            cancel_order_id: Some(123456789),
            cancel_orig_client_order_id: Some("original-order-123".to_string()),
            cancel_new_client_order_id: Some("cancel-new-123".to_string()),
            new_client_order_id: Some("new-order-456".to_string()),
            strategy_id: Some(1000),
            strategy_type: Some(5),
            stop_price: Some(dec!(950.25)),
            trailing_delta: Some(100),
            iceberg_qty: Some(dec!(0.5)),
            new_order_resp_type: Some(OrderResponseType::Full),
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            cancel_restrictions: Some(CancelRestrictions::OnlyNew),
            order_rate_limit_exceeded_mode: Some(OrderRateLimitExceededMode::CancelOnly),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["cancelReplaceMode"], "ALLOW_FAILURE");
        assert_eq!(json["timeInForce"], "GTC");
        assert_eq!(json["quantity"], "1.5");
        assert_eq!(json["quoteOrderQty"], "1500");
        assert_eq!(json["price"], "1000.50");
        assert_eq!(json["cancelOrderId"], 123456789);
        assert_eq!(json["cancelOrigClientOrderId"], "original-order-123");
        assert_eq!(json["cancelNewClientOrderId"], "cancel-new-123");
        assert_eq!(json["newClientOrderId"], "new-order-456");
        assert_eq!(json["strategyId"], 1000);
        assert_eq!(json["strategyType"], 5);
        assert_eq!(json["stopPrice"], "950.25");
        assert_eq!(json["trailingDelta"], 100);
        assert_eq!(json["icebergQty"], "0.5");
        assert_eq!(json["newOrderRespType"], "FULL");
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_TAKER");
        assert_eq!(json["cancelRestrictions"], "ONLY_NEW");
        assert_eq!(json["orderRateLimitExceededMode"], "CANCEL_ONLY");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_cancel_replace_order_request_market_order() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            cancel_replace_mode: CancelReplaceMode::StopOnFailure,
            time_in_force: None,
            quantity: Some(dec!(0.001)),
            quote_order_qty: None,
            price: None,
            cancel_order_id: Some(987654321),
            cancel_orig_client_order_id: None,
            cancel_new_client_order_id: None,
            new_client_order_id: Some("market-order-123".to_string()),
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: Some(OrderResponseType::Ack),
            self_trade_prevention_mode: None,
            cancel_restrictions: None,
            order_rate_limit_exceeded_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "MARKET");
        assert_eq!(json["cancelReplaceMode"], "STOP_ON_FAILURE");
        assert_eq!(json["quantity"], "0.001");
        assert_eq!(json["cancelOrderId"], 987654321);
        assert_eq!(json["newClientOrderId"], "market-order-123");
        assert_eq!(json["newOrderRespType"], "ACK");
    }

    #[test]
    fn test_cancel_replace_order_request_stop_limit_order() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            cancel_replace_mode: CancelReplaceMode::AllowFailure,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(0.5)),
            quote_order_qty: None,
            price: Some(dec!(25000)),
            cancel_order_id: None,
            cancel_orig_client_order_id: Some("stop-order-789".to_string()),
            cancel_new_client_order_id: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: Some(dec!(24500)),
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: Some(OrderResponseType::Result),
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireMaker),
            cancel_restrictions: Some(CancelRestrictions::OnlyPartiallyFilled),
            order_rate_limit_exceeded_mode: Some(OrderRateLimitExceededMode::DoNothing),
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["type"], "STOP_LOSS_LIMIT");
        assert_eq!(json["cancelReplaceMode"], "ALLOW_FAILURE");
        assert_eq!(json["timeInForce"], "GTC");
        assert_eq!(json["quantity"], "0.5");
        assert_eq!(json["price"], "25000");
        assert_eq!(json["cancelOrigClientOrderId"], "stop-order-789");
        assert_eq!(json["stopPrice"], "24500");
        assert_eq!(json["newOrderRespType"], "RESULT");
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_MAKER");
        assert_eq!(json["cancelRestrictions"], "ONLY_PARTIALLY_FILLED");
        assert_eq!(json["orderRateLimitExceededMode"], "DO_NOTHING");
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_cancel_replace_order_request_with_strategy() {
        let request = CancelReplaceOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            cancel_replace_mode: CancelReplaceMode::StopOnFailure,
            time_in_force: Some(TimeInForce::IOC),
            quantity: Some(dec!(2.0)),
            quote_order_qty: None,
            price: Some(dec!(2000)),
            cancel_order_id: Some(111222333),
            cancel_orig_client_order_id: None,
            cancel_new_client_order_id: Some("cancel-strat-123".to_string()),
            new_client_order_id: Some("new-strat-456".to_string()),
            strategy_id: Some(5000),
            strategy_type: Some(10),
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            cancel_restrictions: None,
            order_rate_limit_exceeded_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["cancelReplaceMode"], "STOP_ON_FAILURE");
        assert_eq!(json["timeInForce"], "IOC");
        assert_eq!(json["quantity"], "2.0");
        assert_eq!(json["price"], "2000");
        assert_eq!(json["cancelOrderId"], 111222333);
        assert_eq!(json["cancelNewClientOrderId"], "cancel-strat-123");
        assert_eq!(json["newClientOrderId"], "new-strat-456");
        assert_eq!(json["strategyId"], 5000);
        assert_eq!(json["strategyType"], 10);
    }

    #[test]
    fn test_cancel_replace_order_request_iceberg() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            cancel_replace_mode: CancelReplaceMode::StopOnFailure,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(10)),
            quote_order_qty: None,
            price: Some(dec!(30000)),
            cancel_order_id: None,
            cancel_orig_client_order_id: Some("iceberg-order".to_string()),
            cancel_new_client_order_id: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: Some(dec!(1)),
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            cancel_restrictions: None,
            order_rate_limit_exceeded_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["cancelReplaceMode"], "STOP_ON_FAILURE");
        assert_eq!(json["timeInForce"], "GTC");
        assert_eq!(json["quantity"], "10");
        assert_eq!(json["price"], "30000");
        assert_eq!(json["cancelOrigClientOrderId"], "iceberg-order");
        assert_eq!(json["icebergQty"], "1");
    }

    #[test]
    fn test_cancel_replace_order_request_trailing_stop() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::StopLoss,
            cancel_replace_mode: CancelReplaceMode::AllowFailure,
            time_in_force: None,
            quantity: Some(dec!(0.1)),
            quote_order_qty: None,
            price: None,
            cancel_order_id: Some(999888777),
            cancel_orig_client_order_id: None,
            cancel_new_client_order_id: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: Some(dec!(29000)),
            trailing_delta: Some(500),
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            cancel_restrictions: None,
            order_rate_limit_exceeded_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["type"], "STOP_LOSS");
        assert_eq!(json["cancelReplaceMode"], "ALLOW_FAILURE");
        assert_eq!(json["quantity"], "0.1");
        assert_eq!(json["cancelOrderId"], 999888777);
        assert_eq!(json["stopPrice"], "29000");
        assert_eq!(json["trailingDelta"], 500);
    }

    #[test]
    fn test_cancel_replace_mode_serialization() {
        assert_eq!(
            serde_json::to_value(CancelReplaceMode::StopOnFailure).unwrap(),
            json!("STOP_ON_FAILURE")
        );
        assert_eq!(
            serde_json::to_value(CancelReplaceMode::AllowFailure).unwrap(),
            json!("ALLOW_FAILURE")
        );
    }

    #[test]
    fn test_cancel_restrictions_serialization() {
        assert_eq!(
            serde_json::to_value(CancelRestrictions::OnlyNew).unwrap(),
            json!("ONLY_NEW")
        );
        assert_eq!(
            serde_json::to_value(CancelRestrictions::OnlyPartiallyFilled).unwrap(),
            json!("ONLY_PARTIALLY_FILLED")
        );
    }

    #[test]
    fn test_order_rate_limit_exceeded_mode_serialization() {
        assert_eq!(
            serde_json::to_value(OrderRateLimitExceededMode::DoNothing).unwrap(),
            json!("DO_NOTHING")
        );
        assert_eq!(
            serde_json::to_value(OrderRateLimitExceededMode::CancelOnly).unwrap(),
            json!("CANCEL_ONLY")
        );
    }

    #[test]
    fn test_cancel_replace_order_response_deserialization_new_order_result() {
        // Test successful new order result response
        let response_json = json!({
            "cancelResult": "SUCCESS",
            "newOrderResult": "SUCCESS",
            "cancelResponse": {
                "symbol": "BTCUSDT",
                "origClientOrderId": "DnLo3vTAQcjha43lAZhZ0y",
                "orderId": 9,
                "orderListId": -1,
                "clientOrderId": "osxN3JXAtJvKvCqGeMWMVR",
                "transactTime": 1669277779555_i64,
                "price": "0.01000000",
                "origQty": "1000.00000000",
                "executedQty": "0.00000000",
                "cummulativeQuoteQty": "0.00000000",
                "status": "CANCELED",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "SELL",
                "selfTradePreventionMode": "NONE"
            },
            "newOrderResponse": {
                "symbol": "BTCUSDT",
                "orderId": 10,
                "orderListId": -1,
                "clientOrderId": "wOceeeOzNORyLiQfw7jd8S",
                "transactTime": 1669277779555_i64,
                "price": "0.02000000",
                "origQty": "1000.00000000",
                "executedQty": "0.00000000",
                "cummulativeQuoteQty": "0.00000000",
                "status": "NEW",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "workingTime": 1669277779555_i64,
                "fills": [],
                "selfTradePreventionMode": "NONE"
            }
        });

        // Verify the response can be parsed as a serde_json::Value
        let response: serde_json::Value = serde_json::from_value(response_json).unwrap();
        assert_eq!(response["cancelResult"], "SUCCESS");
        assert_eq!(response["newOrderResult"], "SUCCESS");
        assert_eq!(response["cancelResponse"]["symbol"], "BTCUSDT");
        assert_eq!(response["cancelResponse"]["status"], "CANCELED");
        assert_eq!(response["newOrderResponse"]["symbol"], "BTCUSDT");
        assert_eq!(response["newOrderResponse"]["status"], "NEW");
    }

    #[test]
    fn test_cancel_replace_order_response_deserialization_cancel_failure() {
        // Test when cancel fails
        let response_json = json!({
            "code": -2011,
            "msg": "Unknown order sent.",
            "data": {
                "cancelResult": "FAILURE",
                "newOrderResult": "NOT_ATTEMPTED",
                "cancelResponse": {
                    "code": -2011,
                    "msg": "Unknown order sent."
                },
                "newOrderResponse": null
            }
        });

        // Verify the response can be parsed as a serde_json::Value
        let response: serde_json::Value = serde_json::from_value(response_json).unwrap();
        assert_eq!(response["code"], -2011);
        assert_eq!(response["msg"], "Unknown order sent.");
        assert_eq!(response["data"]["cancelResult"], "FAILURE");
        assert_eq!(response["data"]["newOrderResult"], "NOT_ATTEMPTED");
        assert!(response["data"]["newOrderResponse"].is_null());
    }

    #[test]
    fn test_cancel_replace_order_response_deserialization_new_order_failure() {
        // Test when cancel succeeds but new order fails
        let response_json = json!({
            "code": -1013,
            "msg": "The quantity is too small.",
            "data": {
                "cancelResult": "SUCCESS",
                "newOrderResult": "FAILURE",
                "cancelResponse": {
                    "symbol": "BTCUSDT",
                    "origClientOrderId": "4d96324ff9d44481926157",
                    "orderId": 125690984230_i64,
                    "orderListId": -1,
                    "clientOrderId": "91fe37ce9e69c90d6358c0",
                    "transactTime": 1669277779555_i64,
                    "price": "0.01000000",
                    "origQty": "1000.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "CANCELED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "SELL",
                    "selfTradePreventionMode": "NONE"
                },
                "newOrderResponse": {
                    "code": -1013,
                    "msg": "The quantity is too small."
                }
            }
        });

        // Verify the response can be parsed as a serde_json::Value
        let response: serde_json::Value = serde_json::from_value(response_json).unwrap();
        assert_eq!(response["code"], -1013);
        assert_eq!(response["data"]["cancelResult"], "SUCCESS");
        assert_eq!(response["data"]["newOrderResult"], "FAILURE");
        assert_eq!(response["data"]["newOrderResponse"]["code"], -1013);
    }

    #[test]
    fn test_cancel_replace_order_request_with_quote_order_qty() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            cancel_replace_mode: CancelReplaceMode::StopOnFailure,
            time_in_force: None,
            quantity: None,
            quote_order_qty: Some(dec!(1000)),
            price: None,
            cancel_order_id: Some(12345),
            cancel_orig_client_order_id: None,
            cancel_new_client_order_id: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            cancel_restrictions: None,
            order_rate_limit_exceeded_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "MARKET");
        assert_eq!(json["quoteOrderQty"], "1000");
        assert!(json.get("quantity").is_none());
    }

    #[test]
    fn test_cancel_replace_order_request_all_self_trade_prevention_modes() {
        let modes = vec![
            SelfTradePreventionMode::None,
            SelfTradePreventionMode::ExpireTaker,
            SelfTradePreventionMode::ExpireMaker,
            SelfTradePreventionMode::ExpireBoth,
        ];

        for mode in modes {
            let request = CancelReplaceOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                cancel_replace_mode: CancelReplaceMode::StopOnFailure,
                time_in_force: Some(TimeInForce::GTC),
                quantity: Some(dec!(0.1)),
                quote_order_qty: None,
                price: Some(dec!(30000)),
                cancel_order_id: Some(123),
                cancel_orig_client_order_id: None,
                cancel_new_client_order_id: None,
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: Some(mode),
                cancel_restrictions: None,
                order_rate_limit_exceeded_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            let expected = match mode {
                SelfTradePreventionMode::None => "NONE",
                SelfTradePreventionMode::ExpireTaker => "EXPIRE_TAKER",
                SelfTradePreventionMode::ExpireMaker => "EXPIRE_MAKER",
                SelfTradePreventionMode::ExpireBoth => "EXPIRE_BOTH",
            };
            assert_eq!(json["selfTradePreventionMode"], expected);
        }
    }

    #[test]
    fn test_cancel_replace_order_request_all_order_response_types() {
        let response_types = vec![
            OrderResponseType::Ack,
            OrderResponseType::Result,
            OrderResponseType::Full,
        ];

        for resp_type in response_types {
            let request = CancelReplaceOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                cancel_replace_mode: CancelReplaceMode::StopOnFailure,
                time_in_force: Some(TimeInForce::GTC),
                quantity: Some(dec!(0.1)),
                quote_order_qty: None,
                price: Some(dec!(30000)),
                cancel_order_id: Some(123),
                cancel_orig_client_order_id: None,
                cancel_new_client_order_id: None,
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: Some(resp_type),
                self_trade_prevention_mode: None,
                cancel_restrictions: None,
                order_rate_limit_exceeded_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            let expected = match resp_type {
                OrderResponseType::Ack => "ACK",
                OrderResponseType::Result => "RESULT",
                OrderResponseType::Full => "FULL",
            };
            assert_eq!(json["newOrderRespType"], expected);
        }
    }
}
