//! Integration tests for Bitget Spot Trading endpoints
//!
//! These tests verify that the spot trading endpoint implementations work correctly
//! with the Bitget API structure.

use venues::bitget::private::rest::spot::*;
use venues::bitget::enums::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request_builder() {
        let request = PlaceOrderRequest::new()
            .symbol("BTCUSDT")
            .side(OrderSide::Buy)
            .order_type(OrderType::Limit)
            .force(TimeInForce::GTC)
            .size("0.001")
            .price("50000");

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.force, TimeInForce::GTC);
        assert_eq!(request.size, "0.001");
        assert_eq!(request.price, Some("50000".to_string()));
    }

    #[test]
    fn test_place_order_market_request() {
        let request = PlaceOrderRequest::market_buy("ETHUSDT", "100");
        
        assert_eq!(request.symbol, "ETHUSDT");
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Market);
        assert_eq!(request.force, TimeInForce::IOC);
        assert_eq!(request.size, "100");
        assert!(request.price.is_none());
    }

    #[test]
    fn test_get_order_info_request() {
        let request = GetOrderInfoRequest::new()
            .symbol("BTCUSDT")
            .order_id("123456789");

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("123456789".to_string()));
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_cancel_order_request() {
        let request = CancelOrderRequest::new()
            .symbol("BTCUSDT")
            .order_id("123456789");

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("123456789".to_string()));
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_get_current_orders_request() {
        let request = GetCurrentOrdersRequest::new()
            .symbol("BTCUSDT")
            .limit(50);

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.limit, Some("50".to_string()));
    }

    #[test]
    fn test_get_order_history_request() {
        let request = GetOrderHistoryRequest::new()
            .symbol("BTCUSDT")
            .start_time(1640995200000)
            .end_time(1672531200000)
            .limit(100);

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.start_time, Some("1640995200000".to_string()));
        assert_eq!(request.end_time, Some("1672531200000".to_string()));
        assert_eq!(request.limit, Some("100".to_string()));
    }

    #[test]
    fn test_get_fills_request() {
        let request = GetFillsRequest::new()
            .symbol("BTCUSDT")
            .order_id("123456789")
            .limit(100);

        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.order_id, Some("123456789".to_string()));
        assert_eq!(request.limit, Some("100".to_string()));
    }

    #[test]
    fn test_place_order_response_deserialization() {
        let json = r#"{
            "orderId": "1627293504612",
            "clientOrderId": "abc123"
        }"#;

        let response: PlaceOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "1627293504612");
        assert_eq!(response.client_order_id, "abc123");
    }

    #[test]
    fn test_order_info_response_deserialization() {
        let json = r#"[{
            "userId": "123456789",
            "symbol": "BTCUSDT",
            "orderId": "1627293504612", 
            "clientOrderId": "abc123",
            "price": "50000",
            "size": "0.001",
            "orderType": "limit",
            "side": "buy",
            "status": "filled",
            "priceAvg": "49500",
            "baseVolume": "0.001",
            "quoteVolume": "49.5",
            "enterPointSource": "api",
            "feeDetail": {
                "deduction": "no",
                "feeCoin": "USDT",
                "totalDeductionFee": "",
                "fee": "0.049"
            },
            "orderSource": "normal",
            "cTime": "1627293504612",
            "uTime": "1627293510000"
        }]"#;

        let response: GetOrderInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        
        let order = &response[0];
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, "1627293504612");
        assert_eq!(order.price, "50000");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.status, OrderStatus::Filled);
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        let json = r#"{
            "orderId": "1627293504612",
            "clientOrderId": "abc123"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "1627293504612");
        assert_eq!(response.client_order_id, "abc123");
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
            "fillPrice": "49500",
            "fillQuantity": "0.001",
            "fillTotalAmount": "49.5",
            "cTime": "1627293504612"
        }"#;

        let fill: FillInfo = serde_json::from_str(json).unwrap();
        assert_eq!(fill.symbol, "BTCUSDT");
        assert_eq!(fill.trade_id, "1627293504613");
        assert_eq!(fill.side, OrderSide::Buy);
        assert_eq!(fill.price, "49500");
        assert_eq!(fill.quantity, "0.001");
    }

    #[test]  
    fn test_enum_serialization() {
        // Test OrderSide
        assert_eq!(serde_json::to_string(&OrderSide::Buy).unwrap(), "\"buy\"");
        assert_eq!(serde_json::to_string(&OrderSide::Sell).unwrap(), "\"sell\"");

        // Test OrderType
        assert_eq!(serde_json::to_string(&OrderType::Limit).unwrap(), "\"limit\"");
        assert_eq!(serde_json::to_string(&OrderType::Market).unwrap(), "\"market\"");

        // Test TimeInForce
        assert_eq!(serde_json::to_string(&TimeInForce::GTC).unwrap(), "\"gtc\"");
        assert_eq!(serde_json::to_string(&TimeInForce::IOC).unwrap(), "\"ioc\"");
        assert_eq!(serde_json::to_string(&TimeInForce::FOK).unwrap(), "\"fok\"");

        // Test OrderStatus
        assert_eq!(serde_json::to_string(&OrderStatus::New).unwrap(), "\"new\"");
        assert_eq!(serde_json::to_string(&OrderStatus::PartialFill).unwrap(), "\"partial_fill\"");
        assert_eq!(serde_json::to_string(&OrderStatus::Filled).unwrap(), "\"filled\"");
        assert_eq!(serde_json::to_string(&OrderStatus::Cancelled).unwrap(), "\"cancelled\"");
    }
}
