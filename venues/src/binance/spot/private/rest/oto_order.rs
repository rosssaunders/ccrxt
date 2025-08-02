use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderResponseType, OrderSide,
    OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

const CREATE_OTO_ORDER_ENDPOINT: &str = "/api/v3/orderList/oto";

/// Request parameters for OTO order
#[derive(Debug, Clone, Serialize)]
pub struct OtoOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// List client order ID
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,

    /// Response type
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self-trade prevention mode
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    // Working order parameters
    /// Working order type
    #[serde(rename = "workingType")]
    pub working_type: OrderType,

    /// Working order side
    #[serde(rename = "workingSide")]
    pub working_side: OrderSide,

    /// Working client order ID
    #[serde(
        rename = "workingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub working_client_order_id: Option<String>,

    /// Working order quantity
    #[serde(rename = "workingQuantity")]
    pub working_quantity: Decimal,

    /// Working order price
    #[serde(rename = "workingPrice")]
    pub working_price: Decimal,

    /// Working time in force
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub working_time_in_force: Option<TimeInForce>,

    /// Working strategy ID
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    pub working_strategy_id: Option<u32>,

    /// Working strategy type
    #[serde(
        rename = "workingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub working_strategy_type: Option<u32>,

    /// Working iceberg quantity
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub working_iceberg_qty: Option<Decimal>,

    // Pending order parameters
    /// Pending order type
    #[serde(rename = "pendingType")]
    pub pending_type: OrderType,

    /// Pending order side
    #[serde(rename = "pendingSide")]
    pub pending_side: OrderSide,

    /// Pending client order ID
    #[serde(
        rename = "pendingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_client_order_id: Option<String>,

    /// Pending order quantity
    #[serde(rename = "pendingQuantity")]
    pub pending_quantity: Decimal,

    /// Pending order price
    #[serde(rename = "pendingPrice", skip_serializing_if = "Option::is_none")]
    pub pending_price: Option<Decimal>,

    /// Pending stop price
    #[serde(rename = "pendingStopPrice", skip_serializing_if = "Option::is_none")]
    pub pending_stop_price: Option<Decimal>,

    /// Pending trailing delta
    #[serde(
        rename = "pendingTrailingDelta",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_trailing_delta: Option<u32>,

    /// Pending time in force
    #[serde(rename = "pendingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub pending_time_in_force: Option<TimeInForce>,

    /// Pending strategy ID
    #[serde(rename = "pendingStrategyId", skip_serializing_if = "Option::is_none")]
    pub pending_strategy_id: Option<u32>,

    /// Pending strategy type
    #[serde(
        rename = "pendingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_strategy_type: Option<u32>,

    /// Pending iceberg quantity
    #[serde(rename = "pendingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub pending_iceberg_qty: Option<Decimal>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// OTO order response
