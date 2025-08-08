use serde::{Deserialize, Serialize};

use super::RestClient;
pub use super::add_block_rfq_quote::{BlockRfqHedge, Side};
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const CREATE_BLOCK_RFQ_ENDPOINT: &str = "private/create_block_rfq";

/// A leg in the Block RFQ for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockRfqLeg {
    /// Instrument name
    pub instrument_name: String,

    /// Amount - represents the requested trade size
    pub amount: f64,

    /// Direction of selected leg (buy or sell)
    pub direction: Side,
}

/// Request parameters for creating a Block RFQ
#[derive(Debug, Clone, Serialize)]
pub struct CreateBlockRfqRequest {
    /// List of legs used to create Block RFQ
    pub legs: Vec<CreateBlockRfqLeg>,

    /// Hedge leg of the Block RFQ (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hedge: Option<BlockRfqHedge>,

    /// User defined label for the Block RFQ (maximum 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// List of targeted Block RFQ makers (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub makers: Option<Vec<String>>,

    /// Determines whether the RFQ is non-anonymous (default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosed: Option<bool>,
}

/// Quote information in the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Amount for the quote
    pub amount: f64,
    /// Execution instruction of the quote
    pub execution_instruction: String,
    /// The timestamp when the quote expires (milliseconds since the Unix epoch)
    pub expires_at: i64,
    /// Timestamp of the last update of the quote (milliseconds since the UNIX epoch)
    pub last_update_timestamp: i64,
    /// Makers of the quote
    pub makers: Vec<String>,
    /// Price of the quote
    pub price: f64,
}

/// Response leg data for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseLeg {
    /// Direction: buy or sell
    pub direction: String,
    /// Unique instrument identifier
    pub instrument_name: String,
    /// Ratio of amount between legs
    pub ratio: i32,
}

/// Response hedge data for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHedge {
    /// Amount for hedge leg
    pub amount: i64,
    /// Direction: buy or sell
    pub direction: String,
    /// Unique instrument identifier
    pub instrument_name: String,
    /// Price for hedge leg
    pub price: f64,
}

/// The result data from creating a Block RFQ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockRfqResult {
    /// This value multiplied by the ratio of a leg gives trade size on that leg
    pub amount: f64,

    /// The name of the application that created the Block RFQ on behalf of the user (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,

    /// Ask quotes
    pub asks: Vec<Quote>,

    /// Bid quotes
    pub bids: Vec<Quote>,

    /// ID of the Block RFQ
    pub block_rfq_id: i64,

    /// Unique combo identifier
    pub combo_id: String,

    /// The timestamp when Block RFQ was created (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,

    /// Indicates whether the RFQ was created as non-anonymous
    pub disclosed: bool,

    /// The timestamp when the Block RFQ will expire (milliseconds since the UNIX epoch)
    pub expiration_timestamp: i64,

    /// Hedge leg information (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hedge: Option<ResponseHedge>,

    /// Indicates whether the RFQ is included in the taker's rating calculation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_in_taker_rating: Option<bool>,

    /// A list of index prices for the underlying instrument(s) at the time of trade execution
    pub index_prices: Vec<f64>,

    /// User defined label for the Block RFQ (maximum 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// List of legs in the Block RFQ
    pub legs: Vec<ResponseLeg>,

    /// List of targeted Block RFQ makers
    pub makers: Vec<String>,

    /// The mark price for the instrument
    pub mark_price: f64,

    /// Minimum amount for trading
    pub min_trade_amount: f64,

    /// Role of the user in Block RFQ
    pub role: String,

    /// State of the Block RFQ
    pub state: String,
}

/// Response for create Block RFQ endpoint
pub type CreateBlockRfqResponse = JsonRpcResult<CreateBlockRfqResult>;

