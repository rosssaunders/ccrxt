use serde::{Deserialize, Serialize};

use super::RestClient;
pub use super::get_user_trades_by_currency::Trade;
use crate::deribit::{Currency, EndpointType, InstrumentKind, JsonRpcResult, RestResult, Sorting};

/// REST API endpoint constant
const GET_USER_TRADES_BY_CURRENCY_AND_TIME_ENDPOINT: &str =
    "private/get_user_trades_by_currency_and_time";

/// Request parameters for getting user trades by currency and time
#[derive(Debug, Clone, Serialize)]
pub struct GetUserTradesByCurrencyAndTimeRequest {
    /// The currency symbol
    pub currency: Currency,

    /// Instrument kind (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,

    /// The earliest timestamp to return result from (milliseconds since the UNIX epoch)
    pub start_timestamp: i64,

    /// The most recent timestamp to return result from (milliseconds since the UNIX epoch)
    pub end_timestamp: i64,

    /// Number of requested items, default - 10 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// Direction of results sorting (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Sorting>,

    /// Determines whether historical trade and order records should be retrieved (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
}

/// Result data for get user trades by currency and time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTradesByCurrencyAndTimeResult {
    /// Whether there are more trades available
    pub has_more: bool,

    /// Array of trades
    pub trades: Vec<Trade>,
}

/// Response for get user trades by currency and time endpoint
pub type GetUserTradesByCurrencyAndTimeResponse =
    JsonRpcResult<GetUserTradesByCurrencyAndTimeResult>;

impl RestClient {
    /// Retrieve the latest user trades that have occurred for instruments in a specific
    /// currency symbol and within a given time range.
    ///
    /// This is a private method; it can only be used after authentication.
    /// Scope: trade:read
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-get_user_trades_by_currency_and_time
    ///
    /// Rate limit: Non-matching engine rate limits apply (500 credits)
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (required)
    /// * `kind` - Instrument kind (optional)
    /// * `start_timestamp` - The earliest timestamp to return result from (milliseconds since the UNIX epoch) (required)
    /// * `end_timestamp` - The most recent timestamp to return result from (milliseconds since the UNIX epoch) (required)
    /// * `count` - Number of requested items, default - 10 (optional)
    /// * `sorting` - Direction of results sorting (optional)
    /// * `historical` - Whether to retrieve historical records (optional)
    ///
    /// # Returns
    /// Trade history information for the specified currency within the time range
    pub async fn get_user_trades_by_currency_and_time(
        &self,
        request: GetUserTradesByCurrencyAndTimeRequest,
    ) -> RestResult<GetUserTradesByCurrencyAndTimeResponse> {
        self.send_signed_request(
            GET_USER_TRADES_BY_CURRENCY_AND_TIME_ENDPOINT,
            &request,
            EndpointType::NonMatchingEngine,
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
        let request = GetUserTradesByCurrencyAndTimeRequest {
            currency: Currency::BTC,
            kind: Some(InstrumentKind::Future),
            start_timestamp: 1640995200000,
            end_timestamp: 1640995260000,
            count: Some(50),
            sorting: Some(Sorting::Desc),
            historical: Some(false),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("kind").unwrap(), "future");
        assert_eq!(json_value.get("start_timestamp").unwrap(), 1640995200000i64);
        assert_eq!(json_value.get("end_timestamp").unwrap(), 1640995260000i64);
        assert_eq!(json_value.get("count").unwrap(), 50);
        assert_eq!(json_value.get("sorting").unwrap(), "desc");
        assert_eq!(json_value.get("historical").unwrap(), false);
    }

    #[test]
    fn test_request_with_minimal_parameters() {
        let request = GetUserTradesByCurrencyAndTimeRequest {
            currency: Currency::ETH,
            kind: None,
            start_timestamp: 1640995200000,
            end_timestamp: 1640995260000,
            count: None,
            sorting: None,
            historical: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("start_timestamp").unwrap(), 1640995200000i64);
        assert_eq!(json_value.get("end_timestamp").unwrap(), 1640995260000i64);

        // Optional fields should not be present when None
        assert!(!json_value.as_object().unwrap().contains_key("kind"));
        assert!(!json_value.as_object().unwrap().contains_key("count"));
        assert!(!json_value.as_object().unwrap().contains_key("sorting"));
        assert!(!json_value.as_object().unwrap().contains_key("historical"));
    }

    #[test]
    fn test_required_timestamps_validation() {
        // Test that timestamps are always serialized (not optional)
        let request = GetUserTradesByCurrencyAndTimeRequest {
            currency: Currency::USDC,
            kind: None,
            start_timestamp: 0,           // Unix epoch
            end_timestamp: 4102444800000, // Far future timestamp
            count: None,
            sorting: None,
            historical: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // These fields must always be present
        assert!(
            json_value
                .as_object()
                .unwrap()
                .contains_key("start_timestamp")
        );
        assert!(
            json_value
                .as_object()
                .unwrap()
                .contains_key("end_timestamp")
        );
        assert_eq!(json_value.get("start_timestamp").unwrap(), 0);
        assert_eq!(json_value.get("end_timestamp").unwrap(), 4102444800000i64);
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

        let response: GetUserTradesByCurrencyAndTimeResponse =
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
        assert_eq!(trade.timestamp, 1640995200000i64);
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

        let response: GetUserTradesByCurrencyAndTimeResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(!response.result.has_more);
        assert_eq!(response.result.trades.len(), 0);
    }

    #[tokio::test]
    async fn test_get_user_trades_by_currency_and_time_method_exists() {
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
        let _ = RestClient::get_user_trades_by_currency_and_time;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_user_trades_by_currency_and_time method is accessible and properly typed");
    }
}