#[derive(Debug, Clone, Deserialize)]
pub struct OtoOrderResponse {
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
    pub orders: Vec<OtoOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// OTO order information
#[derive(Debug, Clone, Deserialize)]
pub struct OtoOrder {
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
    /// Place an OTO order
    ///
    /// Place an OTO (One-Triggers-Other) order.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-oto--trade)
    /// Method: POST /api/v3/orderList/oto
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_oto_order(&self, params: OtoOrderRequest) -> RestResult<OtoOrderResponse> {
        self.send_post_signed_request(CREATE_OTO_ORDER_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_oto_order_request_minimal_serialization() {
        let request = OtoOrderRequest {
            symbol: "BTCUSDT".to_string(),
            list_client_order_id: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            working_type: OrderType::Limit,
            working_side: OrderSide::Buy,
            working_client_order_id: None,
            working_quantity: dec!(0.001),
            working_price: dec!(50000),
            working_time_in_force: None,
            working_strategy_id: None,
            working_strategy_type: None,
            working_iceberg_qty: None,
            pending_type: OrderType::Limit,
            pending_side: OrderSide::Sell,
            pending_client_order_id: None,
            pending_quantity: dec!(0.001),
            pending_price: Some(dec!(60000)),
            pending_stop_price: None,
            pending_trailing_delta: None,
            pending_time_in_force: None,
            pending_strategy_id: None,
            pending_strategy_type: None,
            pending_iceberg_qty: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["workingType"], "LIMIT");
        assert_eq!(json["workingSide"], "BUY");
        assert_eq!(json["workingQuantity"], "0.001");
        assert_eq!(json["workingPrice"], "50000");
        assert_eq!(json["pendingType"], "LIMIT");
        assert_eq!(json["pendingSide"], "SELL");
        assert_eq!(json["pendingQuantity"], "0.001");
        assert_eq!(json["pendingPrice"], "60000");

        // Verify optional fields are not present
        assert!(json.get("listClientOrderId").is_none());
        assert!(json.get("newOrderRespType").is_none());
        assert!(json.get("selfTradePreventionMode").is_none());
        assert!(json.get("workingClientOrderId").is_none());
        assert!(json.get("workingTimeInForce").is_none());
        assert!(json.get("pendingStopPrice").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_oto_order_request_full_serialization() {
        let request = OtoOrderRequest {
            symbol: "ETHUSDT".to_string(),
            list_client_order_id: Some("my-oto-list-123".to_string()),
            new_order_resp_type: Some(OrderResponseType::Full),
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            working_type: OrderType::Limit,
            working_side: OrderSide::Buy,
            working_client_order_id: Some("working-order-456".to_string()),
            working_quantity: dec!(1.5),
            working_price: dec!(3000.25),
            working_time_in_force: Some(TimeInForce::GTC),
            working_strategy_id: Some(12345),
            working_strategy_type: Some(1000000),
            working_iceberg_qty: Some(dec!(0.5)),
            pending_type: OrderType::StopLossLimit,
            pending_side: OrderSide::Sell,
            pending_client_order_id: Some("pending-order-789".to_string()),
            pending_quantity: dec!(1.5),
            pending_price: Some(dec!(2900)),
            pending_stop_price: Some(dec!(2950)),
            pending_trailing_delta: Some(100),
            pending_time_in_force: Some(TimeInForce::IOC),
            pending_strategy_id: Some(67890),
            pending_strategy_type: Some(2000000),
            pending_iceberg_qty: Some(dec!(0.25)),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["listClientOrderId"], "my-oto-list-123");
        assert_eq!(json["newOrderRespType"], "FULL");
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_TAKER");
        assert_eq!(json["workingType"], "LIMIT");
        assert_eq!(json["workingSide"], "BUY");
        assert_eq!(json["workingClientOrderId"], "working-order-456");
        assert_eq!(json["workingQuantity"], "1.5");
        assert_eq!(json["workingPrice"], "3000.25");
        assert_eq!(json["workingTimeInForce"], "GTC");
        assert_eq!(json["workingStrategyId"], 12345);
        assert_eq!(json["workingStrategyType"], 1000000);
        assert_eq!(json["workingIcebergQty"], "0.5");
        assert_eq!(json["pendingType"], "STOP_LOSS_LIMIT");
        assert_eq!(json["pendingSide"], "SELL");
        assert_eq!(json["pendingClientOrderId"], "pending-order-789");
        assert_eq!(json["pendingQuantity"], "1.5");
        assert_eq!(json["pendingPrice"], "2900");
        assert_eq!(json["pendingStopPrice"], "2950");
        assert_eq!(json["pendingTrailingDelta"], 100);
        assert_eq!(json["pendingTimeInForce"], "IOC");
        assert_eq!(json["pendingStrategyId"], 67890);
        assert_eq!(json["pendingStrategyType"], 2000000);
        assert_eq!(json["pendingIcebergQty"], "0.25");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_oto_order_request_market_orders_serialization() {
        let request = OtoOrderRequest {
            symbol: "BNBUSDT".to_string(),
            list_client_order_id: Some("bnb-oto-market".to_string()),
            new_order_resp_type: Some(OrderResponseType::Ack),
            self_trade_prevention_mode: None,
            working_type: OrderType::Market,
            working_side: OrderSide::Buy,
            working_client_order_id: None,
            working_quantity: dec!(10),
            working_price: dec!(0), // Market orders don't use price, but field is required
            working_time_in_force: None,
            working_strategy_id: None,
            working_strategy_type: None,
            working_iceberg_qty: None,
            pending_type: OrderType::Market,
            pending_side: OrderSide::Sell,
            pending_client_order_id: None,
            pending_quantity: dec!(10),
            pending_price: None,
            pending_stop_price: None,
            pending_trailing_delta: None,
            pending_time_in_force: None,
            pending_strategy_id: None,
            pending_strategy_type: None,
            pending_iceberg_qty: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert_eq!(json["listClientOrderId"], "bnb-oto-market");
        assert_eq!(json["newOrderRespType"], "ACK");
        assert_eq!(json["workingType"], "MARKET");
        assert_eq!(json["workingSide"], "BUY");
        assert_eq!(json["workingQuantity"], "10");
        assert_eq!(json["workingPrice"], "0");
        assert_eq!(json["pendingType"], "MARKET");
        assert_eq!(json["pendingSide"], "SELL");
        assert_eq!(json["pendingQuantity"], "10");
    }

    #[test]
    fn test_oto_order_request_stop_loss_limit_serialization() {
        let request = OtoOrderRequest {
            symbol: "ADAUSDT".to_string(),
            list_client_order_id: None,
            new_order_resp_type: Some(OrderResponseType::Result),
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireBoth),
            working_type: OrderType::Limit,
            working_side: OrderSide::Buy,
            working_client_order_id: Some("ada-limit-buy".to_string()),
            working_quantity: dec!(1000),
            working_price: dec!(0.5),
            working_time_in_force: Some(TimeInForce::FOK),
            working_strategy_id: None,
            working_strategy_type: None,
            working_iceberg_qty: None,
            pending_type: OrderType::StopLossLimit,
            pending_side: OrderSide::Sell,
            pending_client_order_id: Some("ada-stop-loss-limit".to_string()),
            pending_quantity: dec!(1000),
            pending_price: Some(dec!(0.44)),
            pending_stop_price: Some(dec!(0.45)),
            pending_trailing_delta: None,
            pending_time_in_force: Some(TimeInForce::GTC),
            pending_strategy_id: None,
            pending_strategy_type: None,
            pending_iceberg_qty: None,
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ADAUSDT");
        assert_eq!(json["newOrderRespType"], "RESULT");
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_BOTH");
        assert_eq!(json["workingType"], "LIMIT");
        assert_eq!(json["workingSide"], "BUY");
        assert_eq!(json["workingClientOrderId"], "ada-limit-buy");
        assert_eq!(json["workingQuantity"], "1000");
        assert_eq!(json["workingPrice"], "0.5");
        assert_eq!(json["workingTimeInForce"], "FOK");
        assert_eq!(json["pendingType"], "STOP_LOSS_LIMIT");
        assert_eq!(json["pendingSide"], "SELL");
        assert_eq!(json["pendingClientOrderId"], "ada-stop-loss-limit");
        assert_eq!(json["pendingQuantity"], "1000");
        assert_eq!(json["pendingPrice"], "0.44");
        assert_eq!(json["pendingStopPrice"], "0.45");
        assert_eq!(json["pendingTimeInForce"], "GTC");
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_oto_order_request_decimal_precision_serialization() {
        let request = OtoOrderRequest {
            symbol: "SHIBUSDT".to_string(),
            list_client_order_id: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            working_type: OrderType::Limit,
            working_side: OrderSide::Buy,
            working_client_order_id: None,
            working_quantity: dec!(1000000.123456789),
            working_price: dec!(0.00001234),
            working_time_in_force: None,
            working_strategy_id: None,
            working_strategy_type: None,
            working_iceberg_qty: Some(dec!(100000.987654321)),
            pending_type: OrderType::Limit,
            pending_side: OrderSide::Sell,
            pending_client_order_id: None,
            pending_quantity: dec!(1000000.123456789),
            pending_price: Some(dec!(0.00002345)),
            pending_stop_price: None,
            pending_trailing_delta: None,
            pending_time_in_force: None,
            pending_strategy_id: None,
            pending_strategy_type: None,
            pending_iceberg_qty: Some(dec!(50000.111111111)),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["workingQuantity"], "1000000.123456789");
        assert_eq!(json["workingPrice"], "0.00001234");
        assert_eq!(json["workingIcebergQty"], "100000.987654321");
        assert_eq!(json["pendingQuantity"], "1000000.123456789");
        assert_eq!(json["pendingPrice"], "0.00002345");
        assert_eq!(json["pendingIcebergQty"], "50000.111111111");
    }

    #[test]
    fn test_oto_order_response_minimal_deserialization() {
        let json = r#"{
            "orderListId": 12345678,
            "contingencyType": "OTO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "my-oto-list-123",
            "transactionTime": 1621234567890,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654321,
                    "clientOrderId": "working-order-456"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654322,
                    "clientOrderId": "pending-order-789"
                }
            ],
            "orderReports": []
        }"#;

        let response: OtoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 12345678);
        assert_eq!(response.contingency_type, ContingencyType::Oto);
        assert_eq!(response.list_status_type, OrderListStatus::ExecStarted);
        assert_eq!(response.list_order_status, OrderListOrderStatus::Executing);
        assert_eq!(response.list_client_order_id, "my-oto-list-123");
        assert_eq!(response.transaction_time, 1621234567890);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.orders[0].symbol, "BTCUSDT");
        assert_eq!(response.orders[0].order_id, 987654321);
        assert_eq!(response.orders[0].client_order_id, "working-order-456");
        assert_eq!(response.orders[1].symbol, "BTCUSDT");
        assert_eq!(response.orders[1].order_id, 987654322);
        assert_eq!(response.orders[1].client_order_id, "pending-order-789");
        assert_eq!(response.order_reports.len(), 0);
    }

    #[test]
    fn test_oto_order_response_with_order_reports_deserialization() {
        let json = r#"{
            "orderListId": 99887766,
            "contingencyType": "OTO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "eth-oto-999",
            "transactionTime": 1621234567999,
            "symbol": "ETHUSDT",
            "orders": [
                {
                    "symbol": "ETHUSDT",
                    "orderId": 111222333,
                    "clientOrderId": "eth-working-111"
                },
                {
                    "symbol": "ETHUSDT",
                    "orderId": 111222334,
                    "clientOrderId": "eth-pending-222"
                }
            ],
            "orderReports": [
                {
                    "symbol": "ETHUSDT",
                    "orderId": 111222333,
                    "orderListId": 99887766,
                    "clientOrderId": "eth-working-111",
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
                    "clientOrderId": "eth-pending-222",
                    "price": "3500.00",
                    "origQty": "1.00",
                    "executedQty": "1.00",
                    "cummulativeQuoteQty": "3500.00",
                    "status": "FILLED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "SELL",
                    "stopPrice": "0.00",
                    "selfTradePreventionMode": "NONE"
                }
            ]
        }"#;

        let response: OtoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 99887766);
        assert_eq!(response.contingency_type, ContingencyType::Oto);
        assert_eq!(response.list_status_type, OrderListStatus::AllDone);
        assert_eq!(response.list_order_status, OrderListOrderStatus::AllDone);
        assert_eq!(response.list_client_order_id, "eth-oto-999");
        assert_eq!(response.transaction_time, 1621234567999);
        assert_eq!(response.symbol, "ETHUSDT");
        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.order_reports.len(), 2);
    }

