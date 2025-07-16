#[cfg(test)]
mod tests {
    use venues::binance::spot::{TestNewOrderRequest, OrderSide, OrderType, TimeInForce, AccountCommissionResponse};
    use rust_decimal::Decimal;

    #[test]
    fn test_symbol_not_duplicated_in_request() {
        // Test that symbol doesn't get duplicated when building request
        let request = TestNewOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(Decimal::new(1, 5)), // 0.00001
            quote_order_qty: None,
            price: Some(Decimal::new(20000, 0)), // $20,000
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            compute_commission_rates: None,
            recv_window: None,
        };
        
        // Serialize to JSON to check fields
        let json_str = serde_json::to_string(&request).unwrap();
        
        // Check that symbol appears only once
        let symbol_count = json_str.matches("\"symbol\"").count();
        assert_eq!(symbol_count, 1, "Symbol should appear only once in serialized request");
    }

    #[test]
    fn test_account_commission_null_discount_asset() {
        // Test deserialization of account commission response with null discount_asset
        let json_response = r#"{
            "symbol": "BTCUSDT",
            "standardCommission": {
                "maker": "0.001",
                "taker": "0.001",
                "buyer": "0",
                "seller": "0"
            },
            "taxCommission": {
                "maker": "0",
                "taker": "0",
                "buyer": "0",
                "seller": "0"
            },
            "discount": {
                "enabledForAccount": true,
                "enabledForSymbol": true,
                "discountAsset": null,
                "discount": "0.75"
            }
        }"#;
        
        let response: AccountCommissionResponse = serde_json::from_str(json_response).unwrap();
        
        assert_eq!(response.discount.discount_asset, None);
        assert_eq!(response.symbol, "BTCUSDT");
        assert!(response.discount.enabled_for_account);
        assert!(response.discount.enabled_for_symbol);
    }
}