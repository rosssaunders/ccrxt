use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::options::{
    OptionsContractType, OptionsOrderResponseType, OptionsOrderSide, OptionsOrderStatus,
    OptionsOrderType, OptionsTimeInForce, RestResult, private_client::RestClient,
};

const NEW_ORDER_ENDPOINT: &str = "/eapi/v1/order";

/// Request parameters for placing a new order
#[derive(Debug, Clone, Serialize)]
pub struct NewOrderRequest {
    /// Option trading pair (e.g., "BTC-200730-9000-C")
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order type (currently only LIMIT is supported)
    #[serde(rename = "type")]
    pub order_type: OptionsOrderType,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Order price (required for LIMIT orders)
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Time in force method (default: GTC)
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<OptionsTimeInForce>,

    /// Reduce only (default: false)
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Post only (default: false)
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Response type: "ACK" or "RESULT" (default: "ACK")
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OptionsOrderResponseType>,

    /// User-defined order ID (cannot be repeated in pending orders)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Market maker protection order flag
    #[serde(rename = "isMmp", skip_serializing_if = "Option::is_none")]
    pub is_mmp: Option<bool>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Order response (ACK type)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAckResponse {
    /// System order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OptionsOrderType,

    /// Order creation time
    #[serde(rename = "createDate")]
    pub create_date: u64,

    /// Reduce only flag
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Post only flag
    #[serde(rename = "postOnly")]
    pub post_only: bool,

    /// Market maker protection flag
    #[serde(rename = "mmp")]
    pub mmp: bool,
}

/// Order response (RESULT type)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderResultResponse {
    /// System order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Number of executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: Decimal,

    /// Fee
    #[serde(rename = "fee")]
    pub fee: Decimal,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OptionsOrderType,

    /// Time in force method
    #[serde(rename = "timeInForce")]
    pub time_in_force: OptionsTimeInForce,

    /// Reduce only flag
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Post only flag
    #[serde(rename = "postOnly")]
    pub post_only: bool,

    /// Order creation time
    #[serde(rename = "createTime")]
    pub create_time: u64,

    /// Order update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Order status
    #[serde(rename = "status")]
    pub status: OptionsOrderStatus,

    /// Average price of completed trades
    #[serde(rename = "avgPrice")]
    pub avg_price: Decimal,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Price scale
    #[serde(rename = "priceScale")]
    pub price_scale: u32,

    /// Quantity scale
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,

    /// Option side (CALL or PUT)
    #[serde(rename = "optionSide")]
    pub option_side: OptionsContractType,

    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    /// Market maker protection flag
    #[serde(rename = "mmp")]
    pub mmp: bool,
}

impl RestClient {
    /// Place a new option order
    ///
    /// Places a new limit order for options contracts. Currently only LIMIT orders are supported.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/trade)
    ///
    /// Method: POST /eapi/v1/order
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn new_order(&self, params: NewOrderRequest) -> RestResult<OrderAckResponse> {
        self.send_post_signed_request(NEW_ORDER_ENDPOINT, params, 1, true)
            .await
    }