    #[test]
    fn test_oto_order_deserialization() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "orderId": 555666777,
            "clientOrderId": "bnb-order-xyz"
        }"#;

        let order: OtoOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BNBUSDT");
        assert_eq!(order.order_id, 555666777);
        assert_eq!(order.client_order_id, "bnb-order-xyz");
    }

    #[test]
    fn test_order_type_serialization() {
        assert_eq!(
            serde_json::to_string(&OrderType::Limit).unwrap(),
            "\"LIMIT\""
        );
        assert_eq!(
            serde_json::to_string(&OrderType::Market).unwrap(),
            "\"MARKET\""
        );
        assert_eq!(
            serde_json::to_string(&OrderType::StopLoss).unwrap(),
            "\"STOP_LOSS\""
        );
        assert_eq!(
            serde_json::to_string(&OrderType::StopLossLimit).unwrap(),
            "\"STOP_LOSS_LIMIT\""
        );
        assert_eq!(
            serde_json::to_string(&OrderType::TakeProfit).unwrap(),
            "\"TAKE_PROFIT\""
        );
        assert_eq!(
            serde_json::to_string(&OrderType::TakeProfitLimit).unwrap(),
            "\"TAKE_PROFIT_LIMIT\""
        );
        assert_eq!(
            serde_json::to_string(&OrderType::LimitMaker).unwrap(),
            "\"LIMIT_MAKER\""
        );
    }

    #[test]
    fn test_contingency_type_deserialization() {
        let json = "\"OTO\"";
        let contingency_type: ContingencyType = serde_json::from_str(json).unwrap();
        assert_eq!(contingency_type, ContingencyType::Oto);
    }

    #[test]
    fn test_order_list_status_deserialization() {
        let json = "\"EXEC_STARTED\"";
        let status: OrderListStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, OrderListStatus::ExecStarted);

        let json = "\"ALL_DONE\"";
        let status: OrderListStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, OrderListStatus::AllDone);

        let json = "\"RESPONSE\"";
        let status: OrderListStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, OrderListStatus::Response);

        let json = "\"REJECT\"";
        let status: OrderListStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, OrderListStatus::Reject);
    }

    #[test]
    fn test_order_list_order_status_deserialization() {
        let json = "\"EXECUTING\"";
        let status: OrderListOrderStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, OrderListOrderStatus::Executing);

        let json = "\"ALL_DONE\"";
        let status: OrderListOrderStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, OrderListOrderStatus::AllDone);

        let json = "\"REJECT\"";
        let status: OrderListOrderStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, OrderListOrderStatus::Reject);
    }
}
