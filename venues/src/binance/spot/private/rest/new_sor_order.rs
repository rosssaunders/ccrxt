use rust_decimal::Decimal;
use serde::Serialize;

use super::client::RestClient;
use crate::binance::spot::{
    OrderResponseType, OrderSide, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

const CREATE_SOR_ORDER_ENDPOINT: &str = "/api/v3/sor/order";

/// Request parameters for SOR order
#[derive(Debug, Clone, Serialize)]
pub struct SorOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order side (BUY or SELL)
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,

    /// Order price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Client order ID
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Strategy ID
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u32>,

    /// Strategy type
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u32>,

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

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

impl RestClient {
    /// Place a SOR order
    ///
    /// Place an order using smart order routing.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-sor-order--trade)
    /// Method: POST /api/v3/sor/order
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_sor_order(&self, params: SorOrderRequest) -> RestResult<serde_json::Value> {
        self.send_post_signed_request(CREATE_SOR_ORDER_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_sor_order_request_basic_market_order_serialization() {
        let request = SorOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(dec!(0.001)),
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "MARKET");
        assert_eq!(json["quantity"], "0.001");
        // Test that optional fields are not included when None
        assert!(json.get("timeInForce").is_none());
        assert!(json.get("price").is_none());
        assert!(json.get("newClientOrderId").is_none());
        assert!(json.get("strategyId").is_none());
        assert!(json.get("strategyType").is_none());
        assert!(json.get("icebergQty").is_none());
        assert!(json.get("newOrderRespType").is_none());
        assert!(json.get("selfTradePreventionMode").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_sor_order_request_limit_order_with_all_fields_serialization() {
        let request = SorOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(0.5)),
            price: Some(dec!(3000.50)),
            new_client_order_id: Some("sor-order-123".to_string()),
            strategy_id: Some(12345),
            strategy_type: Some(1000000),
            iceberg_qty: Some(dec!(0.1)),
            new_order_resp_type: Some(OrderResponseType::Full),
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["timeInForce"], "GTC");
        assert_eq!(json["quantity"], "0.5");
        assert_eq!(json["price"], "3000.50");
        assert_eq!(json["newClientOrderId"], "sor-order-123");
        assert_eq!(json["strategyId"], 12345);
        assert_eq!(json["strategyType"], 1000000);
        assert_eq!(json["icebergQty"], "0.1");
        assert_eq!(json["newOrderRespType"], "FULL");
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_TAKER");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_sor_order_request_with_strategy_fields_serialization() {
        let request = SorOrderRequest {
            symbol: "BNBUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::IOC),
            quantity: Some(dec!(1.5)),
            price: Some(dec!(300.00)),
            new_client_order_id: None,
            strategy_id: Some(98765),
            strategy_type: Some(2000000),
            iceberg_qty: None,
            new_order_resp_type: Some(OrderResponseType::Ack),
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["timeInForce"], "IOC");
        assert_eq!(json["quantity"], "1.5");
        assert_eq!(json["price"], "300.00");
        assert_eq!(json["strategyId"], 98765);
        assert_eq!(json["strategyType"], 2000000);
        assert_eq!(json["newOrderRespType"], "ACK");
        assert!(json.get("newClientOrderId").is_none());
        assert!(json.get("icebergQty").is_none());
        assert!(json.get("selfTradePreventionMode").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_sor_order_response_deserialization_ack() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 123456789,
            "orderListId": -1,
            "clientOrderId": "sor-test-123",
            "transactTime": 1684123456789
        }"#;

        let response: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(response["symbol"], "BTCUSDT");
        assert_eq!(response["orderId"], 123456789);
        assert_eq!(response["orderListId"], -1);
        assert_eq!(response["clientOrderId"], "sor-test-123");
        assert_eq!(response["transactTime"], 1684123456789i64);
    }

    #[test]
    fn test_sor_order_response_deserialization_full() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 111222333,
            "orderListId": -1,
            "clientOrderId": "sor-full-789",
            "transactTime": 1684123456791,
            "price": "3000.00",
            "origQty": "0.5",
            "executedQty": "0.5",
            "cummulativeQuoteQty": "1500.00",
            "status": "FILLED",
            "timeInForce": "IOC",
            "type": "LIMIT",
            "side": "SELL",
            "workingTime": 1684123456791,
            "icebergQty": "0.00000000",
            "fills": [
                {
                    "price": "3000.00",
                    "qty": "0.3",
                    "commission": "0.0003",
                    "commissionAsset": "ETH",
                    "tradeId": 12345
                },
                {
                    "price": "3000.00",
                    "qty": "0.2",
                    "commission": "0.0002",
                    "commissionAsset": "ETH",
                    "tradeId": 12346
                }
            ],
            "selfTradePreventionMode": "EXPIRE_TAKER",
            "usedSor": true
        }"#;

        let response: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(response["symbol"], "ETHUSDT");
        assert_eq!(response["orderId"], 111222333);
        assert_eq!(response["executedQty"], "0.5");
        assert_eq!(response["status"], "FILLED");
        assert_eq!(response["selfTradePreventionMode"], "EXPIRE_TAKER");
        assert_eq!(response["usedSor"], true);

        // Check fills array
        assert!(response["fills"].is_array());
        let fills = response["fills"].as_array().unwrap();
        assert_eq!(fills.len(), 2);
        assert_eq!(fills[0]["price"], "3000.00");
        assert_eq!(fills[0]["qty"], "0.3");
        assert_eq!(fills[0]["commission"], "0.0003");
        assert_eq!(fills[0]["commissionAsset"], "ETH");
        assert_eq!(fills[0]["tradeId"], 12345);
        assert_eq!(fills[1]["qty"], "0.2");
    }

    #[test]
    fn test_large_decimal_values_serialization() {
        let request = SorOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(999999.123456789)),
            price: Some(dec!(0.00000001)),
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["quantity"], "999999.123456789");
        assert_eq!(json["price"], "0.00000001");
    }
}