use serde::{Deserialize, Serialize};

use super::client::RestClient;
// Reuse the result structure from create_block_rfq since the API returns the same Block RFQ object
use super::create_block_rfq::CreateBlockRfqResult;
use crate::deribit::{EndpointType, RestResult};

/// Request parameters for cancel block RFQ endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelBlockRfqRequest {
    /// ID of the Block RFQ
    pub block_rfq_id: i64,
}

/// Response for cancel block RFQ endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelBlockRfqResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result containing the cancelled Block RFQ details
    pub result: CreateBlockRfqResult,
}

impl RestClient {
    /// Cancel a Block RFQ
    ///
    /// This method cancels a Block RFQ using the specified `block_rfq_id`.
    /// This is a taker method that cancels an existing Block RFQ.
    ///
    /// This endpoint requires block_rfq:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-cancel_block_rfq>
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: block_rfq:read_write
    ///
    /// # Arguments
    /// * `block_rfq_id` - ID of the Block RFQ to cancel
    ///
    /// # Returns
    /// Result containing the cancelled Block RFQ details
    pub async fn cancel_block_rfq(
        &self,
        block_rfq_id: i64,
    ) -> RestResult<CancelBlockRfqResponse> {
        let request = CancelBlockRfqRequest { block_rfq_id };
        self.send_signed_request(
            "private/cancel_block_rfq",
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::{json, Value};

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
        let request = CancelBlockRfqRequest {
            block_rfq_id: 123456,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("block_rfq_id").unwrap(), 123456);
    }

    #[test]
    fn test_response_structure_deserialization() {
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
                "state": "cancelled"
            }
        });

        let response: CancelBlockRfqResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.block_rfq_id, 123);
        assert_eq!(response.result.combo_id, "combo_123");
        assert_eq!(response.result.disclosed, true);
        assert_eq!(response.result.mark_price, 45000.0);
        assert_eq!(response.result.min_trade_amount, 0.01);
        assert_eq!(response.result.role, "taker");
        assert_eq!(response.result.state, "cancelled");
        assert_eq!(response.result.legs.len(), 1);
        assert_eq!(response.result.legs[0].direction, "buy");
        assert_eq!(response.result.legs[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(response.result.legs[0].ratio, 1);
    }

    #[test]
    fn test_response_with_complex_structure() {
        let response_json = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": {
                "amount": 5.5,
                "app_name": "test_app",
                "asks": [{
                    "amount": 5.5,
                    "execution_instruction": "any_part_of",
                    "expires_at": 1672738194824i64,
                    "last_update_timestamp": 1672738134824i64,
                    "makers": ["maker1", "maker2"],
                    "price": 45100.0
                }],
                "bids": [{
                    "amount": 5.5,
                    "execution_instruction": "all_or_none",
                    "expires_at": 1672738194824i64,
                    "last_update_timestamp": 1672738134824i64,
                    "makers": ["maker3"],
                    "price": 44900.0
                }],
                "block_rfq_id": 456,
                "combo_id": "combo_456",
                "creation_timestamp": 1672738134824i64,
                "disclosed": false,
                "expiration_timestamp": 1672738194824i64,
                "hedge": {
                    "amount": 100,
                    "direction": "sell",
                    "instrument_name": "BTC-PERPETUAL",
                    "price": 45000.0
                },
                "included_in_taker_rating": true,
                "index_prices": [45000.0, 45050.0],
                "label": "test_label",
                "legs": [{
                    "direction": "buy",
                    "instrument_name": "BTC-29DEC23-40000-C",
                    "ratio": 1
                }, {
                    "direction": "sell",
                    "instrument_name": "BTC-29DEC23-50000-C",
                    "ratio": 2
                }],
                "makers": ["maker1", "maker2"],
                "mark_price": 45000.0,
                "min_trade_amount": 0.01,
                "role": "taker",
                "state": "cancelled"
            }
        });

        let response: CancelBlockRfqResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 42);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.block_rfq_id, 456);
        assert_eq!(response.result.combo_id, "combo_456");
        assert_eq!(response.result.disclosed, false);
        assert_eq!(response.result.app_name, Some("test_app".to_string()));
        assert_eq!(response.result.label, Some("test_label".to_string()));
        assert_eq!(response.result.included_in_taker_rating, Some(true));
        assert_eq!(response.result.asks.len(), 1);
        assert_eq!(response.result.bids.len(), 1);
        assert_eq!(response.result.legs.len(), 2);
        assert_eq!(response.result.makers, vec!["maker1", "maker2"]);
        assert_eq!(response.result.index_prices, vec![45000.0, 45050.0]);
        
        // Test hedge information
        let hedge = response.result.hedge.unwrap();
        assert_eq!(hedge.amount, 100);
        assert_eq!(hedge.direction, "sell");
        assert_eq!(hedge.instrument_name, "BTC-PERPETUAL");
        assert_eq!(hedge.price, 45000.0);
    }

    #[tokio::test]
    async fn test_cancel_block_rfq_method_exists() {
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
        let _ = RestClient::cancel_block_rfq;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_block_rfq method is accessible and properly typed");
    }
}