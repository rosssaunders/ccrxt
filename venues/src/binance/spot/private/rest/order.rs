use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::PrivateRestClient as RestClient;
use crate::binance::spot::{
    OrderResponseType, OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode,
    TimeInForce,
};

const CREATE_ORDER_ENDPOINT: &str = "/api/v3/order";

/// Request parameters for placing a new order
#[derive(Debug, Clone, Serialize)]
pub struct NewOrderRequest {
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

    /// Quote order quantity
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<Decimal>,

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

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Order fill information
#[derive(Debug, Clone, Deserialize)]
pub struct Fill {
    /// Trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Trade quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Commission amount
    #[serde(rename = "commission")]
    pub commission: Decimal,

    /// Commission asset
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,

    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: u64,
}

/// Order response (ACK type)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAckResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

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
}

/// Order response (RESULT type)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderResultResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

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

    /// Working time
    #[serde(rename = "workingTime")]
    pub working_time: u64,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

/// Order response (FULL type)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderFullResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

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

    /// Working time
    #[serde(rename = "workingTime")]
    pub working_time: u64,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    /// Fills
    #[serde(rename = "fills")]
    pub fills: Vec<Fill>,
}

impl RestClient {
    /// Place a new order
    ///
    /// Send in a new order.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#new-order--trade)
    ///
    /// Method: POST /api/v3/order
    /// Weight: 1
    /// Security: TRADE
    pub async fn new_order(&self, params: NewOrderRequest) -> RestResult<serde_json::Value> {
        self.send_post_signed_request(CREATE_ORDER_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_new_order_request_minimal_serialization() {
        let request = NewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(dec!(0.001)),
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
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
        assert!(json.get("timeInForce").is_none());
        assert!(json.get("price").is_none());
    }

    #[test]
    fn test_new_order_request_limit_order_serialization() {
        let request = NewOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(0.5)),
            quote_order_qty: None,
            price: Some(dec!(3000.50)),
            new_client_order_id: Some("my-order-123".to_string()),
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: Some(OrderResponseType::Full),
            self_trade_prevention_mode: None,
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["side"], "SELL");
        assert_eq!(json["type"], "LIMIT");
        assert_eq!(json["timeInForce"], "GTC");
        assert_eq!(json["quantity"], "0.5");
        assert_eq!(json["price"], "3000.50");
        assert_eq!(json["newClientOrderId"], "my-order-123");
        assert_eq!(json["newOrderRespType"], "FULL");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_new_order_request_stop_loss_serialization() {
        let request = NewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(0.1)),
            quote_order_qty: None,
            price: Some(dec!(45000)),
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: Some(dec!(45500)),
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["type"], "STOP_LOSS_LIMIT");
        assert_eq!(json["stopPrice"], "45500");
        assert_eq!(json["price"], "45000");
    }

    #[test]
    fn test_new_order_request_iceberg_order_serialization() {
        let request = NewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(dec!(10)),
            quote_order_qty: None,
            price: Some(dec!(50000)),
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: Some(dec!(1)),
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["icebergQty"], "1");
    }

    #[test]
    fn test_new_order_request_with_strategy_serialization() {
        let request = NewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::IOC),
            quantity: Some(dec!(0.5)),
            quote_order_qty: None,
            price: Some(dec!(48000)),
            new_client_order_id: None,
            strategy_id: Some(12345),
            strategy_type: Some(1000000),
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["strategyId"], 12345);
        assert_eq!(json["strategyType"], 1000000);
        assert_eq!(json["selfTradePreventionMode"], "EXPIRE_TAKER");
        assert_eq!(json["timeInForce"], "IOC");
    }

    #[test]
    fn test_new_order_request_quote_order_qty_serialization() {
        let request = NewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: None,
            quote_order_qty: Some(dec!(1000)),
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["quoteOrderQty"], "1000");
        assert!(json.get("quantity").is_none());
    }

    #[test]
    fn test_new_order_request_trailing_stop_serialization() {
        let request = NewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::StopLoss,
            time_in_force: None,
            quantity: Some(dec!(0.1)),
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: Some(200),
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["trailingDelta"], 200);
    }

    #[test]
    fn test_fill_deserialization() {
        let json = r#"{
            "price": "4000.00000000",
            "qty": "1.00000000",
            "commission": "1.00000000",
            "commissionAsset": "USDT",
            "tradeId": 12345
        }"#;

        let fill: Fill = serde_json::from_str(json).unwrap();
        assert_eq!(fill.price.to_string(), "4000.00000000");
        assert_eq!(fill.qty.to_string(), "1.00000000");
        assert_eq!(fill.commission.to_string(), "1.00000000");
        assert_eq!(fill.commission_asset, "USDT");
        assert_eq!(fill.trade_id, 12345);
    }

    #[test]
    fn test_order_ack_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 28,
            "orderListId": -1,
            "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
            "transactTime": 1507725176595
        }"#;

        let response: OrderAckResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 28);
        assert_eq!(response.order_list_id, -1);
        assert_eq!(response.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(response.transact_time, 1507725176595);
    }

    #[test]
    fn test_order_result_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 28,
            "orderListId": -1,
            "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
            "transactTime": 1507725176595,
            "price": "1.00000000",
            "origQty": "10.00000000",
            "executedQty": "10.00000000",
            "cummulativeQuoteQty": "10.00000000",
            "status": "FILLED",
            "timeInForce": "GTC",
            "type": "MARKET",
            "side": "SELL",
            "workingTime": 1507725176595,
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: OrderResultResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 28);
        assert_eq!(response.order_list_id, -1);
        assert_eq!(response.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(response.transact_time, 1507725176595);
        assert_eq!(response.price.to_string(), "1.00000000");
        assert_eq!(response.orig_qty.to_string(), "10.00000000");
        assert_eq!(response.executed_qty.to_string(), "10.00000000");
        assert_eq!(response.cummulative_quote_qty.to_string(), "10.00000000");
        assert_eq!(response.status, OrderStatus::Filled);
        assert_eq!(response.time_in_force, TimeInForce::GTC);
        assert_eq!(response.order_type, OrderType::Market);
        assert_eq!(response.side, OrderSide::Sell);
        assert_eq!(response.working_time, 1507725176595);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
    }

    #[test]
    fn test_order_full_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 28,
            "orderListId": -1,
            "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
            "transactTime": 1507725176595,
            "price": "1.00000000",
            "origQty": "10.00000000",
            "executedQty": "10.00000000",
            "cummulativeQuoteQty": "10.00000000",
            "status": "FILLED",
            "timeInForce": "GTC",
            "type": "MARKET",
            "side": "SELL",
            "workingTime": 1507725176595,
            "selfTradePreventionMode": "NONE",
            "fills": [
                {
                    "price": "4000.00000000",
                    "qty": "1.00000000",
                    "commission": "4.00000000",
                    "commissionAsset": "USDT",
                    "tradeId": 56
                },
                {
                    "price": "3999.00000000",
                    "qty": "5.00000000",
                    "commission": "19.99500000",
                    "commissionAsset": "USDT",
                    "tradeId": 57
                }
            ]
        }"#;

        let response: OrderFullResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 28);
        assert_eq!(response.fills.len(), 2);
        assert_eq!(response.fills[0].price.to_string(), "4000.00000000");
        assert_eq!(response.fills[0].qty.to_string(), "1.00000000");
        assert_eq!(response.fills[0].trade_id, 56);
        assert_eq!(response.fills[1].price.to_string(), "3999.00000000");
        assert_eq!(response.fills[1].qty.to_string(), "5.00000000");
        assert_eq!(response.fills[1].trade_id, 57);
    }

    #[test]
    fn test_order_full_response_with_optional_fields_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 28,
            "orderListId": -1,
            "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
            "transactTime": 1507725176595,
            "price": "50000.00000000",
            "origQty": "10.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "stopPrice": "49500.00000000",
            "icebergQty": "1.00000000",
            "workingTime": 1507725176595,
            "selfTradePreventionMode": "EXPIRE_BOTH",
            "fills": []
        }"#;

        let response: OrderFullResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.stop_price.unwrap().to_string(), "49500.00000000");
        assert_eq!(response.iceberg_qty.unwrap().to_string(), "1.00000000");
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        );
        assert!(response.fills.is_empty());
    }

    #[test]
    fn test_order_status_edge_cases_deserialization() {
        // Test all order statuses
        let statuses = vec![
            ("NEW", OrderStatus::New),
            ("PARTIALLY_FILLED", OrderStatus::PartiallyFilled),
            ("FILLED", OrderStatus::Filled),
            ("CANCELED", OrderStatus::Canceled),
            ("PENDING_CANCEL", OrderStatus::PendingCancel),
            ("REJECTED", OrderStatus::Rejected),
            ("EXPIRED", OrderStatus::Expired),
            ("EXPIRED_IN_MATCH", OrderStatus::ExpiredInMatch),
        ];

        for (status_str, expected_status) in statuses {
            let json = format!(r#"{{"status": "{}"}}"#, status_str);
            #[derive(Deserialize)]
            struct TestStatus {
                status: OrderStatus,
            }
            let result: TestStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(result.status, expected_status);
        }
    }

    #[test]
    fn test_order_type_edge_cases_serialization() {
        // Test all order types
        let order_types = vec![
            (OrderType::Limit, "LIMIT"),
            (OrderType::Market, "MARKET"),
            (OrderType::StopLoss, "STOP_LOSS"),
            (OrderType::StopLossLimit, "STOP_LOSS_LIMIT"),
            (OrderType::TakeProfit, "TAKE_PROFIT"),
            (OrderType::TakeProfitLimit, "TAKE_PROFIT_LIMIT"),
            (OrderType::LimitMaker, "LIMIT_MAKER"),
        ];

        for (order_type, expected_str) in order_types {
            let request = NewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type,
                time_in_force: None,
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: None,
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["type"], expected_str);
        }
    }

    #[test]
    fn test_time_in_force_serialization() {
        // Test all TimeInForce values
        let tif_values = vec![
            (TimeInForce::GTC, "GTC"),
            (TimeInForce::IOC, "IOC"),
            (TimeInForce::FOK, "FOK"),
        ];

        for (tif, expected_str) in tif_values {
            let request = NewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                time_in_force: Some(tif),
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: Some(dec!(50000)),
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["timeInForce"], expected_str);
        }
    }

    #[test]
    fn test_self_trade_prevention_mode_serialization() {
        // Test all SelfTradePreventionMode values
        let stp_modes = vec![
            (SelfTradePreventionMode::None, "NONE"),
            (SelfTradePreventionMode::ExpireTaker, "EXPIRE_TAKER"),
            (SelfTradePreventionMode::ExpireMaker, "EXPIRE_MAKER"),
            (SelfTradePreventionMode::ExpireBoth, "EXPIRE_BOTH"),
        ];

        for (stp_mode, expected_str) in stp_modes {
            let request = NewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                time_in_force: Some(TimeInForce::GTC),
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: Some(dec!(50000)),
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: None,
                self_trade_prevention_mode: Some(stp_mode),
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["selfTradePreventionMode"], expected_str);
        }
    }

    #[test]
    fn test_order_response_type_serialization() {
        // Test all OrderResponseType values
        let response_types = vec![
            (OrderResponseType::Ack, "ACK"),
            (OrderResponseType::Result, "RESULT"),
            (OrderResponseType::Full, "FULL"),
        ];

        for (resp_type, expected_str) in response_types {
            let request = NewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Market,
                time_in_force: None,
                quantity: Some(dec!(1)),
                quote_order_qty: None,
                price: None,
                new_client_order_id: None,
                strategy_id: None,
                strategy_type: None,
                stop_price: None,
                trailing_delta: None,
                iceberg_qty: None,
                new_order_resp_type: Some(resp_type),
                self_trade_prevention_mode: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["newOrderRespType"], expected_str);
        }
    }

    #[test]
    fn test_decimal_precision_in_responses() {
        // Test that decimal values maintain precision
        let json = r#"{
            "price": "12345.67890123",
            "qty": "0.00000001",
            "commission": "999999999.99999999",
            "commissionAsset": "BTC",
            "tradeId": 1
        }"#;

        let fill: Fill = serde_json::from_str(json).unwrap();
        assert_eq!(fill.price.to_string(), "12345.67890123");
        assert_eq!(fill.qty.to_string(), "0.00000001");
        assert_eq!(fill.commission.to_string(), "999999999.99999999");
    }

    #[test]
    fn test_large_order_id_deserialization() {
        // Test handling of large order IDs
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 9223372036854775807,
            "orderListId": -9223372036854775808,
            "clientOrderId": "test",
            "transactTime": 1507725176595
        }"#;

        let response: OrderAckResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 9223372036854775807u64);
        assert_eq!(response.order_list_id, -9223372036854775808i64);
    }
}
