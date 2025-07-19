use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

const ALL_ORDERS_ENDPOINT: &str = "/dapi/v1/allOrders";

/// Request parameters for all orders (GET /dapi/v1/allOrders).
#[derive(Debug, Clone, Serialize, Default)]
pub struct AllOrdersRequest {
    /// Trading symbol (e.g., "BTCUSD_200925"). Optional.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Trading pair (e.g., "BTCUSD"). Optional.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// Order ID. Optional.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Start time (ms since epoch). Optional.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time (ms since epoch). Optional.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Limit (default 50, max 100). Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// The value cannot be greater than 60000. Optional.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp (milliseconds since epoch).
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for all orders (GET /dapi/v1/allOrders).
#[derive(Debug, Clone, Deserialize)]
pub struct AllOrder {
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    #[serde(rename = "cumBase")]
    pub cum_base: String,

    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    #[serde(rename = "orderId")]
    pub order_id: u64,

    #[serde(rename = "origQty")]
    pub orig_qty: String,

    #[serde(rename = "origType")]
    pub orig_type: String,

    pub price: String,

    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    pub side: String,

    #[serde(rename = "positionSide")]
    pub position_side: String,

    pub status: String,

    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    #[serde(rename = "closePosition")]
    pub close_position: bool,

    pub symbol: String,

    pub pair: String,

    pub time: u64,

    #[serde(rename = "timeInForce")]
    pub time_in_force: String,

    #[serde(rename = "type")]
    pub order_type: String,

    #[serde(rename = "activatePrice")]
    pub activate_price: Option<String>,

    #[serde(rename = "priceRate")]
    pub price_rate: Option<String>,

    #[serde(rename = "updateTime")]
    pub update_time: u64,

