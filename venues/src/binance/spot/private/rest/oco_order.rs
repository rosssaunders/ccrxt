use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderResponseType, OrderSide,
    RestResult, SelfTradePreventionMode, TimeInForce,
};

const CREATE_OCO_ORDER_ENDPOINT: &str = "/api/v3/order/oco";

/// Request parameters for OCO order
#[derive(Debug, Clone, Serialize)]
pub struct OcoOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// List client order ID
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,

    /// Order side (BUY or SELL)
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Limit client order ID
    #[serde(rename = "limitClientOrderId", skip_serializing_if = "Option::is_none")]
    pub limit_client_order_id: Option<String>,

    /// Limit order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Limit iceberg quantity
    #[serde(rename = "limitIcebergQty", skip_serializing_if = "Option::is_none")]
    pub limit_iceberg_qty: Option<Decimal>,

    /// Stop client order ID
    #[serde(rename = "stopClientOrderId", skip_serializing_if = "Option::is_none")]
    pub stop_client_order_id: Option<String>,

    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: Decimal,

    /// Stop limit price
    #[serde(rename = "stopLimitPrice", skip_serializing_if = "Option::is_none")]
    pub stop_limit_price: Option<Decimal>,

    /// Stop limit time in force
    #[serde(
        rename = "stopLimitTimeInForce",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_limit_time_in_force: Option<TimeInForce>,

    /// Stop iceberg quantity
    #[serde(rename = "stopIcebergQty", skip_serializing_if = "Option::is_none")]
    pub stop_iceberg_qty: Option<Decimal>,

    /// Stop strategy ID
    #[serde(rename = "stopStrategyId", skip_serializing_if = "Option::is_none")]
    pub stop_strategy_id: Option<u32>,

    /// Stop strategy type
    #[serde(rename = "stopStrategyType", skip_serializing_if = "Option::is_none")]
    pub stop_strategy_type: Option<u32>,

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

