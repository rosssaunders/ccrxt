use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    AdvancedType, Currency, EndpointType, InstrumentKind, LiquidationSide, Liquidity, OrderDirection, OrderState, RestResult, Sorting, TickDirection,
    TradeOrderType,
};

/// Request parameters for getting user trades by currency
#[derive(Debug, Clone, Serialize)]
pub struct GetUserTradesByCurrencyRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Instrument kind (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,
    /// The ID of the first trade to be returned (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_id: Option<String>,
    /// The ID of the last trade to be returned (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_id: Option<String>,
    /// Number of requested items, default - 10 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The earliest timestamp to return result from (milliseconds since the UNIX epoch) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// The most recent timestamp to return result from (milliseconds since the UNIX epoch) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<i64>,
    /// Direction of results sorting (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting: Option<Sorting>,
    /// Determines whether historical trade and order records should be retrieved (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
    /// The user id for the subaccount (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<i32>,
}

/// Individual trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Unique (per currency) trade identifier
    pub trade_id: String,
    /// Direction of the "tick"
    pub tick_direction: TickDirection,
    /// Currency for fees
    pub fee_currency: String,
    /// True if user order was created with API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<bool>,
    /// Advanced type of user order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<AdvancedType>,
    /// Id of the user order (maker or taker)
    pub order_id: String,
    /// Describes what was role of users order
    pub liquidity: Liquidity,
    /// True if user order is post-only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Direction: buy, or sell
    pub direction: OrderDirection,
    /// Trade size in contract units (optional, may be absent in historical trades)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<f64>,
    /// True if user order is MMP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// User's fee in units of the specified fee_currency
    pub fee: f64,
    /// QuoteID of the user order (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    /// Index Price at the moment of trade
    pub index_price: f64,
    /// User defined label (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Block trade id - when trade was part of a block trade (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_trade_id: Option<String>,
    /// Price in base currency
    pub price: f64,
    /// Optional field containing combo instrument name if the trade is a combo trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combo_id: Option<String>,
    /// Always null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_id: Option<String>,
    /// Order type: "limit", "market", or "liquidation"
    pub order_type: TradeOrderType,
    /// Profit and loss in base currency
    pub profit_loss: f64,
    /// The timestamp of the trade (milliseconds since the UNIX epoch)
    pub timestamp: i64,
    /// Option implied volatility for the price (Option only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<f64>,
    /// Order state
    pub state: OrderState,
    /// Underlying price for implied volatility calculations (Options only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<f64>,
    /// ID of the Block RFQ quote - when trade was part of the Block RFQ (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_quote_id: Option<i64>,
    /// QuoteSet of the user order (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_set_id: Option<String>,
    /// Mark Price at the moment of trade
    pub mark_price: f64,
    /// ID of the Block RFQ - when trade was part of the Block RFQ (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_id: Option<i64>,
    /// Optional field containing combo trade identifier if the trade is a combo trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combo_trade_id: Option<f64>,
    /// True if user order is reduce-only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Trade amount
    pub amount: f64,
    /// Optional field for liquidation info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation: Option<LiquidationSide>,
    /// The sequence number of the trade within instrument
    pub trade_seq: i64,
    /// True if user order is marked by the platform as a risk reducing order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_reducing: Option<bool>,
    /// Unique instrument identifier
    pub instrument_name: String,
    /// Optional field containing leg trades if trade is a combo trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<serde_json::Value>>, // Using generic Value for legs as structure is complex
}

/// Result data for get user trades by currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTradesByCurrencyResult {
    /// Whether there are more trades available
    pub has_more: bool,
    /// Array of trades
    pub trades: Vec<Trade>,
}

/// Response for get user trades by currency endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTradesByCurrencyResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result data containing trades
    pub result: GetUserTradesByCurrencyResult,
}

