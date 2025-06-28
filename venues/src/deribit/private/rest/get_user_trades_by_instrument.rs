use serde::{Deserialize, Serialize};

use super::RestClient;
// Reuse the Trade struct from get_user_trades_by_currency since it's identical
pub use super::get_user_trades_by_currency::Trade;
use crate::deribit::{EndpointType, RestResult, Sorting};

/// Request parameters for getting user trades by instrument
#[derive(Debug, Clone, Serialize)]
pub struct GetUserTradesByInstrumentRequest {
    /// Instrument name
    pub instrument_name: String,
    /// The sequence number of the first trade to be returned (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seq: Option<i64>,
    /// The sequence number of the last trade to be returned (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_seq: Option<i64>,
    /// Number of requested items, default - 10 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The earliest timestamp to return result from (milliseconds since the UNIX epoch) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// The most recent timestamp to return result from (milliseconds since the UNIX epoch) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<i64>,
    /// Determines whether historical trade and order records should be retrieved (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
    /// Direction of results sorting (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Sorting>,
}

/// Result data for get user trades by instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTradesByInstrumentResult {
    /// Whether there are more trades available
    pub has_more: bool,
    /// Array of trades
    pub trades: Vec<Trade>,
}

/// Response for get user trades by instrument endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTradesByInstrumentResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result data containing trades
    pub result: GetUserTradesByInstrumentResult,
}

impl RestClient {
    /// Retrieve the latest user trades that have occurred for a specific instrument
    ///
    /// This is a private method; it can only be used after authentication.
    /// Scope: trade:read
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_user_trades_by_instrument>
    ///
    /// Rate limit: Non-matching engine rate limits apply (500 credits)
    ///
    /// # Arguments
    /// * `instrument_name` - Instrument name (required)
    /// * `start_seq` - The sequence number of the first trade to be returned (optional)
    /// * `end_seq` - The sequence number of the last trade to be returned (optional)
    /// * `count` - Number of requested items, default - 10 (optional)
    /// * `start_timestamp` - The earliest timestamp to return result from (optional)
    /// * `end_timestamp` - The most recent timestamp to return result from (optional)
    /// * `historical` - Whether to retrieve historical records (optional)
    /// * `sorting` - Direction of results sorting (optional)
    ///
    /// # Returns
    /// Trade history information for the specified instrument
    pub async fn get_user_trades_by_instrument(
        &self,
        request: GetUserTradesByInstrumentRequest,
    ) -> RestResult<GetUserTradesByInstrumentResponse> {
        self.send_signed_request(
            "private/get_user_trades_by_instrument",
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;

    // Test secret implementation
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_request_parameters_serialization() {
        let request = GetUserTradesByInstrumentRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            start_seq: Some(12345),
            end_seq: Some(67890),
            count: Some(50),
            start_timestamp: Some(1640995200000),
            end_timestamp: Some(1640995260000),
            historical: Some(false),
            sorting: Some(Sorting::Desc),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "BTC-PERPETUAL");
        assert_eq!(json_value.get("start_seq").unwrap(), 12345);
        assert_eq!(json_value.get("end_seq").unwrap(), 67890);
        assert_eq!(json_value.get("count").unwrap(), 50);
        assert_eq!(json_value.get("start_timestamp").unwrap(), 1640995200000i64);
        assert_eq!(json_value.get("end_timestamp").unwrap(), 1640995260000i64);
        assert_eq!(json_value.get("historical").unwrap(), false);
        assert_eq!(json_value.get("sorting").unwrap(), "desc");
    }

    #[test]
    fn test_request_with_minimal_parameters() {
        let request = GetUserTradesByInstrumentRequest {
            instrument_name: "ETH-29MAR24-3000-C".to_string(),
            start_seq: None,
            end_seq: None,
            count: None,
            start_timestamp: None,
            end_timestamp: None,
            historical: None,
            sorting: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(
            json_value.get("instrument_name").unwrap(),
            "ETH-29MAR24-3000-C"
        );

        // Optional fields should not be present when None
        assert!(!json_value.as_object().unwrap().contains_key("start_seq"));
        assert!(!json_value.as_object().unwrap().contains_key("end_seq"));
        assert!(!json_value.as_object().unwrap().contains_key("count"));
        assert!(
            !json_value
                .as_object()
                .unwrap()
                .contains_key("start_timestamp")
        );
        assert!(
            !json_value
                .as_object()
                .unwrap()
                .contains_key("end_timestamp")
        );
        assert!(!json_value.as_object().unwrap().contains_key("historical"));
        assert!(!json_value.as_object().unwrap().contains_key("sorting"));
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "has_more": true,
                "trades": [
                    {
                        "trade_id": "BTC-123456",
                        "tick_direction": "0",
                        "fee_currency": "BTC",
                        "api": true,
                        "order_id": "ETH-987654",
                        "liquidity": "M",
                        "direction": "buy",
                        "fee": 0.0001,
                        "index_price": 45000.0,
                        "price": 45100.0,
                        "order_type": "limit",
                        "profit_loss": 100.0,
                        "timestamp": 1640995200000i64,
                        "state": "filled",
                        "mark_price": 45050.0,
                        "amount": 1000.0,
                        "trade_seq": 12345,
                        "instrument_name": "BTC-PERPETUAL"
                    }
                ]
            }
        });

        let response: GetUserTradesByInstrumentResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.has_more);
        assert_eq!(response.result.trades.len(), 1);

        let trade = &response.result.trades[0];
        assert_eq!(trade.trade_id, "BTC-123456");
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
        assert_eq!(trade.trade_seq, 12345);
        assert_eq!(trade.fee, 0.0001);
        assert_eq!(trade.price, 45100.0);
        assert_eq!(trade.amount, 1000.0);
    }

    #[test]
    fn test_response_with_empty_trades() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": {
                "has_more": false,
                "trades": []
            }
        });

        let response: GetUserTradesByInstrumentResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(!response.result.has_more);
        assert_eq!(response.result.trades.len(), 0);
    }

    #[tokio::test]
    async fn test_get_user_trades_by_instrument_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key =
            Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::get_user_trades_by_instrument;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_user_trades_by_instrument method is accessible and properly typed");
    }
}
