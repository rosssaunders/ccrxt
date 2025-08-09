use serde::Serialize;

use super::RestClient;
use super::add_block_rfq_quote::{AddBlockRfqQuoteResult, BlockRfqHedge, BlockRfqLeg};
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const EDIT_BLOCK_RFQ_QUOTE_ENDPOINT: &str = "private/edit_block_rfq_quote";

/// Request parameters for editing a Block RFQ quote
#[derive(Debug, Clone, Serialize)]
pub struct EditBlockRfqQuoteRequest {
    /// List of legs used for Block RFQ quote
    pub legs: Vec<BlockRfqLeg>,

    /// This value multiplied by the ratio of a leg gives trade size on that leg
    pub amount: f64,

    /// ID of the Block RFQ quote (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_quote_id: Option<i64>,

    /// User defined label for the Block RFQ quote (maximum 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Hedge leg of the Block RFQ (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hedge: Option<BlockRfqHedge>,

    /// ID of the Block RFQ (optional, used with label to identify quote)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_id: Option<i64>,

    /// Aggregated price used for quoting future spreads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}

/// Response for edit Block RFQ quote endpoint
pub type EditBlockRfqQuoteResponse = JsonRpcResult<AddBlockRfqQuoteResult>;

impl RestClient {
    /// Edit a Block RFQ quote
    ///
    /// This method edits a Block RFQ quote using the specified `block_rfq_quote_id`.
    /// Alternatively, user can use a combination of `block_rfq_id` and `label` to edit
    /// the quote.
    ///
    /// This endpoint requires block_rfq:read_write scope.
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: block_rfq:read_write
    ///
    /// # Arguments
    /// * `legs` - List of legs used for Block RFQ quote
    /// * `amount` - This value multiplied by the ratio of a leg gives trade size on that leg
    /// * `block_rfq_quote_id` - Optional ID of the Block RFQ quote
    /// * `label` - Optional user defined label for the Block RFQ quote (maximum 64 characters)
    /// * `hedge` - Optional hedge leg of the Block RFQ
    /// * `block_rfq_id` - Optional ID of the Block RFQ (used with label to identify quote)
    /// * `price` - Optional aggregated price used for quoting future spreads
    ///
    /// # Returns
    /// Result containing the edited Block RFQ quote details
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-edit_block_rfq_quote
    pub async fn edit_block_rfq_quote(
        &self,
        request: EditBlockRfqQuoteRequest,
    ) -> RestResult<EditBlockRfqQuoteResponse> {
        self.send_signed_request(
            EDIT_BLOCK_RFQ_QUOTE_ENDPOINT,
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

    use super::{super::add_block_rfq_quote::Side, *};
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
    fn test_request_serialization_minimal() {
        let request = EditBlockRfqQuoteRequest {
            legs: vec![BlockRfqLeg {
                instrument_name: "BTC-PERPETUAL".to_string(),
                price: 50000.0,
                ratio: 1,
                direction: Side::Buy,
            }],
            amount: 1.0,
            block_rfq_quote_id: None,
            label: None,
            hedge: None,
            block_rfq_id: None,
            price: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("legs").unwrap().is_array());
        assert_eq!(json_value.get("amount").unwrap(), 1.0);
        assert!(json_value.get("block_rfq_quote_id").is_none());
        assert!(json_value.get("label").is_none());
        assert!(json_value.get("hedge").is_none());
        assert!(json_value.get("block_rfq_id").is_none());
        assert!(json_value.get("price").is_none());
    }

    #[test]
    fn test_request_serialization_with_quote_id() {
        let request = EditBlockRfqQuoteRequest {
            legs: vec![BlockRfqLeg {
                instrument_name: "ETH-PERPETUAL".to_string(),
                price: 3000.0,
                ratio: 1,
                direction: Side::Sell,
            }],
            amount: 2.5,
            block_rfq_quote_id: Some(12345),
            label: None,
            hedge: None,
            block_rfq_id: None,
            price: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("amount").unwrap(), 2.5);
        assert_eq!(json_value.get("block_rfq_quote_id").unwrap(), 12345);
        assert!(json_value.get("label").is_none());
        assert!(json_value.get("block_rfq_id").is_none());
    }

    #[test]
    fn test_request_serialization_with_label_and_rfq_id() {
        let request = EditBlockRfqQuoteRequest {
            legs: vec![BlockRfqLeg {
                instrument_name: "SOL-PERPETUAL".to_string(),
                price: 100.0,
                ratio: 2,
                direction: Side::Buy,
            }],
            amount: 5.0,
            block_rfq_quote_id: None,
            label: Some("my_edit_quote".to_string()),
            hedge: None,
            block_rfq_id: Some(67890),
            price: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("amount").unwrap(), 5.0);
        assert!(json_value.get("block_rfq_quote_id").is_none());
        assert_eq!(json_value.get("label").unwrap(), "my_edit_quote");
        assert_eq!(json_value.get("block_rfq_id").unwrap(), 67890);
    }

    #[test]
    fn test_request_serialization_with_hedge() {
        let request = EditBlockRfqQuoteRequest {
            legs: vec![BlockRfqLeg {
                instrument_name: "BTC-PERPETUAL".to_string(),
                price: 45000.0,
                ratio: 1,
                direction: Side::Buy,
            }],
            amount: 1.5,
            block_rfq_quote_id: Some(98765),
            label: None,
            hedge: Some(BlockRfqHedge {
                instrument_name: "ETH-PERPETUAL".to_string(),
                direction: Side::Sell,
                price: 2800.0,
                amount: 3.0,
            }),
            block_rfq_id: None,
            price: Some(44000.0),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("amount").unwrap(), 1.5);
        assert_eq!(json_value.get("block_rfq_quote_id").unwrap(), 98765);
        assert!(json_value.get("hedge").is_some());
        assert_eq!(json_value.get("price").unwrap(), 44000.0);

        let hedge = json_value.get("hedge").unwrap();
        assert_eq!(hedge.get("instrument_name").unwrap(), "ETH-PERPETUAL");
        assert_eq!(hedge.get("direction").unwrap(), "sell");
        assert_eq!(hedge.get("price").unwrap(), 2800.0);
        assert_eq!(hedge.get("amount").unwrap(), 3.0);
    }

    #[test]
    fn test_request_serialization_full() {
        let request = EditBlockRfqQuoteRequest {
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
            amount: 2.0,
            block_rfq_quote_id: Some(11111),
            label: Some("test_edit_label".to_string()),
            hedge: Some(BlockRfqHedge {
                instrument_name: "SOL-PERPETUAL".to_string(),
                direction: Side::Buy,
                price: 120.0,
                amount: 5.0,
            }),
            block_rfq_id: Some(22222),
            price: Some(48000.0),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("legs").unwrap().as_array().unwrap().len(), 2);
        assert_eq!(json_value.get("amount").unwrap(), 2.0);
        assert_eq!(json_value.get("block_rfq_quote_id").unwrap(), 11111);
        assert_eq!(json_value.get("label").unwrap(), "test_edit_label");
        assert!(json_value.get("hedge").is_some());
        assert_eq!(json_value.get("block_rfq_id").unwrap(), 22222);
        assert_eq!(json_value.get("price").unwrap(), 48000.0);
    }

    #[test]
    fn test_response_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "amount": 2.0,
                "app_name": null,
                "block_rfq_id": 123,
                "block_rfq_quote_id": 456,
                "creation_timestamp": 1640995200000i64,
                "direction": "buy",
                "execution_instruction": "any_part_of",
                "filled_amount": 0.0,
                "hedge": {
                    "amount": 100,
                    "direction": "sell",
                    "instrument_name": "ETH-PERPETUAL",
                    "price": 3000.0
                },
                "label": "edited_quote",
                "last_update_timestamp": 1640995300000i64,
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
                "replaced": true
            }
        });

        let response: EditBlockRfqQuoteResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 2.0);
        assert_eq!(response.result.block_rfq_id, 123);
        assert_eq!(response.result.block_rfq_quote_id, 456);
        assert_eq!(response.result.legs.len(), 1);
        assert_eq!(response.result.legs[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(response.result.label, Some("edited_quote".to_string()));
        assert!(response.result.replaced);
        assert!(response.result.hedge.is_some());
    }

    #[tokio::test]
    async fn test_edit_block_rfq_quote_method_exists() {
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
        let _ = RestClient::edit_block_rfq_quote;

        // Verify the client exists
        let _ = &rest_client;

        println!("edit_block_rfq_quote method is accessible and properly typed");
    }
}