    #[serde(rename = "workingType")]
    pub working_type: String,

    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    #[serde(rename = "priceMatch")]
    pub price_match: String,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
}

impl RestClient {
    /// Get all account orders (active, canceled, or filled) on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/All-Orders
    ///
    /// GET /dapi/v1/allOrders
    /// Weight: 20 with symbol, 40 with pair
    /// Requires API key and signature.
    pub async fn get_all_orders(&self, params: AllOrdersRequest) -> RestResult<Vec<AllOrder>> {
        let weight = if params.pair.is_some() { 40 } else { 20 };
        shared::send_signed_request(
            self,
            ALL_ORDERS_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_orders_request_serialization_with_symbol() {
        let request = AllOrdersRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            pair: None,
            order_id: Some(123456),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: 1625184000000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("orderId=123456"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625184000000"));
        assert!(!serialized.contains("pair="));
    }

    #[test]
    fn test_all_orders_request_serialization_with_pair() {
        let request = AllOrdersRequest {
            symbol: None,
            pair: Some("BTCUSD".to_string()),
            order_id: None,
            start_time: None,
            end_time: None,
            limit: Some(50),
            recv_window: None,
            timestamp: 1625184000000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSD"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("timestamp=1625184000000"));
        assert!(!serialized.contains("symbol="));
        assert!(!serialized.contains("orderId="));
        assert!(!serialized.contains("startTime="));
        assert!(!serialized.contains("endTime="));
        assert!(!serialized.contains("recvWindow="));
    }

    #[test]
    fn test_all_orders_request_minimal_serialization() {
        let request = AllOrdersRequest {
            symbol: Some("ETHUSD_PERP".to_string()),
            pair: None,
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1625184000000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSD_PERP&timestamp=1625184000000");
    }

    #[test]
    fn test_all_order_deserialization() {
        let json = r#"{
            "avgPrice": "0.0",
            "clientOrderId": "abc123",
            "cumBase": "0",
            "executedQty": "0",
            "orderId": 1917641,
            "origQty": "0.40",
            "origType": "TRAILING_STOP_MARKET",
            "price": "0",
            "reduceOnly": false,
            "side": "BUY",
            "positionSide": "SHORT",
            "status": "NEW",
            "stopPrice": "9300",
            "closePosition": false,
            "symbol": "BTCUSD_200925",
            "pair": "BTCUSD",
            "time": 1579276756075,
            "timeInForce": "GTC",
            "type": "TRAILING_STOP_MARKET",
            "activatePrice": "9020",
            "priceRate": "0.3",
            "updateTime": 1579276756075,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE"
        }"#;

        let order: AllOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.avg_price, "0.0");
        assert_eq!(order.client_order_id, "abc123");
        assert_eq!(order.cum_base, "0");
        assert_eq!(order.executed_qty, "0");
        assert_eq!(order.order_id, 1917641);
        assert_eq!(order.orig_qty, "0.40");
        assert_eq!(order.orig_type, "TRAILING_STOP_MARKET");
        assert_eq!(order.price, "0");
        assert!(!order.reduce_only);
        assert_eq!(order.side, "BUY");
        assert_eq!(order.position_side, "SHORT");
        assert_eq!(order.status, "NEW");
        assert_eq!(order.stop_price, "9300");
        assert!(!order.close_position);
        assert_eq!(order.symbol, "BTCUSD_200925");
        assert_eq!(order.pair, "BTCUSD");
        assert_eq!(order.time, 1579276756075);
        assert_eq!(order.time_in_force, "GTC");
        assert_eq!(order.order_type, "TRAILING_STOP_MARKET");
        assert_eq!(order.activate_price, Some("9020".to_string()));
        assert_eq!(order.price_rate, Some("0.3".to_string()));
        assert_eq!(order.update_time, 1579276756075);
        assert_eq!(order.working_type, "CONTRACT_PRICE");
        assert!(!order.price_protect);
        assert_eq!(order.price_match, "NONE");
        assert_eq!(order.self_trade_prevention_mode, "NONE");
    }

    #[test]
    fn test_all_order_deserialization_without_optional_fields() {
        let json = r#"{
            "avgPrice": "50000.0",
            "clientOrderId": "test_order_1",
            "cumBase": "0.1",
            "executedQty": "0.1",
            "orderId": 987654321,
            "origQty": "0.5",
            "origType": "LIMIT",
            "price": "50000",
            "reduceOnly": true,
            "side": "SELL",
            "positionSide": "LONG",
            "status": "FILLED",
            "stopPrice": "0",
            "closePosition": true,
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "time": 1625097600000,
            "timeInForce": "IOC",
            "type": "LIMIT",
            "updateTime": 1625097700000,
            "workingType": "MARK_PRICE",
            "priceProtect": true,
            "priceMatch": "QUEUE",
            "selfTradePreventionMode": "EXPIRE_MAKER"
        }"#;

        let order: AllOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.avg_price, "50000.0");
        assert_eq!(order.client_order_id, "test_order_1");
        assert_eq!(order.executed_qty, "0.1");
        assert_eq!(order.order_id, 987654321);
        assert!(order.reduce_only);
        assert_eq!(order.side, "SELL");
        assert_eq!(order.position_side, "LONG");
        assert_eq!(order.status, "FILLED");
        assert!(order.close_position);
        assert_eq!(order.symbol, "BTCUSD_PERP");
        assert_eq!(order.time_in_force, "IOC");
        assert_eq!(order.order_type, "LIMIT");
        assert!(order.activate_price.is_none());
        assert!(order.price_rate.is_none());
        assert_eq!(order.working_type, "MARK_PRICE");
        assert!(order.price_protect);
        assert_eq!(order.price_match, "QUEUE");
        assert_eq!(order.self_trade_prevention_mode, "EXPIRE_MAKER");
    }

    #[test]
    fn test_all_orders_list_deserialization() {
        let json = r#"[
            {
                "avgPrice": "0.0",
                "clientOrderId": "order1",
                "cumBase": "0",
                "executedQty": "0",
                "orderId": 111111,
                "origQty": "1.0",
                "origType": "LIMIT",
                "price": "45000",
                "reduceOnly": false,
                "side": "BUY",
                "positionSide": "LONG",
                "status": "NEW",
                "stopPrice": "0",
                "closePosition": false,
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "time": 1625097600000,
                "timeInForce": "GTC",
                "type": "LIMIT",
                "updateTime": 1625097600000,
                "workingType": "CONTRACT_PRICE",
                "priceProtect": false,
                "priceMatch": "NONE",
                "selfTradePreventionMode": "NONE"
            },
            {
                "avgPrice": "46000.0",
                "clientOrderId": "order2",
                "cumBase": "2.0",
                "executedQty": "2.0",
                "orderId": 222222,
                "origQty": "2.0",
                "origType": "MARKET",
                "price": "0",
                "reduceOnly": true,
                "side": "SELL",
                "positionSide": "LONG",
                "status": "FILLED",
                "stopPrice": "0",
                "closePosition": false,
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "time": 1625097700000,
                "timeInForce": "IOC",
                "type": "MARKET",
                "updateTime": 1625097700000,
                "workingType": "CONTRACT_PRICE",
                "priceProtect": false,
                "priceMatch": "NONE",
                "selfTradePreventionMode": "NONE"
            }
        ]"#;

        let orders: Vec<AllOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 2);

        assert_eq!(orders[0].order_id, 111111);
        assert_eq!(orders[0].side, "BUY");
        assert_eq!(orders[0].status, "NEW");
        assert_eq!(orders[0].order_type, "LIMIT");

        assert_eq!(orders[1].order_id, 222222);
        assert_eq!(orders[1].side, "SELL");
        assert_eq!(orders[1].status, "FILLED");
        assert_eq!(orders[1].order_type, "MARKET");
    }
}
