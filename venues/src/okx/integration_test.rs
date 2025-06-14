#[cfg(test)]
mod integration_test {
    use crate::okx::{
        GetMarkPriceCandlesRequest, 
        GetMarkPriceCandlesResponse,
        GetMarkPriceCandlesHistoryRequest,
        GetMarkPriceCandlesHistoryResponse
    };

    #[test]
    fn test_mark_price_candles_types_are_exported() {
        // Test that we can create instances of the new types
        let request = GetMarkPriceCandlesRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: Some("1H".to_string()),
            limit: Some("100".to_string()),
        };
        
        assert_eq!(request.inst_id, "BTC-USD-SWAP");
        assert_eq!(request.bar, Some("1H".to_string()));
    }

    #[test]
    fn test_mark_price_candles_history_types_are_exported() {
        // Test that we can create instances of the new history types
        let request = GetMarkPriceCandlesHistoryRequest {
            inst_id: "ETH-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: Some("1D".to_string()),
            limit: Some("50".to_string()),
        };
        
        assert_eq!(request.inst_id, "ETH-USD-SWAP");
        assert_eq!(request.bar, Some("1D".to_string()));
    }

    #[test]
    fn test_response_types_exported() {
        use serde_json::json;
        
        // Test mark price candles response
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                ["1597026383085", "3811.2", "3811.2", "3811.2", "3811.2", "1"]
            ]
        });
        
        let response: GetMarkPriceCandlesResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        
        // Test mark price candles history response
        let history_response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                ["1597026383085", "3811.2", "3811.2", "3811.2", "3811.2", "1"]
            ]
        });
        
        let history_response: GetMarkPriceCandlesHistoryResponse = serde_json::from_value(history_response_json).unwrap();
        assert_eq!(history_response.code, "0");
        assert_eq!(history_response.data.len(), 1);
    }
}