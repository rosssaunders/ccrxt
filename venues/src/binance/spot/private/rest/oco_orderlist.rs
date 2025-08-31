use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderResponseType, OrderSide,
    OrderType, RestResult, SelfTradePreventionMode, TimeInForce, private_client::RestClient,
};

const CREATE_OCO_ORDERLIST_ENDPOINT: &str = "/api/v3/orderList/oco";

/// Request parameters for OCO orderList
#[derive(Debug, Clone, Serialize)]
pub struct OcoOrderListRequest {
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

    /// Above order type (STOP_LOSS_LIMIT, LIMIT_MAKER)
    #[serde(rename = "aboveType")]
    pub above_type: OrderType,

    /// Above client order ID
    #[serde(rename = "aboveClientOrderId", skip_serializing_if = "Option::is_none")]
    pub above_client_order_id: Option<String>,

    /// Above iceberg quantity
    #[serde(rename = "aboveIcebergQty", skip_serializing_if = "Option::is_none")]
    pub above_iceberg_qty: Option<Decimal>,

    /// Above price
    #[serde(rename = "abovePrice", skip_serializing_if = "Option::is_none")]
    pub above_price: Option<Decimal>,

    /// Above stop price
    #[serde(rename = "aboveStopPrice", skip_serializing_if = "Option::is_none")]
    pub above_stop_price: Option<Decimal>,

    /// Above trailing delta
    #[serde(rename = "aboveTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub above_trailing_delta: Option<u32>,

    /// Above time in force
    #[serde(rename = "aboveTimeInForce", skip_serializing_if = "Option::is_none")]
    pub above_time_in_force: Option<TimeInForce>,

    /// Above strategy ID
    #[serde(rename = "aboveStrategyId", skip_serializing_if = "Option::is_none")]
    pub above_strategy_id: Option<u32>,

    /// Above strategy type
    #[serde(rename = "aboveStrategyType", skip_serializing_if = "Option::is_none")]
    pub above_strategy_type: Option<u32>,

    /// Below order type (STOP_LOSS_LIMIT, LIMIT_MAKER)
    #[serde(rename = "belowType")]
    pub below_type: OrderType,

    /// Below client order ID
    #[serde(rename = "belowClientOrderId", skip_serializing_if = "Option::is_none")]
    pub below_client_order_id: Option<String>,

    /// Below iceberg quantity
    #[serde(rename = "belowIcebergQty", skip_serializing_if = "Option::is_none")]
    pub below_iceberg_qty: Option<Decimal>,

    /// Below price
    #[serde(rename = "belowPrice", skip_serializing_if = "Option::is_none")]
    pub below_price: Option<Decimal>,

    /// Below stop price
    #[serde(rename = "belowStopPrice", skip_serializing_if = "Option::is_none")]
    pub below_stop_price: Option<Decimal>,

    /// Below trailing delta
    #[serde(rename = "belowTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub below_trailing_delta: Option<u32>,

    /// Below time in force
    #[serde(rename = "belowTimeInForce", skip_serializing_if = "Option::is_none")]
    pub below_time_in_force: Option<TimeInForce>,

    /// Below strategy ID
    #[serde(rename = "belowStrategyId", skip_serializing_if = "Option::is_none")]
    pub below_strategy_id: Option<u32>,

