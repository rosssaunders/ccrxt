use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    OrderSide, OrderType, PositionSide, PriceMatch, RestResult, SelfTradePreventionMode,
    TimeInForce, WorkingType, private::rest::client::RestClient,
};

const CANCEL_ORDER_ENDPOINT: &str = "/dapi/v1/order";

/// Request parameters for canceling an active order (DELETE /dapi/v1/order).
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_200925").
    pub symbol: String,

    /// Order ID to cancel. Either `order_id` or `orig_client_order_id` must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either `order_id` or `orig_client_order_id` must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// The value cannot be greater than 60000.
    /// Range: 0 to 60000 milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch. Mandatory.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for canceling an order (DELETE /dapi/v1/order).
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    #[serde(rename = "cumQty")]
    pub cum_qty: String,

    #[serde(rename = "cumBase")]
    pub cum_base: String,

    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    #[serde(rename = "orderId")]
    pub order_id: u64,

    #[serde(rename = "origQty")]
    pub orig_qty: String,

    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    pub price: String,

    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    pub side: OrderSide,

    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    pub status: String,

    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,

    #[serde(rename = "closePosition")]
    pub close_position: Option<bool>,

    pub symbol: String,

    pub pair: String,

    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<TimeInForce>,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    #[serde(rename = "activatePrice")]
    pub activate_price: Option<String>,

    #[serde(rename = "priceRate")]
    pub price_rate: Option<String>,

    #[serde(rename = "updateTime")]
    pub update_time: u64,

    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    #[serde(rename = "priceMatch")]
    pub price_match: Option<PriceMatch>,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

impl RestClient {
    /// Cancels an active order on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-Order
    ///
    /// DELETE /dapi/v1/order
    /// Weight: 1 (order rate limit)
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`CancelOrderRequest`])
    ///
    /// # Returns
    /// A [`CancelOrderResponse`] object with order details.
    pub async fn delete_order(
        &self,
        params: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_delete_signed_request(CANCEL_ORDER_ENDPOINT, params, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_serialization_with_order_id() {
        let request = CancelOrderRequest {
            symbol: "BTCUSD_PERP".to_string(),
            order_id: Some(123456789),
            orig_client_order_id: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("orderId=123456789"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("origClientOrderId"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_cancel_order_request_serialization_with_client_order_id() {
        let request = CancelOrderRequest {
            symbol: "ETHUSD_PERP".to_string(),
            order_id: None,
            orig_client_order_id: Some("myOrder123".to_string()),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("origClientOrderId=myOrder123"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("orderId"));
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        let json = r#"{
            "avgPrice": "0",
            "clientOrderId": "myOrder123",
            "cumQty": "0",
            "cumBase": "0",
            "executedQty": "0",
            "orderId": 123456789,
            "origQty": "10.00000000",
            "origType": "LIMIT",
            "price": "50000.00",
            "reduceOnly": false,
            "side": "BUY",
            "positionSide": "LONG",
            "status": "CANCELED",
            "stopPrice": null,
            "closePosition": false,
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "activatePrice": null,
            "priceRate": null,
            "updateTime": 1625097600000,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "priceMatch": null,
            "selfTradePreventionMode": null
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_price, "0");
        assert_eq!(response.client_order_id, "myOrder123");
        assert_eq!(response.cum_qty, "0");
        assert_eq!(response.cum_base, "0");
        assert_eq!(response.executed_qty, "0");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.orig_qty, "10.00000000");
        assert_eq!(response.orig_type, OrderType::Limit);
        assert_eq!(response.price, "50000.00");
        assert!(!response.reduce_only);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.position_side, PositionSide::Long);
        assert_eq!(response.status, "CANCELED");
        assert!(response.stop_price.is_none());
        assert_eq!(response.close_position, Some(false));
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.pair, "BTCUSD");
        assert_eq!(response.time_in_force, Some(TimeInForce::GTC));
        assert_eq!(response.order_type, OrderType::Limit);
        assert_eq!(response.update_time, 1625097600000);
        assert_eq!(response.working_type, WorkingType::ContractPrice);
        assert!(!response.price_protect);
    }

    #[test]
    fn test_cancel_order_response_deserialization_with_stop_order() {
        let json = r#"{
            "avgPrice": "0",
            "clientOrderId": "stopOrder456",
            "cumQty": "0",
            "cumBase": "0",
            "executedQty": "0",
            "orderId": 987654321,
            "origQty": "5.00000000",
            "origType": "STOP",
            "price": "48000.00",
            "reduceOnly": true,
            "side": "SELL",
            "positionSide": "BOTH",
            "status": "CANCELED",
            "stopPrice": "49000.00",
            "closePosition": null,
            "symbol": "BTCUSD_240329",
            "pair": "BTCUSD",
            "timeInForce": "GTC",
            "type": "STOP",
            "activatePrice": null,
            "priceRate": null,
            "updateTime": 1625097700000,
            "workingType": "MARK_PRICE",
            "priceProtect": true,
            "priceMatch": "OPPONENT",
            "selfTradePreventionMode": "EXPIRE_MAKER"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.client_order_id, "stopOrder456");
        assert_eq!(response.order_id, 987654321);
        assert_eq!(response.orig_qty, "5.00000000");
        assert_eq!(response.orig_type, OrderType::Stop);
        assert!(response.reduce_only);
        assert_eq!(response.side, OrderSide::Sell);
        assert_eq!(response.position_side, PositionSide::Both);
        assert_eq!(response.stop_price, Some("49000.00".to_string()));
        assert_eq!(response.working_type, WorkingType::MarkPrice);
        assert!(response.price_protect);
        assert_eq!(response.price_match, Some(PriceMatch::Opponent));
        assert_eq!(
            response.self_trade_prevention_mode,
            Some(SelfTradePreventionMode::ExpireMaker)
        );
    }
}