impl RestClient {
    /// Retrieve the latest user trades that have occurred for instruments in a specific currency symbol
    ///
    /// This is a private method; it can only be used after authentication.
    /// Scope: trade:read
    ///
    /// See: <https://docs.deribit.com/v2/#private-get_user_trades_by_currency>
    ///
    /// Rate limit: Non-matching engine rate limits apply (500 credits)
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (required)
    /// * `kind` - Instrument kind (optional)
    /// * `start_id` - The ID of the first trade to be returned (optional)
    /// * `end_id` - The ID of the last trade to be returned (optional)
    /// * `count` - Number of requested items, default - 10 (optional)
    /// * `start_timestamp` - The earliest timestamp to return result from (optional)
    /// * `end_timestamp` - The most recent timestamp to return result from (optional)
    /// * `sorting` - Direction of results sorting (optional)
    /// * `historical` - Whether to retrieve historical records (optional)
    /// * `subaccount_id` - The user id for the subaccount (optional)
    ///
    /// # Returns
    /// Trade history information for the specified currency
    pub async fn get_user_trades_by_currency(&self, request: GetUserTradesByCurrencyRequest) -> RestResult<GetUserTradesByCurrencyResponse> {
        self.send_signed_request(
            "private/get_user_trades_by_currency",
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
        let request = GetUserTradesByCurrencyRequest {
            currency: Currency::BTC,
            kind: Some(InstrumentKind::Future),
            start_id: Some("BTC-123".to_string()),
            end_id: Some("BTC-456".to_string()),
            count: Some(50),
            start_timestamp: Some(1640995200000),
            end_timestamp: Some(1640995260000),
            sorting: Some(Sorting::Desc),
            historical: Some(false),
            subaccount_id: Some(12345),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("kind").unwrap(), "future");
        assert_eq!(json_value.get("start_id").unwrap(), "BTC-123");
        assert_eq!(json_value.get("end_id").unwrap(), "BTC-456");
        assert_eq!(json_value.get("count").unwrap(), 50);
        assert_eq!(json_value.get("start_timestamp").unwrap(), 1640995200000i64);
        assert_eq!(json_value.get("end_timestamp").unwrap(), 1640995260000i64);
        assert_eq!(json_value.get("sorting").unwrap(), "desc");
        assert_eq!(json_value.get("historical").unwrap(), false);
        assert_eq!(json_value.get("subaccount_id").unwrap(), 12345);
    }

    #[test]
    fn test_request_with_minimal_parameters() {
        let request = GetUserTradesByCurrencyRequest {
            currency: Currency::ETH,
            kind: None,
            start_id: None,
            end_id: None,
            count: None,
            start_timestamp: None,
            end_timestamp: None,
            sorting: None,
            historical: None,
            subaccount_id: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "ETH");

        // Optional fields should not be present when None
        assert!(!json_value.as_object().unwrap().contains_key("kind"));
        assert!(!json_value.as_object().unwrap().contains_key("start_id"));
        assert!(!json_value.as_object().unwrap().contains_key("count"));
        assert!(!json_value.as_object().unwrap().contains_key("sorting"));
        assert!(!json_value.as_object().unwrap().contains_key("historical"));
        assert!(
            !json_value
                .as_object()
                .unwrap()
                .contains_key("subaccount_id")
        );
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

        let response: GetUserTradesByCurrencyResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.has_more);
        assert_eq!(response.result.trades.len(), 1);

        let trade = &response.result.trades[0];
        assert_eq!(trade.trade_id, "BTC-123456");
        assert_eq!(trade.tick_direction, TickDirection::PlusTick);
        assert_eq!(trade.fee_currency, "BTC");
        assert_eq!(trade.api, Some(true));
        assert_eq!(trade.order_id, "ETH-987654");
        assert_eq!(trade.liquidity, Liquidity::Maker);
        assert_eq!(trade.direction, OrderDirection::Buy);
        assert_eq!(trade.fee, 0.0001);
        assert_eq!(trade.index_price, 45000.0);
        assert_eq!(trade.price, 45100.0);
        assert_eq!(trade.order_type, TradeOrderType::Limit);
        assert_eq!(trade.profit_loss, 100.0);
        assert_eq!(trade.timestamp, 1640995200000);
        assert_eq!(trade.state, OrderState::Filled);
        assert_eq!(trade.mark_price, 45050.0);
        assert_eq!(trade.amount, 1000.0);
        assert_eq!(trade.trade_seq, 12345);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
    }

