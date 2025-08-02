use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private::rest::client::RestClient};

const OPEN_ORDERS_ENDPOINT: &str = "/dapi/v1/openOrders";

/// Request parameters for the Current All Open Orders endpoint
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenOrdersRequest {
    /// The trading symbol (e.g., "BTCUSD_200925").
    /// If not sent, will return orders for all symbols.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// The trading pair (e.g., "BTCUSD").
    /// If not sent, will return orders for all pairs.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// The number of milliseconds the request is valid for after timestamp.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Represents a single open order returned by GET /dapi/v1/openOrders.
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrder {
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

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    #[serde(rename = "side")]
    pub side: String,

    #[serde(rename = "positionSide")]
    pub position_side: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    #[serde(rename = "closePosition")]
    pub close_position: bool,

    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "time")]
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
    pub price_match: Option<String>,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<String>,
}

impl RestClient {
    /// Fetches all open orders for a symbol or all symbols.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Current-All-Open-Orders
    ///
    /// GET /dapi/v1/openOrders
    /// Weight: 1 for single symbol, 40 for multiple symbols
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`OpenOrdersRequest`])
    ///
    /// # Returns
    /// A vector of [`OpenOrder`] objects.
    pub async fn get_open_orders(&self, params: OpenOrdersRequest) -> RestResult<Vec<OpenOrder>> {
        let weight = if params.symbol.is_some() || params.pair.is_some() {
            1
        } else {
            40
        };
        self.send_get_signed_request(OPEN_ORDERS_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_orders_request_serialization() {
        let request = OpenOrdersRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            pair: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("pair"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_open_orders_request_with_pair() {
        let request = OpenOrdersRequest {
            symbol: None,
            pair: Some("BTCUSD".to_string()),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSD"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("symbol"));
    }

    #[test]
    fn test_open_orders_request_minimal() {
        let request = OpenOrdersRequest {
            symbol: None,
            pair: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timestamp=1625097600000");
    }

    #[test]
    fn test_open_order_deserialization() {
        let json = r#"[
            {
                "avgPrice": "0.00000",
                "clientOrderId": "abc123",
                "cumBase": "0",
                "executedQty": "0",
                "orderId": 123456789,
                "origQty": "10",
                "origType": "LIMIT",
                "price": "45000.0",
                "reduceOnly": false,
                "side": "BUY",
                "positionSide": "LONG",
                "status": "NEW",
                "stopPrice": "0",
                "closePosition": false,
                "symbol": "BTCUSD_PERP",
                "time": 1625097600000,
                "timeInForce": "GTC",
                "type": "LIMIT",
                "activatePrice": null,
                "priceRate": null,
                "updateTime": 1625097600000,
                "workingType": "CONTRACT_PRICE",
                "priceProtect": false,
                "priceMatch": "NONE",
                "selfTradePreventionMode": "NONE"
            }
        ]"#;

        let response: Vec<OpenOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);

        let order = &response[0];
        assert_eq!(order.avg_price, "0.00000");
        assert_eq!(order.client_order_id, "abc123");
        assert_eq!(order.cum_base, "0");
        assert_eq!(order.executed_qty, "0");
        assert_eq!(order.order_id, 123456789);
        assert_eq!(order.orig_qty, "10");
        assert_eq!(order.orig_type, "LIMIT");
        assert_eq!(order.price, "45000.0");
        assert!(!order.reduce_only);
        assert_eq!(order.side, "BUY");
        assert_eq!(order.position_side, "LONG");
        assert_eq!(order.status, "NEW");
        assert_eq!(order.stop_price, "0");
        assert!(!order.close_position);
        assert_eq!(order.symbol, "BTCUSD_PERP");
        assert_eq!(order.time, 1625097600000);
        assert_eq!(order.time_in_force, "GTC");
        assert_eq!(order.order_type, "LIMIT");
        assert!(order.activate_price.is_none());
        assert!(order.price_rate.is_none());
        assert_eq!(order.update_time, 1625097600000);
        assert_eq!(order.working_type, "CONTRACT_PRICE");
        assert!(!order.price_protect);
        assert_eq!(order.price_match.as_ref().unwrap(), "NONE");
        assert_eq!(order.self_trade_prevention_mode.as_ref().unwrap(), "NONE");
    }

    #[test]
    fn test_multiple_open_orders() {
        let json = r#"[
            {
                "avgPrice": "0.00000",
                "clientOrderId": "order1",
                "cumBase": "0",
                "executedQty": "0",
                "orderId": 123456789,
                "origQty": "10",
                "origType": "LIMIT",
                "price": "45000.0",
                "reduceOnly": false,
                "side": "BUY",
                "positionSide": "LONG",
                "status": "NEW",
                "stopPrice": "0",
                "closePosition": false,
                "symbol": "BTCUSD_PERP",
                "time": 1625097600000,
                "timeInForce": "GTC",
                "type": "LIMIT",
                "updateTime": 1625097600000,
                "workingType": "CONTRACT_PRICE",
                "priceProtect": false
            },
            {
                "avgPrice": "3000.50",
                "clientOrderId": "order2",
                "cumBase": "5",
                "executedQty": "5",
                "orderId": 987654321,
                "origQty": "20",
                "origType": "LIMIT",
                "price": "3000.0",
                "reduceOnly": true,
                "side": "SELL",
                "positionSide": "SHORT",
                "status": "PARTIALLY_FILLED",
                "stopPrice": "0",
                "closePosition": false,
                "symbol": "ETHUSD_PERP",
                "time": 1625097700000,
                "timeInForce": "IOC",
                "type": "LIMIT",
                "updateTime": 1625097800000,
                "workingType": "MARK_PRICE",
                "priceProtect": true
            }
        ]"#;

        let response: Vec<OpenOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        assert_eq!(response[0].order_id, 123456789);
        assert_eq!(response[0].symbol, "BTCUSD_PERP");
        assert_eq!(response[0].side, "BUY");

        assert_eq!(response[1].order_id, 987654321);
        assert_eq!(response[1].symbol, "ETHUSD_PERP");
        assert_eq!(response[1].side, "SELL");
        assert_eq!(response[1].status, "PARTIALLY_FILLED");
        assert!(response[1].reduce_only);
    }
}
