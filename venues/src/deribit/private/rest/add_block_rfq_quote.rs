use serde::{Deserialize, Serialize};

use super::RestClient;
// Reuse the Side enum from send_rfq for direction fields
pub use super::send_rfq::Side;
use crate::deribit::{EndpointType, RestResult};

/// Execution instruction for Block RFQ quotes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ExecutionInstruction {
    /// All or none - quote can only be filled entirely or not at all
    AllOrNone,
    /// Any part of - quote can be filled partially or fully (default)
    #[default]
    AnyPartOf,
}

/// A leg in the Block RFQ quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqLeg {
    /// Instrument name
    pub instrument_name: String,
    /// Price for trade
    pub price: f64,
    /// Ratio of amount between legs
    pub ratio: i32,
    /// Direction of selected leg (buy or sell)
    pub direction: Side,
}

/// Hedge leg of the Block RFQ (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqHedge {
    /// Instrument name
    pub instrument_name: String,
    /// Direction of selected leg (buy or sell)
    pub direction: Side,
    /// Hedge leg price
    pub price: f64,
    /// Trade size for hedge leg
    pub amount: f64,
}

/// Request parameters for adding a Block RFQ quote
#[derive(Debug, Clone, Serialize)]
pub struct AddBlockRfqQuoteRequest {
    /// User defined label for the Block RFQ quote (maximum 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// ID of the Block RFQ
    pub block_rfq_id: i64,
    /// This value multiplied by the ratio of a leg gives trade size on that leg
    pub amount: f64,
    /// Direction of trade from the maker perspective
    pub direction: Side,
    /// List of legs used for Block RFQ quote
    pub legs: Vec<BlockRfqLeg>,
    /// Hedge leg of the Block RFQ (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hedge: Option<BlockRfqHedge>,
    /// Execution instruction of the quote
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_instruction: Option<ExecutionInstruction>,
    /// Aggregated price used for quoting future spreads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// The timestamp when the quote expires (milliseconds since the Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}

/// Response leg data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseLeg {
    /// Direction: buy or sell
    pub direction: Side,
    /// Unique instrument identifier
    pub instrument_name: String,
    /// Price for a leg
    pub price: f64,
    /// Ratio of amount between legs
    pub ratio: i32,
}

/// Response hedge data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHedge {
    /// Trade size for hedge leg
    pub amount: i64,
    /// Direction: buy or sell
    pub direction: Side,
    /// Unique instrument identifier
    pub instrument_name: String,
    /// Price for a hedge leg
    pub price: f64,
}

/// The result data from adding a Block RFQ quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBlockRfqQuoteResult {
    /// This value multiplied by the ratio of a leg gives trade size on that leg
    pub amount: f64,
    /// The name of the application that placed the quote on behalf of the user (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// ID of the Block RFQ
    pub block_rfq_id: i64,
    /// ID of the Block RFQ quote
    pub block_rfq_quote_id: i64,
    /// The timestamp when quote was created (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,
    /// Direction of trade from the maker perspective
    pub direction: Side,
    /// Execution instruction of the quote
    pub execution_instruction: ExecutionInstruction,
    /// Filled amount of the quote
    pub filled_amount: f64,
    /// Hedge leg information (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hedge: Option<ResponseHedge>,
    /// User defined label for the quote (maximum 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Timestamp of the last update of the quote (milliseconds since the UNIX epoch)
    pub last_update_timestamp: i64,
    /// List of legs in the quote
    pub legs: Vec<ResponseLeg>,
    /// Price of a quote
    pub price: f64,
    /// State of the quote
    pub quote_state: String,
    /// Reason of quote cancellation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_state_reason: Option<String>,
    /// true if the quote was edited
    pub replaced: bool,
}

/// Response for add Block RFQ quote endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBlockRfqQuoteResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result containing the Block RFQ quote details
    pub result: AddBlockRfqQuoteResult,
}