/// OCO order response
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrderResponse {
    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: u64,

    /// Contingency type
    #[serde(rename = "contingencyType")]
    pub contingency_type: ContingencyType,

    /// List status type
    #[serde(rename = "listStatusType")]
    pub list_status_type: OrderListStatus,

    /// List order status
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: OrderListOrderStatus,

    /// List client order ID
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: String,

    /// Transaction time
    #[serde(rename = "transactionTime")]
    pub transaction_time: u64,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Orders in the list
    #[serde(rename = "orders")]
    pub orders: Vec<OcoOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// OCO order information
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrder {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
}

impl RestClient {
    /// Send in a new OCO order
    ///
    /// Send in a new OCO (One-Cancels-Other) order.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-oco--trade)
    /// Method: POST /api/v3/order/oco
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_oco_order(&self, params: OcoOrderRequest) -> RestResult<OcoOrderResponse> {
        self.send_signed_request(
            CREATE_OCO_ORDER_ENDPOINT,
            reqwest::Method::POST,
            params,
            1,
            true,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_oco_order_request_minimal_serialization() {
        let request = OcoOrderRequest {
            symbol: "BTCUSDT".to_string(),
            list_client_order_id: None,
            side: OrderSide::Buy,
            quantity: dec!(0.1),
            limit_client_order_id: None,
            price: dec!(50000),
            limit_iceberg_qty: None,
            stop_client_order_id: None,
            stop_price: dec!(48000),
            stop_limit_price: None,
            stop_limit_time_in_force: None,
            stop_iceberg_qty: None,
            stop_strategy_id: None,
            stop_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["quantity"], "0.1");
        assert_eq!(json["price"], "50000");
        assert_eq!(json["stopPrice"], "48000");

        // Verify optional fields are not present
        assert!(json.get("listClientOrderId").is_none());
        assert!(json.get("limitClientOrderId").is_none());
        assert!(json.get("limitIcebergQty").is_none());
        assert!(json.get("stopClientOrderId").is_none());
        assert!(json.get("stopLimitPrice").is_none());
        assert!(json.get("stopLimitTimeInForce").is_none());
        assert!(json.get("stopIcebergQty").is_none());
        assert!(json.get("stopStrategyId").is_none());
        assert!(json.get("stopStrategyType").is_none());
        assert!(json.get("newOrderRespType").is_none());
        assert!(json.get("selfTradePreventionMode").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_oco_order_request_full_serialization() {
        let request = OcoOrderRequest {
            symbol: "ETHUSDT".to_string(),
            list_client_order_id: Some("my-oco-list-123".to_string()),
            side: OrderSide::Sell,
            quantity: dec!(2.5),
            limit_client_order_id: Some("limit-order-456".to_string()),
            price: dec!(3000.75),
            limit_iceberg_qty: Some(dec!(0.5)),
            stop_client_order_id: Some("stop-order-789".to_string()),
            stop_price: dec!(2900),
            stop_limit_price: Some(dec!(2895)),
            stop_limit_time_in_force: Some(TimeInForce::GTC),
            stop_iceberg_qty: Some(dec!(0.25)),
            stop_strategy_id: Some(12345),
            stop_strategy_type: Some(1000000),
            new_order_resp_type: Some(OrderResponseType::Full),
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireBoth),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["listClientOrderId"], "my-oco-list-123");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["quantity"], "2.5");
        assert_eq!(json["limitClientOrderId"], "limit-order-456");
        assert_eq!(json["price"], "3000.75");
        assert_eq!(json["limitIcebergQty"], "0.5");
        assert_eq!(json["stopClientOrderId"], "stop-order-789");
        assert_eq!(json["stopPrice"], "2900");
        assert_eq!(json["stopLimitPrice"], "2895");
        assert_eq!(json["stopLimitTimeInForce"], "GTC");
        assert_eq!(json["stopIcebergQty"], "0.25");
        assert_eq!(json["stopStrategyId"], 12345);
        assert_eq!(json["stopStrategyType"], 1000000);
        assert_eq!(json["newOrderRespType"], "FULL");
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_BOTH");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_oco_order_request_with_stop_limit_serialization() {
        let request = OcoOrderRequest {
            symbol: "BNBUSDT".to_string(),
            list_client_order_id: Some("bnb-oco-001".to_string()),
            side: OrderSide::Buy,
            quantity: dec!(10),
            limit_client_order_id: None,
            price: dec!(400),
            limit_iceberg_qty: None,
            stop_client_order_id: None,
            stop_price: dec!(420),
            stop_limit_price: Some(dec!(425)),
            stop_limit_time_in_force: Some(TimeInForce::IOC),
            stop_iceberg_qty: None,
            stop_strategy_id: None,
            stop_strategy_type: None,
            new_order_resp_type: Some(OrderResponseType::Ack),
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert_eq!(json["listClientOrderId"], "bnb-oco-001");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["quantity"], "10");
        assert_eq!(json["price"], "400");
        assert_eq!(json["stopPrice"], "420");
        assert_eq!(json["stopLimitPrice"], "425");
        assert_eq!(json["stopLimitTimeInForce"], "IOC");
        assert_eq!(json["newOrderRespType"], "ACK");
    }

    #[test]
    fn test_oco_order_request_with_iceberg_serialization() {
        let request = OcoOrderRequest {
            symbol: "ADAUSDT".to_string(),
            list_client_order_id: None,
            side: OrderSide::Sell,
            quantity: dec!(1000),
            limit_client_order_id: None,
            price: dec!(0.5),
            limit_iceberg_qty: Some(dec!(100)),
            stop_client_order_id: None,
            stop_price: dec!(0.45),
            stop_limit_price: None,
            stop_limit_time_in_force: None,
            stop_iceberg_qty: Some(dec!(50)),
            stop_strategy_id: None,
            stop_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ADAUSDT");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["quantity"], "1000");
        assert_eq!(json["price"], "0.5");
        assert_eq!(json["limitIcebergQty"], "100");
        assert_eq!(json["stopPrice"], "0.45");
        assert_eq!(json["stopIcebergQty"], "50");
    }

    #[test]
    fn test_oco_order_request_with_strategy_serialization() {
        let request = OcoOrderRequest {
            symbol: "SOLUSDT".to_string(),
            list_client_order_id: None,
            side: OrderSide::Buy,
            quantity: dec!(5),
            limit_client_order_id: None,
            price: dec!(100),
            limit_iceberg_qty: None,
            stop_client_order_id: None,
            stop_price: dec!(110),
            stop_limit_price: None,
            stop_limit_time_in_force: None,
            stop_iceberg_qty: None,
            stop_strategy_id: Some(99999),
            stop_strategy_type: Some(2000000),
            new_order_resp_type: None,
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "SOLUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["quantity"], "5");
        assert_eq!(json["price"], "100");
        assert_eq!(json["stopPrice"], "110");
        assert_eq!(json["stopStrategyId"], 99999);
        assert_eq!(json["stopStrategyType"], 2000000);
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_TAKER");
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_oco_order_response_deserialization() {
        let json = r#"{
            "orderListId": 12345678,
            "contingencyType": "OCO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "my-oco-list-123",
            "transactionTime": 1621234567890,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654321,
                    "clientOrderId": "limit-order-456"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654322,
                    "clientOrderId": "stop-order-789"
                }
            ],
            "orderReports": []
        }"#;

        let response: OcoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 12345678);
        assert_eq!(response.contingency_type, ContingencyType::Oco);
        assert_eq!(response.list_status_type, OrderListStatus::ExecStarted);
        assert_eq!(response.list_order_status, OrderListOrderStatus::Executing);
        assert_eq!(response.list_client_order_id, "my-oco-list-123");
        assert_eq!(response.transaction_time, 1621234567890);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.orders[0].symbol, "BTCUSDT");
        assert_eq!(response.orders[0].order_id, 987654321);
        assert_eq!(response.orders[0].client_order_id, "limit-order-456");
        assert_eq!(response.orders[1].symbol, "BTCUSDT");
        assert_eq!(response.orders[1].order_id, 987654322);
        assert_eq!(response.orders[1].client_order_id, "stop-order-789");
        assert_eq!(response.order_reports.len(), 0);
    }

    #[test]
    fn test_oco_order_response_with_order_reports_deserialization() {
        let json = r#"{
            "orderListId": 99887766,
            "contingencyType": "OCO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "eth-oco-999",
            "transactionTime": 1621234567999,
            "symbol": "ETHUSDT",
            "orders": [
                {
                    "symbol": "ETHUSDT",
                    "orderId": 111222333,
                    "clientOrderId": "eth-limit-111"
                },
                {
                    "symbol": "ETHUSDT",
                    "orderId": 111222334,
                    "clientOrderId": "eth-stop-222"
                }
            ],
            "orderReports": [
                {
                    "symbol": "ETHUSDT",
                    "orderId": 111222333,
                    "orderListId": 99887766,
                    "clientOrderId": "eth-limit-111",
                    "price": "3000.00",
                    "origQty": "1.00",
                    "executedQty": "1.00",
                    "cummulativeQuoteQty": "3000.00",
                    "status": "FILLED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "stopPrice": "0.00",
                    "selfTradePreventionMode": "NONE"
                },
                {
                    "symbol": "ETHUSDT",
                    "orderId": 111222334,
                    "orderListId": 99887766,
                    "clientOrderId": "eth-stop-222",
                    "price": "2900.00",
                    "origQty": "1.00",
                    "executedQty": "0.00",
                    "cummulativeQuoteQty": "0.00",
                    "status": "EXPIRED",
                    "timeInForce": "GTC",
                    "type": "STOP_LOSS_LIMIT",
                    "side": "BUY",
                    "stopPrice": "2950.00",
                    "selfTradePreventionMode": "NONE"
                }
            ]
        }"#;

        let response: OcoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 99887766);
        assert_eq!(response.contingency_type, ContingencyType::Oco);
        assert_eq!(response.list_status_type, OrderListStatus::AllDone);
        assert_eq!(response.list_order_status, OrderListOrderStatus::AllDone);
        assert_eq!(response.list_client_order_id, "eth-oco-999");
        assert_eq!(response.transaction_time, 1621234567999);
        assert_eq!(response.symbol, "ETHUSDT");
        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.order_reports.len(), 2);
    }

    #[test]
    fn test_oco_order_deserialization() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "orderId": 555666777,
            "clientOrderId": "bnb-order-xyz"
        }"#;

        let order: OcoOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BNBUSDT");
        assert_eq!(order.order_id, 555666777);
        assert_eq!(order.client_order_id, "bnb-order-xyz");
    }

    #[test]
    fn test_order_side_serialization() {
        assert_eq!(serde_json::to_string(&OrderSide::Buy).unwrap(), "\"BUY\"");
        assert_eq!(serde_json::to_string(&OrderSide::Sell).unwrap(), "\"SELL\"");
    }

    #[test]
    fn test_time_in_force_serialization() {
        assert_eq!(serde_json::to_string(&TimeInForce::GTC).unwrap(), "\"GTC\"");
        assert_eq!(serde_json::to_string(&TimeInForce::IOC).unwrap(), "\"IOC\"");
        assert_eq!(serde_json::to_string(&TimeInForce::FOK).unwrap(), "\"FOK\"");
    }

    #[test]
    fn test_order_response_type_serialization() {
        assert_eq!(
            serde_json::to_string(&OrderResponseType::Ack).unwrap(),
            "\"ACK\""
        );
        assert_eq!(
            serde_json::to_string(&OrderResponseType::Result).unwrap(),
            "\"RESULT\""
        );
        assert_eq!(
            serde_json::to_string(&OrderResponseType::Full).unwrap(),
            "\"FULL\""
        );
    }

    #[test]
    fn test_self_trade_prevention_mode_serialization() {
        assert_eq!(
            serde_json::to_string(&SelfTradePreventionMode::None).unwrap(),
            "\"NONE\""
        );
        assert_eq!(
            serde_json::to_string(&SelfTradePreventionMode::ExpireTaker).unwrap(),
            "\"EXPIRE_TAKER\""
        );
        assert_eq!(
            serde_json::to_string(&SelfTradePreventionMode::ExpireMaker).unwrap(),
            "\"EXPIRE_MAKER\""
        );
        assert_eq!(
            serde_json::to_string(&SelfTradePreventionMode::ExpireBoth).unwrap(),
            "\"EXPIRE_BOTH\""
        );
    }

    #[test]
    fn test_oco_order_request_decimal_precision() {
        let request = OcoOrderRequest {
            symbol: "BTCUSDT".to_string(),
            list_client_order_id: None,
            side: OrderSide::Buy,
            quantity: dec!(0.00012345),
            limit_client_order_id: None,
            price: dec!(49999.99),
            limit_iceberg_qty: Some(dec!(0.00001234)),
            stop_client_order_id: None,
            stop_price: dec!(48888.88),
            stop_limit_price: Some(dec!(48777.77)),
            stop_limit_time_in_force: None,
            stop_iceberg_qty: Some(dec!(0.00005678)),
            stop_strategy_id: None,
            stop_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["quantity"], "0.00012345");
        assert_eq!(json["price"], "49999.99");
        assert_eq!(json["limitIcebergQty"], "0.00001234");
        assert_eq!(json["stopPrice"], "48888.88");
        assert_eq!(json["stopLimitPrice"], "48777.77");
        assert_eq!(json["stopIcebergQty"], "0.00005678");
    }
}