impl RestClient {
    /// Create a new Block RFQ
    ///
    /// This method creates a new Block RFQ. This is a taker method that creates
    /// a request for quote on the specified instruments.
    ///
    /// This endpoint requires block_rfq:read_write scope.
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-create_block_rfq
    ///
    /// Rate limit: Depends on endpoint type (matching engine)
    /// Scope: block_rfq:read_write
    ///
    /// # Arguments
    /// * `request` - CreateBlockRfqRequest struct containing all parameters
    ///
    /// # Returns
    /// Result containing the created Block RFQ details
    pub async fn create_block_rfq(
        &self,
        request: CreateBlockRfqRequest,
    ) -> RestResult<CreateBlockRfqResponse> {
        self.send_signed_request(
            CREATE_BLOCK_RFQ_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    /// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;

    // Test secret implementation
    #[derive(Clone)]
    struct PlainTextSecret {
        value: String,
    }

    impl PlainTextSecret {
        fn new(value: String) -> Self {
            Self { value }
        }
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.value.clone()
        }
    }

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let legs = vec![CreateBlockRfqLeg {
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: 10.0,
            direction: Side::Buy,
        }];

        let request = CreateBlockRfqRequest {
            legs,
            hedge: None,
            label: None,
            makers: None,
            disclosed: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("legs").is_some());
        assert!(json_value.get("hedge").is_none());
        assert!(json_value.get("label").is_none());
        assert!(json_value.get("makers").is_none());
        assert!(json_value.get("disclosed").is_none());

        let legs_array = json_value.get("legs").unwrap().as_array().unwrap();
        assert_eq!(legs_array.len(), 1);
        assert_eq!(
            legs_array[0].get("instrument_name").unwrap(),
            "BTC-PERPETUAL"
        );
        assert_eq!(legs_array[0].get("amount").unwrap(), 10.0);
        assert_eq!(legs_array[0].get("direction").unwrap(), "buy");
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let legs = vec![
            CreateBlockRfqLeg {
                instrument_name: "BTC-PERPETUAL".to_string(),
                amount: 10.0,
                direction: Side::Buy,
            },
            CreateBlockRfqLeg {
                instrument_name: "ETH-PERPETUAL".to_string(),
                amount: 5.0,
                direction: Side::Sell,
            },
        ];

        let hedge = BlockRfqHedge {
            instrument_name: "BTC-PERPETUAL".to_string(),
            direction: Side::Sell,
            price: 45000.0,
            amount: 1.0,
        };

        let makers = vec!["maker1".to_string(), "maker2".to_string()];

        let request = CreateBlockRfqRequest {
            legs,
            hedge: Some(hedge),
            label: Some("test_rfq".to_string()),
            makers: Some(makers),
            disclosed: Some(false),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("legs").is_some());
        assert!(json_value.get("hedge").is_some());
        assert_eq!(json_value.get("label").unwrap(), "test_rfq");
        assert!(json_value.get("makers").is_some());
        assert_eq!(json_value.get("disclosed").unwrap(), false);

        let legs_array = json_value.get("legs").unwrap().as_array().unwrap();
        assert_eq!(legs_array.len(), 2);

        let makers_array = json_value.get("makers").unwrap().as_array().unwrap();
        assert_eq!(makers_array.len(), 2);
        assert_eq!(makers_array[0], "maker1");
        assert_eq!(makers_array[1], "maker2");
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "amount": 10.0,
                "asks": [],
                "bids": [],
                "block_rfq_id": 123,
                "combo_id": "combo_123",
                "creation_timestamp": 1672738134824i64,
                "disclosed": true,
                "expiration_timestamp": 1672738194824i64,
                "index_prices": [45000.0],
                "legs": [{
                    "direction": "buy",
                    "instrument_name": "BTC-PERPETUAL",
                    "ratio": 1
                }],
                "makers": [],
                "mark_price": 45000.0,
                "min_trade_amount": 0.01,
                "role": "taker",
                "state": "open"
            }
        });

        let response: CreateBlockRfqResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 10.0);
        assert_eq!(response.result.block_rfq_id, 123);
        assert_eq!(response.result.combo_id, "combo_123");
        assert!(response.result.disclosed);
        assert_eq!(response.result.legs.len(), 1);
        assert_eq!(response.result.legs[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(response.result.legs[0].direction, "buy");
        assert_eq!(response.result.legs[0].ratio, 1);
        assert_eq!(response.result.mark_price, 45000.0);
        assert_eq!(response.result.role, "taker");
        assert_eq!(response.result.state, "open");
    }

    #[tokio::test]
    async fn test_create_block_rfq_method_exists() {
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
        let _ = RestClient::create_block_rfq;

        // Verify the client exists
        let _ = &rest_client;

        println!("create_block_rfq method is accessible and properly typed");
    }
}
