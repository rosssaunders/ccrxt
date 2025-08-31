use serde::Serialize;

use super::add_block_rfq_quote::AddBlockRfqQuoteResult;
use crate::deribit::{EndpointType, JsonRpcResult, PrivateRestClient, RestResult};

/// REST API endpoint constant
const GET_BLOCK_RFQ_QUOTES_ENDPOINT: &str = "private/get_block_rfq_quotes";

/// Request parameters for get block RFQ quotes endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetBlockRfqQuotesRequest {
    /// ID of the Block RFQ (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_id: Option<i64>,

    /// User defined label for the Block RFQ quote (maximum 64 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// ID of the Block RFQ quote (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq_quote_id: Option<i64>,
}

/// Response for get block RFQ quotes endpoint
pub type GetBlockRfqQuotesResponse = JsonRpcResult<Vec<AddBlockRfqQuoteResult>>;

impl PrivateRestClient {
    /// Retrieves all open quotes for Block RFQs.
    ///
    /// This method retrieves all open quotes for Block RFQs. When a `block_rfq_id` is
    /// specified, only the open quotes for that particular Block RFQ will be returned.
    /// When a `label` is specified, all quotes with this label are returned.
    /// `block_rfq_quote_id` returns one specific quote.
    ///
    /// # Arguments
    /// * `block_rfq_id` - Optional ID of the Block RFQ
    /// * `label` - Optional user defined label for the Block RFQ quote (maximum 64 characters)
    /// * `block_rfq_quote_id` - Optional ID of the Block RFQ quote
    ///
    /// # Returns
    /// Result containing the array of Block RFQ quotes
    ///
    /// # Scope
    /// `block_rfq:read`
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_block_rfq_quotes)
    pub async fn get_block_rfq_quotes(
        &self,
        request: GetBlockRfqQuotesRequest,
    ) -> RestResult<GetBlockRfqQuotesResponse> {
        self.send_signed_request(
            GET_BLOCK_RFQ_QUOTES_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    /// REST API endpoint constant
    use super::*;

    #[test]
    fn test_request_serialization_empty() {
        let request = GetBlockRfqQuotesRequest {
            block_rfq_id: None,
            label: None,
            block_rfq_quote_id: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // All fields should be omitted when None
        assert!(json_value.get("block_rfq_id").is_none());
        assert!(json_value.get("label").is_none());
        assert!(json_value.get("block_rfq_quote_id").is_none());
    }

    #[test]
    fn test_request_serialization_with_block_rfq_id() {
        let request = GetBlockRfqQuotesRequest {
            block_rfq_id: Some(12345),
            label: None,
            block_rfq_quote_id: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("block_rfq_id").unwrap(), 12345);
        assert!(json_value.get("label").is_none());
        assert!(json_value.get("block_rfq_quote_id").is_none());
    }

    #[test]
    fn test_request_serialization_with_label() {
        let request = GetBlockRfqQuotesRequest {
            block_rfq_id: None,
            label: Some("test_label".to_string()),
            block_rfq_quote_id: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("block_rfq_id").is_none());
        assert_eq!(json_value.get("label").unwrap(), "test_label");
        assert!(json_value.get("block_rfq_quote_id").is_none());
    }

    #[test]
    fn test_request_serialization_with_quote_id() {
        let request = GetBlockRfqQuotesRequest {
            block_rfq_id: None,
            label: None,
            block_rfq_quote_id: Some(67890),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("block_rfq_id").is_none());
        assert!(json_value.get("label").is_none());
        assert_eq!(json_value.get("block_rfq_quote_id").unwrap(), 67890);
    }

    #[test]
    fn test_request_serialization_all_fields() {
        let request = GetBlockRfqQuotesRequest {
            block_rfq_id: Some(123),
            label: Some("all_fields_test".to_string()),
            block_rfq_quote_id: Some(456),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("block_rfq_id").unwrap(), 123);
        assert_eq!(json_value.get("label").unwrap(), "all_fields_test");
        assert_eq!(json_value.get("block_rfq_quote_id").unwrap(), 456);
    }

    #[test]
    fn test_response_deserialization_empty_array() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": []
        });

        let response: GetBlockRfqQuotesResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 0);
    }

    #[test]
    fn test_response_deserialization_with_quotes() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": [
                {
                    "amount": 1.0,
                    "app_name": null,
                    "block_rfq_id": 123,
                    "block_rfq_quote_id": 456,
                    "creation_timestamp": 1640995200000i64,
                    "direction": "buy",
                    "execution_instruction": "any_part_of",
                    "filled_amount": 0.0,
                    "hedge": null,
                    "label": "test_quote",
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
                    "replaced": false
                }
            ]
        });

        let response: GetBlockRfqQuotesResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 1);

        let quote = &response.result[0];
        assert_eq!(quote.amount, 1.0);
        assert_eq!(quote.block_rfq_id, 123);
        assert_eq!(quote.block_rfq_quote_id, 456);
        assert_eq!(quote.label, Some("test_quote".to_string()));
        assert_eq!(quote.legs.len(), 1);
        assert_eq!(quote.legs[0].instrument_name, "BTC-PERPETUAL");
        assert!(!quote.replaced);
    }
}