    /// Below strategy type
    #[serde(rename = "belowStrategyType", skip_serializing_if = "Option::is_none")]
    pub below_strategy_type: Option<u32>,

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

/// OCO orderList response (same structure as OcoOrderResponse)
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrderListResponse {
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
    pub orders: Vec<OcoOrderListOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// OCO orderList order information
#[derive(Debug, Clone, Deserialize)]
pub struct OcoOrderListOrder {
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
    /// Place an OCO orderList
    ///
    /// Place an OCO pair.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-oco-orderlist--trade)
    ///
    /// Method: POST /api/v3/orderList/oco
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_oco_orderlist(
        &self,
        params: OcoOrderListRequest,
    ) -> RestResult<OcoOrderListResponse> {
        self.send_post_signed_request(CREATE_OCO_ORDERLIST_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_oco_orderlist_request_minimal_serialization() {
        // Test minimal request with only required fields
        let request = OcoOrderListRequest {
            symbol: "BTCUSDT".to_string(),
            list_client_order_id: None,
            side: OrderSide::Buy,
            quantity: dec!(1.0),
            above_type: OrderType::StopLossLimit,
            above_client_order_id: None,
            above_iceberg_qty: None,
            above_price: None,
            above_stop_price: None,
            above_trailing_delta: None,
            above_time_in_force: None,
            above_strategy_id: None,
            above_strategy_type: None,
            below_type: OrderType::LimitMaker,
            below_client_order_id: None,
            below_iceberg_qty: None,
            below_price: None,
            below_stop_price: None,
            below_trailing_delta: None,
            below_time_in_force: None,
            below_strategy_id: None,
            below_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["side"], "BUY");
        assert_eq!(json["quantity"], "1.0");
        assert_eq!(json["aboveType"], "STOP_LOSS_LIMIT");
        assert_eq!(json["belowType"], "LIMIT_MAKER");

        // Ensure optional fields are not serialized when None
        assert!(json.get("listClientOrderId").is_none());
        assert!(json.get("aboveClientOrderId").is_none());
        assert!(json.get("aboveIcebergQty").is_none());
        assert!(json.get("abovePrice").is_none());
        assert!(json.get("aboveStopPrice").is_none());
        assert!(json.get("aboveTrailingDelta").is_none());
        assert!(json.get("aboveTimeInForce").is_none());
        assert!(json.get("aboveStrategyId").is_none());
        assert!(json.get("aboveStrategyType").is_none());
        assert!(json.get("belowClientOrderId").is_none());
        assert!(json.get("belowIcebergQty").is_none());
        assert!(json.get("belowPrice").is_none());
        assert!(json.get("belowStopPrice").is_none());
        assert!(json.get("belowTrailingDelta").is_none());
        assert!(json.get("belowTimeInForce").is_none());
        assert!(json.get("belowStrategyId").is_none());
        assert!(json.get("belowStrategyType").is_none());
        assert!(json.get("newOrderRespType").is_none());
        assert!(json.get("selfTradePreventionMode").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_oco_orderlist_request_full_serialization() {
        // Test complete request with all fields populated
        let request = OcoOrderListRequest {
            symbol: "ETHUSDT".to_string(),
            list_client_order_id: Some("oco-list-123".to_string()),
            side: OrderSide::Sell,
            quantity: dec!(0.5),
            above_type: OrderType::StopLossLimit,
            above_client_order_id: Some("above-order-456".to_string()),
            above_iceberg_qty: Some(dec!(0.1)),
            above_price: Some(dec!(3500.0)),
            above_stop_price: Some(dec!(3550.0)),
            above_trailing_delta: Some(100),
            above_time_in_force: Some(TimeInForce::GTC),
            above_strategy_id: Some(12345),
            above_strategy_type: Some(1000000),
            below_type: OrderType::LimitMaker,
            below_client_order_id: Some("below-order-789".to_string()),
            below_iceberg_qty: Some(dec!(0.2)),
            below_price: Some(dec!(3000.0)),
            below_stop_price: Some(dec!(2950.0)),
            below_trailing_delta: Some(200),
            below_time_in_force: Some(TimeInForce::IOC),
            below_strategy_id: Some(67890),
            below_strategy_type: Some(2000000),
            new_order_resp_type: Some(OrderResponseType::Full),
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["listClientOrderId"], "oco-list-123");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["quantity"], "0.5");
        assert_eq!(json["aboveType"], "STOP_LOSS_LIMIT");
        assert_eq!(json["aboveClientOrderId"], "above-order-456");
        assert_eq!(json["aboveIcebergQty"], "0.1");
        assert_eq!(json["abovePrice"], "3500.0");
        assert_eq!(json["aboveStopPrice"], "3550.0");
        assert_eq!(json["aboveTrailingDelta"], 100);
        assert_eq!(json["aboveTimeInForce"], "GTC");
        assert_eq!(json["aboveStrategyId"], 12345);
        assert_eq!(json["aboveStrategyType"], 1000000);
        assert_eq!(json["belowType"], "LIMIT_MAKER");
        assert_eq!(json["belowClientOrderId"], "below-order-789");
        assert_eq!(json["belowIcebergQty"], "0.2");
        assert_eq!(json["belowPrice"], "3000.0");
        assert_eq!(json["belowStopPrice"], "2950.0");
        assert_eq!(json["belowTrailingDelta"], 200);
        assert_eq!(json["belowTimeInForce"], "IOC");
        assert_eq!(json["belowStrategyId"], 67890);
        assert_eq!(json["belowStrategyType"], 2000000);
        assert_eq!(json["newOrderRespType"], "FULL");
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_TAKER");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_oco_orderlist_request_above_orders_serialization() {
        // Test OCO with various above order types
        let order_types = vec![
            OrderType::StopLossLimit,
            OrderType::LimitMaker,
            OrderType::TakeProfitLimit,
        ];

        for order_type in order_types {
            let request = OcoOrderListRequest {
                symbol: "BTCUSDT".to_string(),
                list_client_order_id: None,
                side: OrderSide::Buy,
                quantity: dec!(1.0),
                above_type: order_type,
                above_client_order_id: None,
                above_iceberg_qty: None,
                above_price: Some(dec!(50000.0)),
                above_stop_price: Some(dec!(51000.0)),
                above_trailing_delta: None,
                above_time_in_force: Some(TimeInForce::GTC),
                above_strategy_id: None,
                above_strategy_type: None,
                below_type: OrderType::LimitMaker,
                below_client_order_id: None,
                below_iceberg_qty: None,
                below_price: Some(dec!(49000.0)),
                below_stop_price: None,
                below_trailing_delta: None,
                below_time_in_force: None,
                below_strategy_id: None,
                below_strategy_type: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["aboveType"], order_type.to_string());
        }
    }

    #[test]
    fn test_oco_orderlist_request_below_orders_serialization() {
        // Test OCO with various below order types
        let order_types = vec![
            OrderType::StopLossLimit,
            OrderType::LimitMaker,
            OrderType::TakeProfitLimit,
        ];

        for order_type in order_types {
            let request = OcoOrderListRequest {
                symbol: "BTCUSDT".to_string(),
                list_client_order_id: None,
                side: OrderSide::Buy,
                quantity: dec!(1.0),
                above_type: OrderType::StopLossLimit,
                above_client_order_id: None,
                above_iceberg_qty: None,
                above_price: Some(dec!(50000.0)),
                above_stop_price: Some(dec!(51000.0)),
                above_trailing_delta: None,
                above_time_in_force: Some(TimeInForce::GTC),
                above_strategy_id: None,
                above_strategy_type: None,
                below_type: order_type,
                below_client_order_id: None,
                below_iceberg_qty: None,
                below_price: Some(dec!(49000.0)),
                below_stop_price: if order_type == OrderType::StopLossLimit {
                    Some(dec!(48000.0))
                } else {
                    None
                },
                below_trailing_delta: None,
                below_time_in_force: if order_type == OrderType::StopLossLimit {
                    Some(TimeInForce::GTC)
                } else {
                    None
                },
                below_strategy_id: None,
                below_strategy_type: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["belowType"], order_type.to_string());
        }
    }

    #[test]
    fn test_oco_orderlist_request_order_sides() {
        // Test both BUY and SELL sides
        let sides = vec![(OrderSide::Buy, "BUY"), (OrderSide::Sell, "SELL")];

        for (side, expected_str) in sides {
            let request = OcoOrderListRequest {
                symbol: "BTCUSDT".to_string(),
                list_client_order_id: None,
                side,
                quantity: dec!(1.0),
                above_type: OrderType::StopLossLimit,
                above_client_order_id: None,
                above_iceberg_qty: None,
                above_price: None,
                above_stop_price: None,
                above_trailing_delta: None,
                above_time_in_force: None,
                above_strategy_id: None,
                above_strategy_type: None,
                below_type: OrderType::LimitMaker,
                below_client_order_id: None,
                below_iceberg_qty: None,
                below_price: None,
                below_stop_price: None,
                below_trailing_delta: None,
                below_time_in_force: None,
                below_strategy_id: None,
                below_strategy_type: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["side"], expected_str);
        }
    }

    #[test]
    fn test_oco_orderlist_request_all_time_in_force_values() {
        // Test all TimeInForce values for above and below orders
        let tif_values = vec![
            (TimeInForce::GTC, "GTC"),
            (TimeInForce::IOC, "IOC"),
            (TimeInForce::FOK, "FOK"),
        ];

        for (tif, expected_str) in tif_values {
            let request = OcoOrderListRequest {
                symbol: "BTCUSDT".to_string(),
                list_client_order_id: None,
                side: OrderSide::Buy,
                quantity: dec!(1.0),
                above_type: OrderType::StopLossLimit,
                above_client_order_id: None,
                above_iceberg_qty: None,
                above_price: None,
                above_stop_price: None,
                above_trailing_delta: None,
                above_time_in_force: Some(tif),
                above_strategy_id: None,
                above_strategy_type: None,
                below_type: OrderType::StopLossLimit,
                below_client_order_id: None,
                below_iceberg_qty: None,
                below_price: None,
                below_stop_price: None,
                below_trailing_delta: None,
                below_time_in_force: Some(tif),
                below_strategy_id: None,
                below_strategy_type: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["aboveTimeInForce"], expected_str);
            assert_eq!(json["belowTimeInForce"], expected_str);
        }
    }

    #[test]
    fn test_oco_orderlist_request_all_response_types() {
        // Test all OrderResponseType values
        let response_types = vec![
            (OrderResponseType::Ack, "ACK"),
            (OrderResponseType::Result, "RESULT"),
            (OrderResponseType::Full, "FULL"),
        ];

        for (resp_type, expected_str) in response_types {
            let request = OcoOrderListRequest {
                symbol: "BTCUSDT".to_string(),
                list_client_order_id: None,
                side: OrderSide::Buy,
                quantity: dec!(1.0),
                above_type: OrderType::StopLossLimit,
                above_client_order_id: None,
                above_iceberg_qty: None,
                above_price: None,
                above_stop_price: None,
                above_trailing_delta: None,
                above_time_in_force: None,
                above_strategy_id: None,
                above_strategy_type: None,
                below_type: OrderType::LimitMaker,
                below_client_order_id: None,
                below_iceberg_qty: None,
                below_price: None,
                below_stop_price: None,
                below_trailing_delta: None,
                below_time_in_force: None,
                below_strategy_id: None,
                below_strategy_type: None,
                new_order_resp_type: Some(resp_type),
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["newOrderRespType"], expected_str);
        }
    }

    #[test]
    fn test_oco_orderlist_request_all_self_trade_prevention_modes() {
        // Test all SelfTradePreventionMode values
        let stp_modes = vec![
            (SelfTradePreventionMode::None, "NONE"),
            (SelfTradePreventionMode::ExpireTaker, "EXPIRE_TAKER"),
            (SelfTradePreventionMode::ExpireMaker, "EXPIRE_MAKER"),
            (SelfTradePreventionMode::ExpireBoth, "EXPIRE_BOTH"),
        ];

        for (stp_mode, expected_str) in stp_modes {
            let request = OcoOrderListRequest {
                symbol: "BTCUSDT".to_string(),
                list_client_order_id: None,
                side: OrderSide::Buy,
                quantity: dec!(1.0),
                above_type: OrderType::StopLossLimit,
                above_client_order_id: None,
                above_iceberg_qty: None,
                above_price: None,
                above_stop_price: None,
                above_trailing_delta: None,
                above_time_in_force: None,
                above_strategy_id: None,
                above_strategy_type: None,
                below_type: OrderType::LimitMaker,
                below_client_order_id: None,
                below_iceberg_qty: None,
                below_price: None,
                below_stop_price: None,
                below_trailing_delta: None,
                below_time_in_force: None,
                below_strategy_id: None,
                below_strategy_type: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: Some(stp_mode),
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["selfTradePreventionMode"], expected_str);
        }
    }

    #[test]
    fn test_oco_orderlist_request_decimal_precision() {
        // Test decimal precision handling
        let request = OcoOrderListRequest {
            symbol: "BTCUSDT".to_string(),
            list_client_order_id: None,
            side: OrderSide::Buy,
            quantity: dec!(0.00000001),
            above_type: OrderType::StopLossLimit,
            above_client_order_id: None,
            above_iceberg_qty: Some(dec!(0.000001)),
            above_price: Some(dec!(12345.67890123)),
            above_stop_price: Some(dec!(12350.12345678)),
            above_trailing_delta: None,
            above_time_in_force: None,
            above_strategy_id: None,
            above_strategy_type: None,
            below_type: OrderType::LimitMaker,
            below_client_order_id: None,
            below_iceberg_qty: Some(dec!(0.000002)),
            below_price: Some(dec!(12340.98765432)),
            below_stop_price: Some(dec!(12335.11111111)),
            below_trailing_delta: None,
            below_time_in_force: None,
            below_strategy_id: None,
            below_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["quantity"], "0.00000001");
        assert_eq!(json["aboveIcebergQty"], "0.000001");
        assert_eq!(json["abovePrice"], "12345.67890123");
        assert_eq!(json["aboveStopPrice"], "12350.12345678");
        assert_eq!(json["belowIcebergQty"], "0.000002");
        assert_eq!(json["belowPrice"], "12340.98765432");
        assert_eq!(json["belowStopPrice"], "12335.11111111");
    }

    #[test]
    fn test_oco_orderlist_request_large_values() {
        // Test large numeric values
        let request = OcoOrderListRequest {
            symbol: "BTCUSDT".to_string(),
            list_client_order_id: None,
            side: OrderSide::Buy,
            quantity: dec!(999999999.99999999),
            above_type: OrderType::StopLossLimit,
            above_client_order_id: None,
            above_iceberg_qty: None,
            above_price: Some(dec!(999999999.99999999)),
            above_stop_price: Some(dec!(999999999.99999999)),
            above_trailing_delta: Some(999999),
            above_time_in_force: None,
            above_strategy_id: Some(4294967295),   // Max u32
            above_strategy_type: Some(4294967295), // Max u32
            below_type: OrderType::LimitMaker,
            below_client_order_id: None,
            below_iceberg_qty: None,
            below_price: Some(dec!(999999999.99999999)),
            below_stop_price: Some(dec!(999999999.99999999)),
            below_trailing_delta: Some(999999),
            below_time_in_force: None,
            below_strategy_id: Some(4294967295),   // Max u32
            below_strategy_type: Some(4294967295), // Max u32
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: Some(60000), // Max recv window
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["quantity"], "999999999.99999999");
        assert_eq!(json["abovePrice"], "999999999.99999999");
        assert_eq!(json["aboveStopPrice"], "999999999.99999999");
        assert_eq!(json["aboveTrailingDelta"], 999999);
        assert_eq!(json["aboveStrategyId"], 4294967295u32);
        assert_eq!(json["aboveStrategyType"], 4294967295u32);
        assert_eq!(json["belowPrice"], "999999999.99999999");
        assert_eq!(json["belowStopPrice"], "999999999.99999999");
        assert_eq!(json["belowTrailingDelta"], 999999);
        assert_eq!(json["belowStrategyId"], 4294967295u32);
        assert_eq!(json["belowStrategyType"], 4294967295u32);
        assert_eq!(json["recvWindow"], 60000);
    }

    #[test]
    fn test_oco_orderlist_request_trailing_delta_serialization() {
        // Test trailing delta functionality
        let request = OcoOrderListRequest {
            symbol: "BTCUSDT".to_string(),
            list_client_order_id: None,
            side: OrderSide::Sell,
            quantity: dec!(1.0),
            above_type: OrderType::StopLossLimit,
            above_client_order_id: None,
            above_iceberg_qty: None,
            above_price: None,
            above_stop_price: None,
            above_trailing_delta: Some(500),
            above_time_in_force: None,
            above_strategy_id: None,
            above_strategy_type: None,
            below_type: OrderType::LimitMaker,
            below_client_order_id: None,
            below_iceberg_qty: None,
            below_price: None,
            below_stop_price: None,
            below_trailing_delta: Some(300),
            below_time_in_force: None,
            below_strategy_id: None,
            below_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["aboveTrailingDelta"], 500);
        assert_eq!(json["belowTrailingDelta"], 300);
    }

    #[test]
    fn test_oco_orderlist_request_client_order_ids() {
        // Test various client order ID formats
        let client_ids = vec![
            "simple-id",
            "id-with-numbers-123",
            "ID_WITH_UNDERSCORES",
            "id.with.dots",
            "very-long-client-order-id-that-tests-the-limits",
        ];

        for client_id in client_ids {
            let request = OcoOrderListRequest {
                symbol: "BTCUSDT".to_string(),
                list_client_order_id: Some(format!("list-{}", client_id)),
                side: OrderSide::Buy,
                quantity: dec!(1.0),
                above_type: OrderType::StopLossLimit,
                above_client_order_id: Some(format!("above-{}", client_id)),
                above_iceberg_qty: None,
                above_price: None,
                above_stop_price: None,
                above_trailing_delta: None,
                above_time_in_force: None,
                above_strategy_id: None,
                above_strategy_type: None,
                below_type: OrderType::LimitMaker,
                below_client_order_id: Some(format!("below-{}", client_id)),
                below_iceberg_qty: None,
                below_price: None,
                below_stop_price: None,
                below_trailing_delta: None,
                below_time_in_force: None,
                below_strategy_id: None,
                below_strategy_type: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["listClientOrderId"], format!("list-{}", client_id));
            assert_eq!(json["aboveClientOrderId"], format!("above-{}", client_id));
            assert_eq!(json["belowClientOrderId"], format!("below-{}", client_id));
        }
    }

    #[test]
    fn test_oco_orderlist_response_deserialization() {
        let json = r#"{
            "orderListId": 123456789,
            "contingencyType": "OCO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "oco-client-order-123",
            "transactionTime": 1617181339494,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654321,
                    "clientOrderId": "above-order-id"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654322,
                    "clientOrderId": "below-order-id"
                }
            ],
            "orderReports": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654321,
                    "orderListId": 123456789,
                    "clientOrderId": "above-order-id",
                    "price": "50000.00000000",
                    "origQty": "0.50000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "STOP_LOSS_LIMIT",
                    "side": "SELL",
                    "stopPrice": "50500.00000000",
                    "workingTime": 1617181339494,
                    "selfTradePreventionMode": "NONE"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654322,
                    "orderListId": 123456789,
                    "clientOrderId": "below-order-id",
                    "price": "49500.00000000",
                    "origQty": "0.50000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "LIMIT_MAKER",
                    "side": "SELL",
                    "workingTime": 1617181339494,
                    "selfTradePreventionMode": "NONE"
                }
            ]
        }"#;

        let response: OcoOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 123456789);
        assert_eq!(response.contingency_type, ContingencyType::Oco);
        assert_eq!(response.list_status_type, OrderListStatus::ExecStarted);
        assert_eq!(response.list_order_status, OrderListOrderStatus::Executing);
        assert_eq!(response.list_client_order_id, "oco-client-order-123");
        assert_eq!(response.transaction_time, 1617181339494);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.order_reports.len(), 2);

        // Test orders
        assert_eq!(response.orders[0].symbol, "BTCUSDT");
        assert_eq!(response.orders[0].order_id, 987654321);
        assert_eq!(response.orders[0].client_order_id, "above-order-id");
        assert_eq!(response.orders[1].symbol, "BTCUSDT");
        assert_eq!(response.orders[1].order_id, 987654322);
        assert_eq!(response.orders[1].client_order_id, "below-order-id");
    }

    #[test]
    fn test_oco_orderlist_response_all_contingency_types() {
        // Test all ContingencyType values
        let contingency_types = vec![
            (ContingencyType::Oco, "OCO"),
            (ContingencyType::Oto, "OTO"),
            (ContingencyType::Otoco, "OTOCO"),
        ];

        for (contingency_type, json_str) in contingency_types {
            let json = format!(
                r#"{{
                    "orderListId": 123456789,
                    "contingencyType": "{}",
                    "listStatusType": "EXEC_STARTED",
                    "listOrderStatus": "EXECUTING",
                    "listClientOrderId": "test-client-order-id",
                    "transactionTime": 1617181339494,
                    "symbol": "BTCUSDT",
                    "orders": [],
                    "orderReports": []
                }}"#,
                json_str
            );

            let response: OcoOrderListResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.contingency_type, contingency_type);
        }
    }

    #[test]
    fn test_oco_orderlist_response_all_list_status_types() {
        // Test all OrderListStatus values
        let status_types = vec![
            (OrderListStatus::Response, "RESPONSE"),
            (OrderListStatus::ExecStarted, "EXEC_STARTED"),
            (OrderListStatus::AllDone, "ALL_DONE"),
            (OrderListStatus::Reject, "REJECT"),
        ];

        for (status_type, json_str) in status_types {
            let json = format!(
                r#"{{
                    "orderListId": 123456789,
                    "contingencyType": "OCO",
                    "listStatusType": "{}",
                    "listOrderStatus": "EXECUTING",
                    "listClientOrderId": "test-client-order-id",
                    "transactionTime": 1617181339494,
                    "symbol": "BTCUSDT",
                    "orders": [],
                    "orderReports": []
                }}"#,
                json_str
            );

            let response: OcoOrderListResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.list_status_type, status_type);
        }
    }

    #[test]
    fn test_oco_orderlist_response_all_order_status_types() {
        // Test all OrderListOrderStatus values
        let order_status_types = vec![
            (OrderListOrderStatus::Executing, "EXECUTING"),
            (OrderListOrderStatus::AllDone, "ALL_DONE"),
            (OrderListOrderStatus::Reject, "REJECT"),
        ];

        for (order_status_type, json_str) in order_status_types {
            let json = format!(
                r#"{{
                    "orderListId": 123456789,
                    "contingencyType": "OCO",
                    "listStatusType": "EXEC_STARTED",
                    "listOrderStatus": "{}",
                    "listClientOrderId": "test-client-order-id",
                    "transactionTime": 1617181339494,
                    "symbol": "BTCUSDT",
                    "orders": [],
                    "orderReports": []
                }}"#,
                json_str
            );

            let response: OcoOrderListResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.list_order_status, order_status_type);
        }
    }

    #[test]
    fn test_oco_orderlist_response_with_empty_arrays() {
        // Test response with empty orders and order reports arrays
        let json = r#"{
            "orderListId": 123456789,
            "contingencyType": "OCO",
            "listStatusType": "REJECT",
            "listOrderStatus": "REJECT",
            "listClientOrderId": "rejected-oco-order",
            "transactionTime": 1617181339494,
            "symbol": "BTCUSDT",
            "orders": [],
            "orderReports": []
        }"#;

        let response: OcoOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 123456789);
        assert_eq!(response.contingency_type, ContingencyType::Oco);
        assert_eq!(response.list_status_type, OrderListStatus::Reject);
        assert_eq!(response.list_order_status, OrderListOrderStatus::Reject);
        assert_eq!(response.list_client_order_id, "rejected-oco-order");
        assert_eq!(response.transaction_time, 1617181339494);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 0);
        assert_eq!(response.order_reports.len(), 0);
    }

    #[test]
    fn test_oco_orderlist_response_with_multiple_orders() {
        // Test response with multiple orders
        let json = r#"{
            "orderListId": 123456789,
            "contingencyType": "OCO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "multi-order-oco",
            "transactionTime": 1617181339494,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 1001,
                    "clientOrderId": "order-1"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 1002,
                    "clientOrderId": "order-2"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 1003,
                    "clientOrderId": "order-3"
                }
            ],
            "orderReports": [
                {"orderId": 1001, "status": "FILLED"},
                {"orderId": 1002, "status": "CANCELED"},
                {"orderId": 1003, "status": "NEW"}
            ]
        }"#;

        let response: OcoOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.orders.len(), 3);
        assert_eq!(response.order_reports.len(), 3);
        assert_eq!(response.orders[0].order_id, 1001);
        assert_eq!(response.orders[1].order_id, 1002);
        assert_eq!(response.orders[2].order_id, 1003);
        assert_eq!(response.orders[0].client_order_id, "order-1");
        assert_eq!(response.orders[1].client_order_id, "order-2");
        assert_eq!(response.orders[2].client_order_id, "order-3");
    }

    #[test]
    fn test_oco_orderlist_order_deserialization() {
        // Test OcoOrderListOrder deserialization
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 999888777,
            "clientOrderId": "my-custom-order-id-123"
        }"#;

        let order: OcoOrderListOrder = serde_json::from_str(json).unwrap();

        assert_eq!(order.symbol, "ETHUSDT");
        assert_eq!(order.order_id, 999888777);
        assert_eq!(order.client_order_id, "my-custom-order-id-123");
    }

    #[test]
    fn test_oco_orderlist_order_multiple_symbols() {
        // Test OCO order with different symbols (though typically they should be the same)
        let orders = vec![
            ("BTCUSDT", 12345, "btc-order"),
            ("ETHUSDT", 67890, "eth-order"),
            ("ADAUSDT", 11111, "ada-order"),
        ];

        for (symbol, order_id, client_order_id) in orders {
            let json = format!(
                r#"{{
                    "symbol": "{}",
                    "orderId": {},
                    "clientOrderId": "{}"
                }}"#,
                symbol, order_id, client_order_id
            );

            let order: OcoOrderListOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.symbol, symbol);
            assert_eq!(order.order_id, order_id);
            assert_eq!(order.client_order_id, client_order_id);
        }
    }

    #[test]
    fn test_oco_orderlist_response_large_values() {
        // Test response with large numeric values
        let json = r#"{
            "orderListId": 18446744073709551615,
            "contingencyType": "OCO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "large-values-test",
            "transactionTime": 9223372036854775807,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 18446744073709551615,
                    "clientOrderId": "large-order-id"
                }
            ],
            "orderReports": []
        }"#;

        let response: OcoOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 18446744073709551615);
        assert_eq!(response.transaction_time, 9223372036854775807);
        assert_eq!(response.orders[0].order_id, 18446744073709551615);
    }

    #[test]
    fn test_oco_orderlist_response_special_characters() {
        // Test response with special characters in string fields
        let json = r#"{
            "orderListId": 123456789,
            "contingencyType": "OCO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "special-chars-123_ABC.xyz",
            "transactionTime": 1617181339494,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 987654321,
                    "clientOrderId": "order-with-special_chars.123"
                }
            ],
            "orderReports": []
        }"#;

        let response: OcoOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.list_client_order_id, "special-chars-123_ABC.xyz");
        assert_eq!(
            response.orders[0].client_order_id,
            "order-with-special_chars.123"
        );
    }

    #[test]
    fn test_oco_orderlist_response_minimal_required_fields() {
        // Test response with only required fields
        let json = r#"{
            "orderListId": 123,
            "contingencyType": "OCO",
            "listStatusType": "RESPONSE",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "minimal-test",
            "transactionTime": 1617181339494,
            "symbol": "BTCUSDT",
            "orders": [],
            "orderReports": []
        }"#;

        let response: OcoOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 123);
        assert_eq!(response.contingency_type, ContingencyType::Oco);
        assert_eq!(response.list_status_type, OrderListStatus::Response);
        assert_eq!(response.list_order_status, OrderListOrderStatus::Executing);
        assert_eq!(response.list_client_order_id, "minimal-test");
        assert_eq!(response.transaction_time, 1617181339494);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 0);
        assert_eq!(response.order_reports.len(), 0);
    }
}