    /// Place a new option order with RESULT response
    ///
    /// Places a new limit order for options contracts and returns detailed execution information.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/trade)
    ///
    /// Method: POST /eapi/v1/order
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn new_order_result(
        &self,
        mut params: NewOrderRequest,
    ) -> RestResult<OrderResultResponse> {
        // Force response type to RESULT
        params.new_order_resp_type = Some(OptionsOrderResponseType::Result);

        self.send_post_signed_request(NEW_ORDER_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_new_order_request_serialization_basic() {
        let request = NewOrderRequest {
            symbol: "BTC-200730-9000-C".to_string(),
            side: OptionsOrderSide::Buy,
            order_type: OptionsOrderType::Limit,
            quantity: Decimal::from_str("10.5").unwrap(),
            price: Some(Decimal::from_str("0.01").unwrap()),
            time_in_force: Some(OptionsTimeInForce::Gtc),
            reduce_only: None,
            post_only: None,
            new_order_resp_type: None,
            client_order_id: Some("my_order_123".to_string()),
            is_mmp: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-200730-9000-C"));
        assert!(serialized.contains("side=BUY"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("quantity=10.5"));
        assert!(serialized.contains("price=0.01"));
        assert!(serialized.contains("timeInForce=GTC"));
        assert!(serialized.contains("clientOrderId=my_order_123"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_new_order_request_serialization_sell() {
        let request = NewOrderRequest {
            symbol: "ETH-200730-2000-P".to_string(),
            side: OptionsOrderSide::Sell,
            order_type: OptionsOrderType::Limit,
            quantity: Decimal::from_str("5.0").unwrap(),
            price: Some(Decimal::from_str("0.02").unwrap()),
            time_in_force: Some(OptionsTimeInForce::Ioc),
            reduce_only: Some(true),
            post_only: Some(false),
            new_order_resp_type: Some(OptionsOrderResponseType::Result),
            client_order_id: None,
            is_mmp: Some(true),
            recv_window: Some(5000),
            timestamp: 1625097700000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETH-200730-2000-P"));
        assert!(serialized.contains("side=SELL"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("quantity=5"));
        assert!(serialized.contains("price=0.02"));
        assert!(serialized.contains("timeInForce=IOC"));
        assert!(serialized.contains("reduceOnly=true"));
        assert!(serialized.contains("postOnly=false"));
        assert!(serialized.contains("newOrderRespType=RESULT"));
        assert!(serialized.contains("isMmp=true"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097700000"));
    }

    #[test]
    fn test_new_order_request_serialization_fok() {
        let request = NewOrderRequest {
            symbol: "BTC-200730-10000-C".to_string(),
            side: OptionsOrderSide::Buy,
            order_type: OptionsOrderType::Limit,
            quantity: Decimal::from_str("1.0").unwrap(),
            price: Some(Decimal::from_str("0.005").unwrap()),
            time_in_force: Some(OptionsTimeInForce::Fok),
            reduce_only: Some(false),
            post_only: Some(true),
            new_order_resp_type: Some(OptionsOrderResponseType::Ack),
            client_order_id: Some("fok_order_456".to_string()),
            is_mmp: Some(false),
            recv_window: Some(10000),
            timestamp: 1625097800000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-200730-10000-C"));
        assert!(serialized.contains("side=BUY"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("quantity=1"));
        assert!(serialized.contains("price=0.005"));
        assert!(serialized.contains("timeInForce=FOK"));
        assert!(serialized.contains("reduceOnly=false"));
        assert!(serialized.contains("postOnly=true"));
        assert!(serialized.contains("newOrderRespType=ACK"));
        assert!(serialized.contains("clientOrderId=fok_order_456"));
        assert!(serialized.contains("isMmp=false"));
        assert!(serialized.contains("recvWindow=10000"));
        assert!(serialized.contains("timestamp=1625097800000"));
    }

    #[test]
    fn test_new_order_request_serialization_minimal() {
        let request = NewOrderRequest {
            symbol: "BTC-200730-9500-P".to_string(),
            side: OptionsOrderSide::Sell,
            order_type: OptionsOrderType::Limit,
            quantity: Decimal::from_str("2.5").unwrap(),
            price: Some(Decimal::from_str("0.03").unwrap()),
            time_in_force: None,
            reduce_only: None,
            post_only: None,
            new_order_resp_type: None,
            client_order_id: None,
            is_mmp: None,
            recv_window: None,
            timestamp: 1625097900000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-200730-9500-P"));
        assert!(serialized.contains("side=SELL"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("quantity=2.5"));
        assert!(serialized.contains("price=0.03"));
        assert!(serialized.contains("timestamp=1625097900000"));

        // Check that optional fields are not serialized
        assert!(!serialized.contains("timeInForce"));
        assert!(!serialized.contains("reduceOnly"));
        assert!(!serialized.contains("postOnly"));
        assert!(!serialized.contains("newOrderRespType"));
        assert!(!serialized.contains("clientOrderId"));
        assert!(!serialized.contains("isMmp"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_new_order_request_serialization_without_price() {
        let request = NewOrderRequest {
            symbol: "ETH-200730-2500-C".to_string(),
            side: OptionsOrderSide::Buy,
            order_type: OptionsOrderType::Limit,
            quantity: Decimal::from_str("1.0").unwrap(),
            price: None,
            time_in_force: Some(OptionsTimeInForce::Gtc),
            reduce_only: None,
            post_only: None,
            new_order_resp_type: None,
            client_order_id: None,
            is_mmp: None,
            recv_window: None,
            timestamp: 1625098000000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETH-200730-2500-C"));
        assert!(serialized.contains("side=BUY"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("quantity=1"));
        assert!(serialized.contains("timeInForce=GTC"));
        assert!(serialized.contains("timestamp=1625098000000"));

        // Check that price is not serialized when None
        assert!(!serialized.contains("price"));
    }

    #[test]
    fn test_order_ack_response_deserialization() {
        let json = r#"{
            "orderId": 123456789,
            "symbol": "BTC-200730-9000-C",
            "price": "0.01",
            "quantity": "10.5",
            "side": "BUY",
            "type": "LIMIT",
            "createDate": 1625097600000,
            "reduceOnly": false,
            "postOnly": true,
            "mmp": false
        }"#;

        let response: OrderAckResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.symbol, "BTC-200730-9000-C");
        assert_eq!(response.price, Decimal::from_str("0.01").unwrap());
        assert_eq!(response.quantity, Decimal::from_str("10.5").unwrap());
        assert_eq!(response.side, OptionsOrderSide::Buy);
        assert_eq!(response.order_type, OptionsOrderType::Limit);
        assert_eq!(response.create_date, 1625097600000);
        assert!(!response.reduce_only);
        assert!(response.post_only);
        assert!(!response.mmp);
    }

    #[test]
    fn test_order_ack_response_deserialization_sell() {
        let json = r#"{
            "orderId": 987654321,
            "symbol": "ETH-200730-2000-P",
            "price": "0.02",
            "quantity": "5.0",
            "side": "SELL",
            "type": "LIMIT",
            "createDate": 1625097700000,
            "reduceOnly": true,
            "postOnly": false,
            "mmp": true
        }"#;

        let response: OrderAckResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 987654321);
        assert_eq!(response.symbol, "ETH-200730-2000-P");
        assert_eq!(response.price, Decimal::from_str("0.02").unwrap());
        assert_eq!(response.quantity, Decimal::from_str("5.0").unwrap());
        assert_eq!(response.side, OptionsOrderSide::Sell);
        assert_eq!(response.order_type, OptionsOrderType::Limit);
        assert_eq!(response.create_date, 1625097700000);
        assert!(response.reduce_only);
        assert!(!response.post_only);
        assert!(response.mmp);
    }

    #[test]
    fn test_order_result_response_deserialization() {
        let json = r#"{
            "orderId": 123456789,
            "symbol": "BTC-200730-9000-C",
            "price": "0.01",
            "quantity": "10.5",
            "executedQty": "10.5",
            "fee": "0.0001",
            "side": "BUY",
            "type": "LIMIT",
            "timeInForce": "GTC",
            "reduceOnly": false,
            "postOnly": true,
            "createTime": 1625097600000,
            "updateTime": 1625097700000,
            "status": "FILLED",
            "avgPrice": "0.01",
            "clientOrderId": "my_order_123",
            "priceScale": 8,
            "quantityScale": 2,
            "optionSide": "CALL",
            "quoteAsset": "USDT",
            "mmp": false
        }"#;

        let response: OrderResultResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.symbol, "BTC-200730-9000-C");
        assert_eq!(response.price, Decimal::from_str("0.01").unwrap());
        assert_eq!(response.quantity, Decimal::from_str("10.5").unwrap());
        assert_eq!(response.executed_qty, Decimal::from_str("10.5").unwrap());
        assert_eq!(response.fee, Decimal::from_str("0.0001").unwrap());
        assert_eq!(response.side, OptionsOrderSide::Buy);
        assert_eq!(response.order_type, OptionsOrderType::Limit);
        assert_eq!(response.time_in_force, OptionsTimeInForce::Gtc);
        assert!(!response.reduce_only);
        assert!(response.post_only);
        assert_eq!(response.create_time, 1625097600000);
        assert_eq!(response.update_time, 1625097700000);
        assert_eq!(response.status, OptionsOrderStatus::Filled);
        assert_eq!(response.avg_price, Decimal::from_str("0.01").unwrap());
        assert_eq!(response.client_order_id, "my_order_123");
        assert_eq!(response.price_scale, 8);
        assert_eq!(response.quantity_scale, 2);
        assert_eq!(response.option_side, OptionsContractType::Call);
        assert_eq!(response.quote_asset, "USDT");
        assert!(!response.mmp);
    }

    #[test]
    fn test_order_result_response_deserialization_sell_put() {
        let json = r#"{
            "orderId": 987654321,
            "symbol": "ETH-200730-2000-P",
            "price": "0.02",
            "quantity": "5.0",
            "executedQty": "0.0",
            "fee": "0.0",
            "side": "SELL",
            "type": "LIMIT",
            "timeInForce": "IOC",
            "reduceOnly": true,
            "postOnly": false,
            "createTime": 1625097700000,
            "updateTime": 1625097700000,
            "status": "ACCEPTED",
            "avgPrice": "0.0",
            "clientOrderId": "sell_order_456",
            "priceScale": 6,
            "quantityScale": 3,
            "optionSide": "PUT",
            "quoteAsset": "USDT",
            "mmp": true
        }"#;

        let response: OrderResultResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 987654321);
        assert_eq!(response.symbol, "ETH-200730-2000-P");
        assert_eq!(response.price, Decimal::from_str("0.02").unwrap());
        assert_eq!(response.quantity, Decimal::from_str("5.0").unwrap());
        assert_eq!(response.executed_qty, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.fee, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.side, OptionsOrderSide::Sell);
        assert_eq!(response.order_type, OptionsOrderType::Limit);
        assert_eq!(response.time_in_force, OptionsTimeInForce::Ioc);
        assert!(response.reduce_only);
        assert!(!response.post_only);
        assert_eq!(response.create_time, 1625097700000);
        assert_eq!(response.update_time, 1625097700000);
        assert_eq!(response.status, OptionsOrderStatus::Accepted);
        assert_eq!(response.avg_price, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.client_order_id, "sell_order_456");
        assert_eq!(response.price_scale, 6);
        assert_eq!(response.quantity_scale, 3);
        assert_eq!(response.option_side, OptionsContractType::Put);
        assert_eq!(response.quote_asset, "USDT");
        assert!(response.mmp);
    }

    #[test]
    fn test_order_result_response_deserialization_fok() {
        let json = r#"{
            "orderId": 456789123,
            "symbol": "BTC-200730-10000-C",
            "price": "0.005",
            "quantity": "1.0",
            "executedQty": "0.5",
            "fee": "0.000025",
            "side": "BUY",
            "type": "LIMIT",
            "timeInForce": "FOK",
            "reduceOnly": false,
            "postOnly": true,
            "createTime": 1625097800000,
            "updateTime": 1625097850000,
            "status": "PARTIALLY_FILLED",
            "avgPrice": "0.005",
            "clientOrderId": "fok_order_789",
            "priceScale": 8,
            "quantityScale": 1,
            "optionSide": "CALL",
            "quoteAsset": "USDT",
            "mmp": false
        }"#;

        let response: OrderResultResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 456789123);
        assert_eq!(response.symbol, "BTC-200730-10000-C");
        assert_eq!(response.price, Decimal::from_str("0.005").unwrap());
        assert_eq!(response.quantity, Decimal::from_str("1.0").unwrap());
        assert_eq!(response.executed_qty, Decimal::from_str("0.5").unwrap());
        assert_eq!(response.fee, Decimal::from_str("0.000025").unwrap());
        assert_eq!(response.side, OptionsOrderSide::Buy);
        assert_eq!(response.order_type, OptionsOrderType::Limit);
        assert_eq!(response.time_in_force, OptionsTimeInForce::Fok);
        assert!(!response.reduce_only);
        assert!(response.post_only);
        assert_eq!(response.create_time, 1625097800000);
        assert_eq!(response.update_time, 1625097850000);
        assert_eq!(response.status, OptionsOrderStatus::PartiallyFilled);
        assert_eq!(response.avg_price, Decimal::from_str("0.005").unwrap());
        assert_eq!(response.client_order_id, "fok_order_789");
        assert_eq!(response.price_scale, 8);
        assert_eq!(response.quantity_scale, 1);
        assert_eq!(response.option_side, OptionsContractType::Call);
        assert_eq!(response.quote_asset, "USDT");
        assert!(!response.mmp);
    }

    #[test]
    fn test_order_result_response_deserialization_cancelled() {
        let json = r#"{
            "orderId": 111222333,
            "symbol": "ETH-200730-2500-C",
            "price": "0.015",
            "quantity": "2.5",
            "executedQty": "0.0",
            "fee": "0.0",
            "side": "BUY",
            "type": "LIMIT",
            "timeInForce": "GTC",
            "reduceOnly": false,
            "postOnly": false,
            "createTime": 1625098000000,
            "updateTime": 1625098100000,
            "status": "CANCELLED",
            "avgPrice": "0.0",
            "clientOrderId": "cancelled_order_999",
            "priceScale": 6,
            "quantityScale": 1,
            "optionSide": "CALL",
            "quoteAsset": "USDT",
            "mmp": false
        }"#;

        let response: OrderResultResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 111222333);
        assert_eq!(response.symbol, "ETH-200730-2500-C");
        assert_eq!(response.price, Decimal::from_str("0.015").unwrap());
        assert_eq!(response.quantity, Decimal::from_str("2.5").unwrap());
        assert_eq!(response.executed_qty, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.fee, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.side, OptionsOrderSide::Buy);
        assert_eq!(response.order_type, OptionsOrderType::Limit);
        assert_eq!(response.time_in_force, OptionsTimeInForce::Gtc);
        assert!(!response.reduce_only);
        assert!(!response.post_only);
        assert_eq!(response.create_time, 1625098000000);
        assert_eq!(response.update_time, 1625098100000);
        assert_eq!(response.status, OptionsOrderStatus::Cancelled);
        assert_eq!(response.avg_price, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.client_order_id, "cancelled_order_999");
        assert_eq!(response.price_scale, 6);
        assert_eq!(response.quantity_scale, 1);
        assert_eq!(response.option_side, OptionsContractType::Call);
        assert_eq!(response.quote_asset, "USDT");
        assert!(!response.mmp);
    }

    #[test]
    fn test_order_result_response_deserialization_rejected() {
        let json = r#"{
            "orderId": 555666777,
            "symbol": "BTC-200730-8000-P",
            "price": "0.03",
            "quantity": "1.5",
            "executedQty": "0.0",
            "fee": "0.0",
            "side": "SELL",
            "type": "LIMIT",
            "timeInForce": "IOC",
            "reduceOnly": true,
            "postOnly": false,
            "createTime": 1625098200000,
            "updateTime": 1625098200000,
            "status": "REJECTED",
            "avgPrice": "0.0",
            "clientOrderId": "rejected_order_111",
            "priceScale": 8,
            "quantityScale": 1,
            "optionSide": "PUT",
            "quoteAsset": "USDT",
            "mmp": true
        }"#;

        let response: OrderResultResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 555666777);
        assert_eq!(response.symbol, "BTC-200730-8000-P");
        assert_eq!(response.price, Decimal::from_str("0.03").unwrap());
        assert_eq!(response.quantity, Decimal::from_str("1.5").unwrap());
        assert_eq!(response.executed_qty, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.fee, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.side, OptionsOrderSide::Sell);
        assert_eq!(response.order_type, OptionsOrderType::Limit);
        assert_eq!(response.time_in_force, OptionsTimeInForce::Ioc);
        assert!(response.reduce_only);
        assert!(!response.post_only);
        assert_eq!(response.create_time, 1625098200000);
        assert_eq!(response.update_time, 1625098200000);
        assert_eq!(response.status, OptionsOrderStatus::Rejected);
        assert_eq!(response.avg_price, Decimal::from_str("0.0").unwrap());
        assert_eq!(response.client_order_id, "rejected_order_111");
        assert_eq!(response.price_scale, 8);
        assert_eq!(response.quantity_scale, 1);
        assert_eq!(response.option_side, OptionsContractType::Put);
        assert_eq!(response.quote_asset, "USDT");
        assert!(response.mmp);
    }

    #[test]
    fn test_new_order_request_serialization_all_options() {
        let request = NewOrderRequest {
            symbol: "BTC-200730-12000-C".to_string(),
            side: OptionsOrderSide::Buy,
            order_type: OptionsOrderType::Limit,
            quantity: Decimal::from_str("7.25").unwrap(),
            price: Some(Decimal::from_str("0.008").unwrap()),
            time_in_force: Some(OptionsTimeInForce::Gtc),
            reduce_only: Some(false),
            post_only: Some(true),
            new_order_resp_type: Some(OptionsOrderResponseType::Result),
            client_order_id: Some("comprehensive_order_123".to_string()),
            is_mmp: Some(false),
            recv_window: Some(30000),
            timestamp: 1625098300000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-200730-12000-C"));
        assert!(serialized.contains("side=BUY"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("quantity=7.25"));
        assert!(serialized.contains("price=0.008"));
        assert!(serialized.contains("timeInForce=GTC"));
        assert!(serialized.contains("reduceOnly=false"));
        assert!(serialized.contains("postOnly=true"));
        assert!(serialized.contains("newOrderRespType=RESULT"));
        assert!(serialized.contains("clientOrderId=comprehensive_order_123"));
        assert!(serialized.contains("isMmp=false"));
        assert!(serialized.contains("recvWindow=30000"));
        assert!(serialized.contains("timestamp=1625098300000"));
    }

    #[test]
    fn test_decimal_precision_serialization() {
        let request = NewOrderRequest {
            symbol: "BTC-200730-9000-C".to_string(),
            side: OptionsOrderSide::Buy,
            order_type: OptionsOrderType::Limit,
            quantity: Decimal::from_str("10.123456789").unwrap(),
            price: Some(Decimal::from_str("0.012345678901234567").unwrap()),
            time_in_force: None,
            reduce_only: None,
            post_only: None,
            new_order_resp_type: None,
            client_order_id: None,
            is_mmp: None,
            recv_window: None,
            timestamp: 1625098400000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("quantity=10.123456789"));
        assert!(serialized.contains("price=0.012345678901234567"));
    }

    #[test]
    fn test_zero_values_serialization() {
        let request = NewOrderRequest {
            symbol: "ETH-200730-2000-P".to_string(),
            side: OptionsOrderSide::Sell,
            order_type: OptionsOrderType::Limit,
            quantity: Decimal::from_str("0.0").unwrap(),
            price: Some(Decimal::from_str("0.0").unwrap()),
            time_in_force: Some(OptionsTimeInForce::Ioc),
            reduce_only: Some(false),
            post_only: Some(false),
            new_order_resp_type: Some(OptionsOrderResponseType::Ack),
            client_order_id: Some("zero_order".to_string()),
            is_mmp: Some(false),
            recv_window: Some(0),
            timestamp: 0,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("quantity=0"));
        assert!(serialized.contains("price=0"));
        assert!(serialized.contains("recvWindow=0"));
        assert!(serialized.contains("timestamp=0"));
    }
}
