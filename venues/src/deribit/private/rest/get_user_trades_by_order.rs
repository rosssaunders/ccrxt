use serde::{Deserialize, Serialize};

use super::RestClient;
pub use super::get_user_trades_by_currency::Trade;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, Sorting};

/// REST API endpoint constant
const GET_USER_TRADES_BY_ORDER_ENDPOINT: &str = "private/get_user_trades_by_order";

/// Request parameters for getting user trades by order
#[derive(Debug, Clone, Serialize)]
pub struct GetUserTradesByOrderRequest {
    /// The order id
    pub order_id: String,

    /// Direction of results sorting (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Sorting>,

    /// Determines whether historical trade and order records should be retrieved (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
}

/// Result data for get user trades by order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTradesByOrderResult {
    /// Whether there are more trades available
    pub has_more: bool,

    /// Array of trades
    pub trades: Vec<Trade>,
}

/// Response for get user trades by order endpoint
pub type GetUserTradesByOrderResponse = JsonRpcResult<GetUserTradesByOrderResult>;

impl RestClient {
    /// Retrieve the list of user trades that was created for given order
    ///
    /// This is a private method; it can only be used after authentication.
    /// Scope: trade:read
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-get_user_trades_by_order
    ///
    /// Rate limit: Non-matching engine rate limits apply (500 credits)
    ///
    /// # Arguments
    /// * `order_id` - The order id (required)
    /// * `sorting` - Direction of results sorting (optional)
    /// * `historical` - Whether to retrieve historical records (optional)
    ///
    /// # Returns
    /// Trade history information for the specified order
    pub async fn get_user_trades_by_order(
        &self,
        request: GetUserTradesByOrderRequest,
    ) -> RestResult<GetUserTradesByOrderResponse> {
        self.send_signed_request(
            GET_USER_TRADES_BY_ORDER_ENDPOINT,
            &request,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
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
        let request = GetUserTradesByOrderRequest {
            order_id: "ETH-12345".to_string(),
            sorting: Some(Sorting::Desc),
            historical: Some(false),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("order_id").unwrap(), "ETH-12345");
        assert_eq!(json_value.get("sorting").unwrap(), "desc");
        assert_eq!(json_value.get("historical").unwrap(), false);
    }

    #[test]
    fn test_request_with_minimal_parameters() {
        let request = GetUserTradesByOrderRequest {
            order_id: "BTC-67890".to_string(),
            sorting: None,
            historical: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("order_id").unwrap(), "BTC-67890");

        // Optional fields should not be present when None
        assert!(!json_value.as_object().unwrap().contains_key("sorting"));
        assert!(!json_value.as_object().unwrap().contains_key("historical"));
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

        let response: GetUserTradesByOrderResponse = serde_json::from_value(response_json).unwrap();

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

        let response: GetUserTradesByOrderResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(!response.result.has_more);
        assert_eq!(response.result.trades.len(), 0);
    }

    #[test]
    fn test_sorting_parameter_values() {
        // Test all sorting values
        let request_asc = GetUserTradesByOrderRequest {
            order_id: "test-order".to_string(),
            sorting: Some(Sorting::Asc),
            historical: None,
        };

        let request_desc = GetUserTradesByOrderRequest {
            order_id: "test-order".to_string(),
            sorting: Some(Sorting::Desc),
            historical: None,
        };

        let request_default = GetUserTradesByOrderRequest {
            order_id: "test-order".to_string(),
            sorting: Some(Sorting::Default),
            historical: None,
        };

        let json_asc = serde_json::to_string(&request_asc).unwrap();
        let json_desc = serde_json::to_string(&request_desc).unwrap();
        let json_default = serde_json::to_string(&request_default).unwrap();

        assert!(json_asc.contains("\"sorting\":\"asc\""));
        assert!(json_desc.contains("\"sorting\":\"desc\""));
        assert!(json_default.contains("\"sorting\":\"default\""));
    }

    #[tokio::test]
    async fn test_get_user_trades_by_order_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key =
            Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::get_user_trades_by_order;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_user_trades_by_order method is accessible and properly typed");
    }
}