impl RestClient {
    /// Add quote to existing Block RFQ
    ///
    /// This method adds quote to the existing Block RFQ. To calculate individual leg
    /// prices, the `private/get_leg_prices` endpoint can be utilized.
    ///
    /// This endpoint requires block_rfq:read_write scope.
    ///
    /// Rate limit: Depends on endpoint type (matching engine)
    /// Scope: block_rfq:read_write
    ///
    /// # Arguments
    /// * `label` - Optional user defined label for the Block RFQ quote (maximum 64 characters)
    /// * `block_rfq_id` - ID of the Block RFQ
    /// * `amount` - This value multiplied by the ratio of a leg gives trade size on that leg
    /// * `direction` - Direction of trade from the maker perspective
    /// * `legs` - List of legs used for Block RFQ quote
    /// * `hedge` - Optional hedge leg of the Block RFQ
    /// * `execution_instruction` - Optional execution instruction (default: any_part_of)
    /// * `price` - Optional aggregated price used for quoting future spreads
    /// * `expires_at` - Optional timestamp when the quote expires (milliseconds since Unix epoch)
    ///
    /// # Returns
    /// Result containing the Block RFQ quote details
    pub async fn add_block_rfq_quote(
        &self,
        request: AddBlockRfqQuoteRequest,
    ) -> RestResult<AddBlockRfqQuoteResponse> {
        self.send_signed_request(
            "private/add_block_rfq_quote",
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
    fn test_execution_instruction_serialization() {
        let all_or_none = ExecutionInstruction::AllOrNone;
        let any_part_of = ExecutionInstruction::AnyPartOf;

        let all_or_none_json = serde_json::to_string(&all_or_none).unwrap();
        let any_part_of_json = serde_json::to_string(&any_part_of).unwrap();

        assert_eq!(all_or_none_json, "\"all_or_none\"");
        assert_eq!(any_part_of_json, "\"any_part_of\"");
    }

    #[test]
    fn test_execution_instruction_deserialization() {
        let all_or_none: ExecutionInstruction = serde_json::from_str("\"all_or_none\"").unwrap();
        let any_part_of: ExecutionInstruction = serde_json::from_str("\"any_part_of\"").unwrap();

        matches!(all_or_none, ExecutionInstruction::AllOrNone);
        matches!(any_part_of, ExecutionInstruction::AnyPartOf);
    }

    #[test]
    fn test_execution_instruction_default() {
        let default_instruction = ExecutionInstruction::default();
        matches!(default_instruction, ExecutionInstruction::AnyPartOf);
    }

    #[test]
    fn test_block_rfq_leg_serialization() {
        let leg = BlockRfqLeg {
            instrument_name: "BTC-PERPETUAL".to_string(),
            price: 50000.0,
            ratio: 1,
            direction: Side::Buy,
        };

        let json_str = serde_json::to_string(&leg).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "BTC-PERPETUAL");
        assert_eq!(json_value.get("price").unwrap(), 50000.0);
        assert_eq!(json_value.get("ratio").unwrap(), 1);
        assert_eq!(json_value.get("direction").unwrap(), "buy");
    }

    #[test]
    fn test_block_rfq_hedge_serialization() {
        let hedge = BlockRfqHedge {
            instrument_name: "ETH-PERPETUAL".to_string(),
            direction: Side::Sell,
            price: 3000.0,
            amount: 1.5,
        };

        let json_str = serde_json::to_string(&hedge).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "ETH-PERPETUAL");
        assert_eq!(json_value.get("direction").unwrap(), "sell");
        assert_eq!(json_value.get("price").unwrap(), 3000.0);
        assert_eq!(json_value.get("amount").unwrap(), 1.5);
    }

