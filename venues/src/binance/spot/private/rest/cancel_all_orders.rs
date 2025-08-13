use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/api/v3/openOrders";

/// Request parameters for cancelling all open orders
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrdersRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Cancel all orders response item
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResponseItem {
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
    /// Cancel all active orders on a symbol
    ///
    /// Cancels all active orders on a symbol.
    /// This includes OCO orders.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#cancel-all-open-orders-on-a-symbol--trade)
    ///
    /// Method: DELETE /api/v3/openOrders
    /// Weight: 1
    /// Security: TRADE
    pub async fn cancel_all_orders(
        &self,
        params: CancelAllOrdersRequest,
    ) -> RestResult<Vec<CancelAllOrdersResponseItem>> {
        self.send_delete_signed_request(CANCEL_ALL_ORDERS_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_cancel_all_orders_request_with_required_field_only() {
        let request = CancelAllOrdersRequest {
            symbol: "BTCUSDT".to_string(),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_cancel_all_orders_request_with_recv_window() {
        let request = CancelAllOrdersRequest {
            symbol: "ETHUSDT".to_string(),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_cancel_all_orders_response_array_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "origClientOrderId": "order1",
                "orderId": 123,
                "orderListId": -1,
                "clientOrderId": "cancelOrder1",
                "transactTime": 1684804350068,
                "price": "50000.00000000",
                "origQty": "1.00000000",
                "executedQty": "0.50000000",
                "cummulativeQuoteQty": "25000.00000000",
                "status": "CANCELED",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "selfTradePreventionMode": "NONE"
            },
            {
                "symbol": "BTCUSDT",
                "origClientOrderId": "order2",
                "orderId": 456,
                "orderListId": -1,
                "clientOrderId": "cancelOrder2",
                "transactTime": 1684804350069,
                "price": "49000.00000000",
                "origQty": "2.00000000",
                "executedQty": "0.00000000",
                "cummulativeQuoteQty": "0.00000000",
                "status": "CANCELED",
                "timeInForce": "IOC",
                "type": "LIMIT",
                "side": "SELL",
                "stopPrice": "48500.00000000",
                "selfTradePreventionMode": "EXPIRE_MAKER"
            }
        ]"#;

        let response: Vec<CancelAllOrdersResponseItem> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        // Check first order
        let first = &response[0];
        assert_eq!(first.symbol, "BTCUSDT");
        assert_eq!(first.orig_client_order_id, "order1");
        assert_eq!(first.order_id, 123);
        assert_eq!(first.order_list_id, -1);
        assert_eq!(first.client_order_id, "cancelOrder1");
        assert_eq!(first.transact_time, 1684804350068);
        assert_eq!(first.price, dec!(50000.00000000));
        assert_eq!(first.orig_qty, dec!(1.00000000));
        assert_eq!(first.executed_qty, dec!(0.50000000));
        assert_eq!(first.cummulative_quote_qty, dec!(25000.00000000));
        assert_eq!(first.status, OrderStatus::Canceled);
        assert_eq!(first.time_in_force, TimeInForce::GTC);
        assert_eq!(first.order_type, OrderType::Limit);
        assert_eq!(first.side, OrderSide::Buy);
        assert!(first.stop_price.is_none());
        assert!(first.iceberg_qty.is_none());
        assert_eq!(
            first.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );

        // Check second order
        let second = &response[1];
        assert_eq!(second.symbol, "BTCUSDT");
        assert_eq!(second.orig_client_order_id, "order2");
        assert_eq!(second.order_id, 456);
        assert_eq!(second.order_list_id, -1);
        assert_eq!(second.client_order_id, "cancelOrder2");
        assert_eq!(second.transact_time, 1684804350069);
        assert_eq!(second.price, dec!(49000.00000000));
        assert_eq!(second.orig_qty, dec!(2.00000000));
        assert_eq!(second.executed_qty, dec!(0.00000000));
        assert_eq!(second.cummulative_quote_qty, dec!(0.00000000));
        assert_eq!(second.status, OrderStatus::Canceled);
        assert_eq!(second.time_in_force, TimeInForce::IOC);
        assert_eq!(second.order_type, OrderType::Limit);
        assert_eq!(second.side, OrderSide::Sell);
        assert_eq!(second.stop_price, Some(dec!(48500.00000000)));
        assert!(second.iceberg_qty.is_none());
        assert_eq!(
            second.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireMaker
        );
    }

    #[test]
    fn test_cancel_all_orders_empty_response_array() {
        let json = r#"[]"#;
        let response: Vec<CancelAllOrdersResponseItem> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }

    #[test]
    fn test_cancel_all_orders_response_with_iceberg_qty() {
        let json = r#"[
            {
                "symbol": "BNBUSDT",
                "origClientOrderId": "icebergOrder",
                "orderId": 789,
                "orderListId": -1,
                "clientOrderId": "cancelIceberg",
                "transactTime": 1684804350070,
                "price": "350.00000000",
                "origQty": "100.00000000",
                "executedQty": "20.00000000",
                "cummulativeQuoteQty": "7000.00000000",
                "status": "CANCELED",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "icebergQty": "10.00000000",
                "selfTradePreventionMode": "EXPIRE_BOTH"
            }
        ]"#;

        let response: Vec<CancelAllOrdersResponseItem> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);

        let order = &response[0];
        assert_eq!(order.symbol, "BNBUSDT");
        assert_eq!(order.orig_client_order_id, "icebergOrder");
        assert_eq!(order.order_id, 789);
        assert_eq!(order.price, dec!(350.00000000));
        assert_eq!(order.orig_qty, dec!(100.00000000));
        assert_eq!(order.executed_qty, dec!(20.00000000));
        assert_eq!(order.cummulative_quote_qty, dec!(7000.00000000));
        assert_eq!(order.iceberg_qty, Some(dec!(10.00000000)));
        assert_eq!(
            order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        );
    }

    #[test]
    fn test_cancel_all_orders_response_various_order_types() {
        let json = r#"[
            {
                "symbol": "ADAUSDT",
                "origClientOrderId": "market1",
                "orderId": 111,
                "orderListId": -1,
                "clientOrderId": "cancelMarket1",
                "transactTime": 1684804350071,
                "price": "0.00000000",
                "origQty": "1000.00000000",
                "executedQty": "1000.00000000",
                "cummulativeQuoteQty": "350.00000000",
                "status": "CANCELED",
                "timeInForce": "GTC",
                "type": "MARKET",
                "side": "BUY",
                "selfTradePreventionMode": "NONE"
            },
            {
                "symbol": "SOLUSDT",
                "origClientOrderId": "stopLimit1",
                "orderId": 222,
                "orderListId": -1,
                "clientOrderId": "cancelStopLimit1",
                "transactTime": 1684804350072,
                "price": "25.00000000",
                "origQty": "50.00000000",
                "executedQty": "0.00000000",
                "cummulativeQuoteQty": "0.00000000",
                "status": "CANCELED",
                "timeInForce": "GTC",
                "type": "STOP_LOSS_LIMIT",
                "side": "SELL",
                "stopPrice": "24.50000000",
                "selfTradePreventionMode": "EXPIRE_TAKER"
            },
            {
                "symbol": "DOTUSDT",
                "origClientOrderId": "takeProfitLimit1",
                "orderId": 333,
                "orderListId": -1,
                "clientOrderId": "cancelTakeProfitLimit1",
                "transactTime": 1684804350073,
                "price": "7.00000000",
                "origQty": "100.00000000",
                "executedQty": "0.00000000",
                "cummulativeQuoteQty": "0.00000000",
                "status": "CANCELED",
                "timeInForce": "FOK",
                "type": "TAKE_PROFIT_LIMIT",
                "side": "SELL",
                "stopPrice": "6.80000000",
                "selfTradePreventionMode": "NONE"
            }
        ]"#;

        let response: Vec<CancelAllOrdersResponseItem> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 3);

        // Check MARKET order
        assert_eq!(response[0].order_type, OrderType::Market);
        assert_eq!(response[0].price, dec!(0.00000000));
        assert_eq!(response[0].executed_qty, dec!(1000.00000000));

        // Check STOP_LOSS_LIMIT order
        assert_eq!(response[1].order_type, OrderType::StopLossLimit);
        assert_eq!(response[1].stop_price, Some(dec!(24.50000000)));
        assert_eq!(response[1].time_in_force, TimeInForce::GTC);

        // Check TAKE_PROFIT_LIMIT order
        assert_eq!(response[2].order_type, OrderType::TakeProfitLimit);
        assert_eq!(response[2].stop_price, Some(dec!(6.80000000)));
        assert_eq!(response[2].time_in_force, TimeInForce::FOK);
    }

    #[test]
    fn test_cancel_all_orders_response_single_order() {
        let json = r#"[
            {
                "symbol": "XRPUSDT",
                "origClientOrderId": "singleOrder",
                "orderId": 999,
                "orderListId": -1,
                "clientOrderId": "cancelSingle",
                "transactTime": 1684804350074,
                "price": "0.50000000",
                "origQty": "2000.00000000",
                "executedQty": "500.00000000",
                "cummulativeQuoteQty": "250.00000000",
                "status": "CANCELED",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "selfTradePreventionMode": "NONE"
            }
        ]"#;

        let response: Vec<CancelAllOrdersResponseItem> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].symbol, "XRPUSDT");
        assert_eq!(response[0].order_id, 999);
        assert_eq!(response[0].executed_qty, dec!(500.00000000));
        assert_eq!(response[0].cummulative_quote_qty, dec!(250.00000000));
    }
}
