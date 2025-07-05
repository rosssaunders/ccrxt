//! Integration tests for Bitget Spot Trading endpoints
//!
//! These tests verify that the spot trading endpoint implementations work correctly
//! with the Bitget API structure.

use venues::bitget::private::rest::spot::*;
use venues::bitget::enums::*;

#[cfg(test)]
mod tests {
    use super::*;

//! Integration tests for Bitget Spot Trading endpoints
//!
//! These tests verify that the spot trading endpoint implementations work correctly
//! with the Bitget API structure.

use venues::bitget::private::rest::spot::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request_structure() {
        let request = PlaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::GTC,
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.force, Force::GTC);
        assert_eq!(request.size, "0.001");
        assert_eq!(request.price, Some("50000".to_string()));
    }

    #[test]
    fn test_place_order_market_request() {
        let request = PlaceOrderRequest {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            force: Force::IOC,
            price: None,
            size: "100".to_string(),
            client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };
        
        assert_eq!(request.symbol, "ETHUSDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Market);
        assert_eq!(request.force, Force::IOC);
        assert_eq!(request.size, "100");
        assert!(request.price.is_none());
    }

    #[test]
    fn test_get_order_info_request() {
        let request = GetOrderInfoRequest::by_order_id("123456789");

        assert_eq!(request.order_id, Some("123456789".to_string()));
        assert!(request.client_order_id.is_none());
    }
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_cancel_order_request() {
        let request = CancelOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("123456789".to_string()),
            client_order_id: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("123456789".to_string()));
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_get_current_orders_request() {
        let request = GetCurrentOrdersRequest {
            symbol: Some("BTCUSDT".to_string()),
            limit: Some(50),
            ..Default::default()
        };

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.limit, Some(50));
    }

    #[test]
    fn test_get_order_history_request() {
        let request = GetOrderHistoryRequest {
            symbol: Some("BTCUSDT".to_string()),
            start_time: Some(1640995200000),
            end_time: Some(1672531200000),
            limit: Some(100),
            id_less_than: None,
            order_id: None,
            tpsl_type: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.start_time, Some(1640995200000));
        assert_eq!(request.end_time, Some(1672531200000));
        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_get_fills_request() {
        let request = GetFillsRequest {
            symbol: Some("BTCUSDT".to_string()),
            order_id: Some("123456789".to_string()),
            limit: Some(100),
            ..Default::default()
        };

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.order_id, Some("123456789".to_string()));
        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_place_order_response_deserialization() {
        let json = r#"{
            "orderId": "1627293504612",
            "clientOid": "abc123"
        }"#;

        let response: PlaceOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "1627293504612");
        assert_eq!(response.client_order_id, Some("abc123".to_string()));
    }

    #[test]
    fn test_order_info_response_deserialization() {
        let json = r#"{
            "userId": "123456789",
            "symbol": "BTCUSDT",
            "orderId": "1627293504612", 
            "clientOid": "abc123",
            "price": "50000",
            "size": "0.001",
            "orderType": "limit",
            "side": "buy",
            "status": "filled",
            "priceAvg": "49500",
            "baseVolume": "0.001",
            "quoteVolume": "49.5",
            "enterPointSource": "API",
            "feeDetail": "{}",
            "orderSource": "normal",
            "cTime": "1627293504612",
            "uTime": "1627293510000"
        }"#;

        let response: GetOrderInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, "1627293504612");
        assert_eq!(response.price, "50000");
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.status, OrderStatus::Filled);
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        let json = r#"{
            "orderId": "1627293504612",
            "clientOid": "abc123"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "1627293504612");
        assert_eq!(response.client_order_id, Some("abc123".to_string()));
    }

    #[test]
    fn test_fill_info_deserialization() {
        let json = r#"{
            "userId": "123456789",
            "symbol": "BTCUSDT",
            "orderId": "1627293504612",
            "tradeId": "1627293504613",
            "orderType": "limit",
            "side": "buy",
            "priceAvg": "49500",
            "size": "0.001",
            "amount": "49.5",
            "feeDetail": {
                "deduction": "no",
                "feeCoin": "USDT",
                "totalDeductionFee": "",
                "fee": "0.049"
            },
            "tradeScope": "taker",
            "cTime": "1627293504612",
            "uTime": "1627293510000"
        }"#;

        let fill: FillInfo = serde_json::from_str(json).unwrap();
        assert_eq!(fill.symbol, "BTCUSDT");
        assert_eq!(fill.trade_id, "1627293504613");
        assert_eq!(fill.side, OrderSide::Buy);
        assert_eq!(fill.price_avg, "49500");
        assert_eq!(fill.size, "0.001");
    }

    #[test]  
    fn test_enum_serialization() {
        // Test OrderSide
        assert_eq!(serde_json::to_string(&OrderSide::Buy).unwrap(), "\"buy\"");
        assert_eq!(serde_json::to_string(&OrderSide::Sell).unwrap(), "\"sell\"");

        // Test OrderType
        assert_eq!(serde_json::to_string(&OrderType::Limit).unwrap(), "\"limit\"");
        assert_eq!(serde_json::to_string(&OrderType::Market).unwrap(), "\"market\"");

        // Test Force (replaces TimeInForce)
        assert_eq!(serde_json::to_string(&Force::GTC).unwrap(), "\"gtc\"");
        assert_eq!(serde_json::to_string(&Force::IOC).unwrap(), "\"ioc\"");
        assert_eq!(serde_json::to_string(&Force::FOK).unwrap(), "\"fok\"");

        // Test OrderStatus
        assert_eq!(serde_json::to_string(&OrderStatus::New).unwrap(), "\"new\"");
        assert_eq!(serde_json::to_string(&OrderStatus::PartiallyFilled).unwrap(), "\"partial_fill\"");
        assert_eq!(serde_json::to_string(&OrderStatus::Filled).unwrap(), "\"full_fill\"");
        assert_eq!(serde_json::to_string(&OrderStatus::Cancelled).unwrap(), "\"cancelled\"");
    }
}
