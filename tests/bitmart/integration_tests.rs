//! BitMart integration tests
//!
//! These tests verify the overall integration and functionality of the BitMart module.

#[cfg(test)]
mod tests {
    use venues::bitmart::{
        PublicRestClient, PrivateRestClient, 
        GetCurrencyListRequest, SubmitOrderRequest,
        OrderSide, OrderType, StpMode
    };

    #[test]
    fn test_public_client_creation() {
        let client = PublicRestClient::new();
        // Just verify that we can create a client without panicking
        assert!(true, "Public client creation successful");
    }

    #[test]
    fn test_private_client_creation() {
        let client = PrivateRestClient::new(
            "test_api_key",
            "test_api_secret", 
            "test_memo"
        );
        // Just verify that we can create a client without panicking
        assert!(true, "Private client creation successful");
    }

    #[test]
    fn test_request_structures() {
        // Test that we can create request structures
        let currency_list_req = GetCurrencyListRequest::default();
        
        let order_req = SubmitOrderRequest {
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            size: "0.001".to_string(),
            price: None,
            client_order_id: None,
            notional: None,
            stp_mode: Some(StpMode::CN), // Cancel newest
        };

        assert_eq!(order_req.symbol, "BTC_USDT");
        assert_eq!(order_req.side, OrderSide::Buy);
        assert_eq!(order_req.order_type, OrderType::Market);
        assert_eq!(order_req.size, "0.001");
        assert_eq!(order_req.stp_mode, Some(StpMode::CN));
    }

    #[test]
    fn test_enums() {
        // Test OrderSide enum
        assert_eq!(format!("{:?}", OrderSide::Buy), "Buy");
        assert_eq!(format!("{:?}", OrderSide::Sell), "Sell");

        // Test OrderType enum  
        assert_eq!(format!("{:?}", OrderType::Market), "Market");
        assert_eq!(format!("{:?}", OrderType::Limit), "Limit");

        // Test StpMode enum
        assert_eq!(format!("{:?}", StpMode::CN), "CN");
        assert_eq!(format!("{:?}", StpMode::CO), "CO");
        assert_eq!(format!("{:?}", StpMode::CB), "CB");
    }

    #[test]
    fn test_module_exports() {
        // Verify that all major types can be imported and used
        use venues::bitmart::{
            // Public API types
            PublicRestClient, Currency, GetCurrencyListRequest, GetCurrencyListResponse,
            TickerData, GetTickerRequest, GetTickerResponse,
            DepthData, GetDepthRequest, GetDepthResponse,
            
            // Private API types
            PrivateRestClient, SubmitOrderRequest, SubmitOrderResponse,
            SubmitBatchOrderRequest, SubmitMarginOrderRequest,
            
            // Enums
            OrderSide, OrderType, StpMode,
            
            // WebSocket types
            WebSocketClient as PublicWebSocketClient,
            
            // Error types
            RestResult, Errors, ApiError
        };

        // If we can import all these types, the module structure is correct
        assert!(true, "All module exports are accessible");
    }

    #[test]
    fn test_constants() {
        use venues::bitmart::public::websocket::BITMART_WS_PUBLIC_URL;
        use venues::bitmart::private::websocket::BITMART_WS_PRIVATE_URL;

        assert!(BITMART_WS_PUBLIC_URL.starts_with("wss://"));
        assert!(BITMART_WS_PRIVATE_URL.starts_with("wss://"));
        assert!(BITMART_WS_PUBLIC_URL.contains("bitmart.com"));
        assert!(BITMART_WS_PRIVATE_URL.contains("bitmart.com"));
    }

    #[test]
    fn test_serialization() {
        use serde_json;
        
        let order_req = SubmitOrderRequest {
            symbol: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            size: "0.001".to_string(),
            price: Some("50000.00".to_string()),
            client_order_id: Some("test_order_123".to_string()),
            notional: None,
            stp_mode: Some(StpMode::CN),
        };

        // Test that we can serialize the request
        let json = serde_json::to_string(&order_req);
        assert!(json.is_ok(), "Order request should serialize to JSON");

        let json_str = json.unwrap();
        assert!(json_str.contains("BTC_USDT"));
        assert!(json_str.contains("\"side\":\"buy\"") || json_str.contains("\"side\":\"Buy\""));
    }
}
