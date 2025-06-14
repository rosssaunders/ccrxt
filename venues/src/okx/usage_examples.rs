//! Integration test to demonstrate usage of the new OKX mark price candles endpoints
//! 
//! This test shows how users can use the new endpoints to fetch mark price candlestick data.

#[cfg(test)]
mod usage_examples {
    use crate::okx::{
        GetMarkPriceCandlesRequest,
        GetMarkPriceCandlesHistoryRequest,
        PublicRestClient,
        RateLimiter
    };
    use reqwest::Client;

    #[test]
    fn test_mark_price_candles_request_usage() {
        // Example of how users would create a mark price candles request
        let request = GetMarkPriceCandlesRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: Some("1H".to_string()),
            limit: Some("100".to_string()),
        };

        // Verify the request can be serialized for the API
        let json = serde_json::to_value(&request).unwrap();
        assert!(json.is_object());
        assert_eq!(json["instId"], "BTC-USD-SWAP");
        assert_eq!(json["bar"], "1H");
        assert_eq!(json["limit"], "100");
    }

    #[test]
    fn test_mark_price_candles_history_request_usage() {
        // Example of how users would create a mark price candles history request
        let request = GetMarkPriceCandlesHistoryRequest {
            inst_id: "ETH-USD-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: None,
            bar: Some("1D".to_string()),
            limit: Some("50".to_string()),
        };

        // Verify the request can be serialized for the API
        let json = serde_json::to_value(&request).unwrap();
        assert!(json.is_object());
        assert_eq!(json["instId"], "ETH-USD-SWAP");
        assert_eq!(json["after"], "1597026383085");
        assert_eq!(json["bar"], "1D");
        assert_eq!(json["limit"], "50");
    }

    #[test]
    fn test_client_has_new_methods() {
        // Verify that the RestClient has the new methods available
        let client = PublicRestClient::new(
            "https://www.okx.com",
            Client::new(),
            RateLimiter::new(),
        );

        // We can't make actual API calls in tests, but we can verify the methods exist
        // and have the correct signatures by attempting to call them with test data
        let mark_price_request = GetMarkPriceCandlesRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: Some("1m".to_string()),
            limit: None,
        };

        let history_request = GetMarkPriceCandlesHistoryRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            bar: Some("1m".to_string()),
            limit: None,
        };

        // These would normally be async calls, but we're just checking method signatures
        let _future1 = client.get_mark_price_candles(mark_price_request);
        let _future2 = client.get_mark_price_candles_history(history_request);
        
        // If we reach this point, the methods exist and have correct signatures
        assert!(true);
    }

    #[test]
    fn test_candlestick_data_format() {
        // Test that we can parse the expected response format
        use crate::okx::{GetMarkPriceCandlesResponse, GetMarkPriceCandlesHistoryResponse};
        use serde_json::json;

        let sample_response = json!({
            "code": "0",
            "msg": "",
            "data": [
                ["1597026383085", "11432.1", "11446.3", "11430.2", "11435.7", "1"],
                ["1597026443085", "11435.7", "11441.2", "11432.0", "11439.8", "1"]
            ]
        });

        // Test mark price candles response
        let mark_response: GetMarkPriceCandlesResponse = 
            serde_json::from_value(sample_response.clone()).unwrap();
        
        assert_eq!(mark_response.code, "0");
        assert_eq!(mark_response.data.len(), 2);
        
        // Verify data format: [ts, o, h, l, c, confirm]
        let first_candle = &mark_response.data[0];
        assert_eq!(first_candle[0], "1597026383085"); // timestamp
        assert_eq!(first_candle[1], "11432.1");       // open
        assert_eq!(first_candle[2], "11446.3");       // high
        assert_eq!(first_candle[3], "11430.2");       // low
        assert_eq!(first_candle[4], "11435.7");       // close
        assert_eq!(first_candle[5], "1");             // confirm

        // Test mark price candles history response
        let history_response: GetMarkPriceCandlesHistoryResponse = 
            serde_json::from_value(sample_response).unwrap();
        
        assert_eq!(history_response.code, "0");
        assert_eq!(history_response.data.len(), 2);
    }
}