    #[test]
    fn test_trade_with_optional_fields() {
        let trade_json = json!({
            "trade_id": "ETH-789123",
            "tick_direction": "2",
            "fee_currency": "ETH",
            "api": false,
            "advanced": "usd",
            "order_id": "ETH-456789",
            "liquidity": "T",
            "post_only": true,
            "direction": "sell",
            "contracts": 10.0,
            "mmp": true,
            "fee": 0.002,
            "quote_id": "quote_123",
            "index_price": 3000.0,
            "label": "my_trade",
            "block_trade_id": "block_456",
            "price": 3050.0,
            "combo_id": "combo_789",
            "matching_id": null,
            "order_type": "market",
            "profit_loss": -50.0,
            "timestamp": 1640995300000i64,
            "iv": 0.25,
            "state": "filled",
            "underlying_price": 3020.0,
            "block_rfq_quote_id": 12345,
            "quote_set_id": "quote_set_456",
            "mark_price": 3025.0,
            "block_rfq_id": 67890,
            "combo_trade_id": 111.0,
            "reduce_only": true,
            "amount": 500.0,
            "liquidation": "M",
            "trade_seq": 54321,
            "risk_reducing": false,
            "instrument_name": "ETH-29MAR24-3000-C"
        });

        let trade: Trade = serde_json::from_value(trade_json).unwrap();

        assert_eq!(trade.trade_id, "ETH-789123");
        assert_eq!(trade.tick_direction, TickDirection::MinusTick);
        assert_eq!(trade.fee_currency, "ETH");
        assert_eq!(trade.api, Some(false));
        assert_eq!(trade.advanced, Some(AdvancedType::Usd));
        assert_eq!(trade.order_id, "ETH-456789");
        assert_eq!(trade.liquidity, Liquidity::Taker);
        assert_eq!(trade.post_only, Some(true));
        assert_eq!(trade.direction, OrderDirection::Sell);
        assert_eq!(trade.contracts, Some(10.0));
        assert_eq!(trade.mmp, Some(true));
        assert_eq!(trade.fee, 0.002);
        assert_eq!(trade.quote_id, Some("quote_123".to_string()));
        assert_eq!(trade.index_price, 3000.0);
        assert_eq!(trade.label, Some("my_trade".to_string()));
        assert_eq!(trade.block_trade_id, Some("block_456".to_string()));
        assert_eq!(trade.price, 3050.0);
        assert_eq!(trade.combo_id, Some("combo_789".to_string()));
        assert_eq!(trade.matching_id, None);
        assert_eq!(trade.order_type, TradeOrderType::Market);
        assert_eq!(trade.profit_loss, -50.0);
        assert_eq!(trade.timestamp, 1640995300000);
        assert_eq!(trade.iv, Some(0.25));
        assert_eq!(trade.state, OrderState::Filled);
        assert_eq!(trade.underlying_price, Some(3020.0));
        assert_eq!(trade.block_rfq_quote_id, Some(12345));
        assert_eq!(trade.quote_set_id, Some("quote_set_456".to_string()));
        assert_eq!(trade.mark_price, 3025.0);
        assert_eq!(trade.block_rfq_id, Some(67890));
        assert_eq!(trade.combo_trade_id, Some(111.0));
        assert_eq!(trade.reduce_only, Some(true));
        assert_eq!(trade.amount, 500.0);
        assert_eq!(trade.liquidation, Some(LiquidationSide::Maker));
        assert_eq!(trade.trade_seq, 54321);
        assert_eq!(trade.risk_reducing, Some(false));
        assert_eq!(trade.instrument_name, "ETH-29MAR24-3000-C");
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

        let response: GetUserTradesByCurrencyResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(!response.result.has_more);
        assert_eq!(response.result.trades.len(), 0);
    }

    #[tokio::test]
    async fn test_get_user_trades_by_currency_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
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
        let _ = RestClient::get_user_trades_by_currency;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_user_trades_by_currency method is accessible and properly typed");
    }
}