    #[test]
    fn test_request_serialization_minimal() {
        let request = AddBlockRfqQuoteRequest {
            label: None,
            block_rfq_id: 123,
            amount: 1.0,
            direction: Side::Buy,
            legs: vec![BlockRfqLeg {
                instrument_name: "BTC-PERPETUAL".to_string(),
                price: 50000.0,
                ratio: 1,
                direction: Side::Buy,
            }],
            hedge: None,
            execution_instruction: None,
            price: None,
            expires_at: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("block_rfq_id").unwrap(), 123);
        assert_eq!(json_value.get("amount").unwrap(), 1.0);
        assert_eq!(json_value.get("direction").unwrap(), "buy");
        assert!(json_value.get("legs").unwrap().is_array());
        assert!(json_value.get("label").is_none());
        assert!(json_value.get("hedge").is_none());
        assert!(json_value.get("execution_instruction").is_none());
        assert!(json_value.get("price").is_none());
        assert!(json_value.get("expires_at").is_none());
    }

    #[test]
    fn test_request_serialization_full() {
        let request = AddBlockRfqQuoteRequest {
            label: Some("test_quote".to_string()),
            block_rfq_id: 456,
            amount: 2.5,
            direction: Side::Sell,
            legs: vec![
                BlockRfqLeg {
                    instrument_name: "BTC-PERPETUAL".to_string(),
                    price: 50000.0,
                    ratio: 1,
                    direction: Side::Buy,
                },
                BlockRfqLeg {
                    instrument_name: "ETH-PERPETUAL".to_string(),
                    price: 3000.0,
                    ratio: 2,
                    direction: Side::Sell,
                },
            ],
            hedge: Some(BlockRfqHedge {
                instrument_name: "SOL-PERPETUAL".to_string(),
                direction: Side::Buy,
                price: 100.0,
                amount: 10.0,
            }),
            execution_instruction: Some(ExecutionInstruction::AllOrNone),
            price: Some(47500.0),
            expires_at: Some(1640995200000),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("label").unwrap(), "test_quote");
        assert_eq!(json_value.get("block_rfq_id").unwrap(), 456);
        assert_eq!(json_value.get("amount").unwrap(), 2.5);
        assert_eq!(json_value.get("direction").unwrap(), "sell");
        assert_eq!(json_value.get("legs").unwrap().as_array().unwrap().len(), 2);
        assert!(json_value.get("hedge").is_some());
        assert_eq!(
            json_value.get("execution_instruction").unwrap(),
            "all_or_none"
        );
        assert_eq!(json_value.get("price").unwrap(), 47500.0);
        assert_eq!(json_value.get("expires_at").unwrap(), 1640995200000i64);
    }

    #[test]
    fn test_response_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "amount": 1.0,
                "app_name": null,
                "block_rfq_id": 123,
                "block_rfq_quote_id": 789,
                "creation_timestamp": 1640995200000i64,
                "direction": "buy",
                "execution_instruction": "any_part_of",
                "filled_amount": 0.0,
                "hedge": null,
                "label": "test_quote",
                "last_update_timestamp": 1640995200000i64,
                "legs": [
                    {
                        "direction": "buy",
                        "instrument_name": "BTC-PERPETUAL",
                        "price": 50000.0,
                        "ratio": 1
                    }
                ],
                "price": 50000.0,
                "quote_state": "open",
                "quote_state_reason": null,
                "replaced": false
            }
        });

        let response: AddBlockRfqQuoteResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 1.0);
        assert_eq!(response.result.block_rfq_id, 123);
        assert_eq!(response.result.block_rfq_quote_id, 789);
        assert_eq!(response.result.legs.len(), 1);
        assert_eq!(response.result.legs[0].instrument_name, "BTC-PERPETUAL");
        assert!(!response.result.replaced);
    }

    #[tokio::test]
    async fn test_add_block_rfq_quote_method_exists() {
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
        let _ = RestClient::add_block_rfq_quote;

        // Verify the client exists
        let _ = &rest_client;

        println!("add_block_rfq_quote method is accessible and properly typed");
    }
}
