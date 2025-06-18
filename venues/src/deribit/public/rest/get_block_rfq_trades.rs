//! Request and response structs for public/get_block_rfq_trades endpoint
//!
//! This method returns a list of recent Block RFQs trades. Can be optionally
//! filtered by currency.

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{Currency, EndpointType, RestResult};

/// Request parameters for the public/get_block_rfq_trades endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockRfqTradesRequest {
    /// The currency symbol or "any" for all
    pub currency: Currency,
    
    /// The continuation parameter specifies the starting point for fetching historical Block RFQs.
    /// When provided, the endpoint returns Block RFQs, starting from the specified ID and
    /// continuing backward (e.g., if continuation is 50, results will include Block RFQs of ID 49, 48, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<i64>,
    
    /// Count of Block RFQs returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

/// Trade information for a Block RFQ
#[derive(Debug, Clone, Deserialize)]
pub struct BlockRfqTrade {
    /// Trade amount. For options, linear futures, linear perpetuals and spots the amount is
    /// denominated in the underlying base currency coin. The inverse perpetuals and inverse
    /// futures are denominated in USD units.
    pub amount: f64,
    
    /// Direction: "buy", or "sell"
    pub direction: String,
    
    /// Amount of the hedge leg. For linear futures, linear perpetuals and spots the amount is
    /// denominated in the underlying base currency coin. The inverse perpetuals and inverse
    /// futures are denominated in USD units.
    pub hedge_amount: f64,
    
    /// Price in base currency
    pub price: f64,
}

/// Leg information for a Block RFQ
#[derive(Debug, Clone, Deserialize)]
pub struct BlockRfqLeg {
    /// Direction: "buy", or "sell"
    pub direction: String,
    
    /// Unique instrument identifier
    pub instrument_name: String,
    
    /// Price for a leg
    pub price: f64,
    
    /// Ratio of amount between legs
    pub ratio: i32,
}

/// Hedge information for a Block RFQ
#[derive(Debug, Clone, Deserialize)]
pub struct BlockRfqHedge {
    /// It represents the requested hedge leg size. For perpetual and inverse futures the amount
    /// is in USD units. For options and linear futures and it is the underlying base currency coin.
    pub amount: i64,
    
    /// Direction: "buy", or "sell"
    pub direction: String,
    
    /// Unique instrument identifier
    pub instrument_name: String,
    
    /// Price for a hedge leg
    pub price: f64,
}

/// Block RFQ information
#[derive(Debug, Clone, Deserialize)]
pub struct BlockRfq {
    /// This value multiplied by the ratio of a leg gives trade size on that leg.
    pub amount: f64,
    
    /// Unique combo identifier
    pub combo_id: String,
    
    /// Direction: "buy", or "sell"
    pub direction: String,
    
    /// Hedge information
    pub hedge: BlockRfqHedge,
    
    /// ID of the Block RFQ
    pub id: i64,
    
    /// Array of legs
    pub legs: Vec<BlockRfqLeg>,
    
    /// Mark Price at the moment of trade
    pub mark_price: f64,
    
    /// The timestamp of the trade (milliseconds since the UNIX epoch)
    pub timestamp: i64,
    
    /// Array of trades
    pub trades: Vec<BlockRfqTrade>,
}

/// Result data for the get_block_rfq_trades response
#[derive(Debug, Clone, Deserialize)]
pub struct GetBlockRfqTradesResult {
    /// Array of Block RFQ trades
    pub block_rfqs: Vec<BlockRfq>,
    
    /// Continuation token for pagination. NULL when no continuation.
    /// Consists of timestamp and block_rfq_id.
    pub continuation: Option<String>,
}

/// Response for public/get_block_rfq_trades endpoint following Deribit JSON-RPC 2.0 format.
#[derive(Debug, Clone, Deserialize)]
pub struct GetBlockRfqTradesResponse {
    /// The id that was sent in the request
    pub id: i64,

    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,

    /// The result data containing Block RFQ trades
    pub result: GetBlockRfqTradesResult,
}

impl RestClient {
    /// Calls the public/get_block_rfq_trades endpoint.
    ///
    /// This method returns a list of recent Block RFQs trades. Can be optionally
    /// filtered by currency.
    ///
    /// # Arguments
    /// * `params` - The request parameters including currency and optional filters
    ///
    /// # Returns
    /// A result containing the response with Block RFQ trades or an error
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_block_rfq_trades)
    pub async fn get_block_rfq_trades(&self, params: GetBlockRfqTradesRequest) -> RestResult<GetBlockRfqTradesResponse> {
        self.send_request(
            "public/get_block_rfq_trades",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::deribit::{AccountTier, RateLimiter};

    #[test]
    fn test_get_block_rfq_trades_request_serialization() {
        let request = GetBlockRfqTradesRequest {
            currency: Currency::BTC,
            continuation: Some(50),
            count: Some(10),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["currency"], "BTC");
        assert_eq!(json_value["continuation"], 50);
        assert_eq!(json_value["count"], 10);
    }

    #[test]
    fn test_get_block_rfq_trades_request_optional_fields() {
        let request = GetBlockRfqTradesRequest {
            currency: Currency::Any,
            continuation: None,
            count: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value["currency"], "any");
        assert!(!json_value.as_object().unwrap().contains_key("continuation"));
        assert!(!json_value.as_object().unwrap().contains_key("count"));
    }

    #[test]
    fn test_block_rfq_trade_deserialization() {
        let trade_json = json!({
            "amount": 1.5,
            "direction": "buy",
            "hedge_amount": 2.0,
            "price": 45000.0
        });

        let trade: BlockRfqTrade = serde_json::from_value(trade_json).unwrap();
        assert_eq!(trade.amount, 1.5);
        assert_eq!(trade.direction, "buy");
        assert_eq!(trade.hedge_amount, 2.0);
        assert_eq!(trade.price, 45000.0);
    }

    #[test]
    fn test_block_rfq_leg_deserialization() {
        let leg_json = json!({
            "direction": "sell",
            "instrument_name": "BTC-PERPETUAL",
            "price": 44500.0,
            "ratio": 1
        });

        let leg: BlockRfqLeg = serde_json::from_value(leg_json).unwrap();
        assert_eq!(leg.direction, "sell");
        assert_eq!(leg.instrument_name, "BTC-PERPETUAL");
        assert_eq!(leg.price, 44500.0);
        assert_eq!(leg.ratio, 1);
    }

    #[test]
    fn test_block_rfq_hedge_deserialization() {
        let hedge_json = json!({
            "amount": 1000,
            "direction": "buy",
            "instrument_name": "BTC-PERPETUAL",
            "price": 45000.0
        });

        let hedge: BlockRfqHedge = serde_json::from_value(hedge_json).unwrap();
        assert_eq!(hedge.amount, 1000);
        assert_eq!(hedge.direction, "buy");
        assert_eq!(hedge.instrument_name, "BTC-PERPETUAL");
        assert_eq!(hedge.price, 45000.0);
    }

    #[test]
    fn test_block_rfq_deserialization() {
        let block_rfq_json = json!({
            "amount": 1.0,
            "combo_id": "combo_123",
            "direction": "buy",
            "hedge": {
                "amount": 1000,
                "direction": "buy",
                "instrument_name": "BTC-PERPETUAL",
                "price": 45000.0
            },
            "id": 12345,
            "legs": [{
                "direction": "sell",
                "instrument_name": "BTC-PERPETUAL",
                "price": 44500.0,
                "ratio": 1
            }],
            "mark_price": 44750.0,
            "timestamp": 1609459200000i64,
            "trades": [{
                "amount": 1.5,
                "direction": "buy",
                "hedge_amount": 2.0,
                "price": 45000.0
            }]
        });

        let block_rfq: BlockRfq = serde_json::from_value(block_rfq_json).unwrap();
        assert_eq!(block_rfq.amount, 1.0);
        assert_eq!(block_rfq.combo_id, "combo_123");
        assert_eq!(block_rfq.direction, "buy");
        assert_eq!(block_rfq.id, 12345);
        assert_eq!(block_rfq.legs.len(), 1);
        assert_eq!(block_rfq.trades.len(), 1);
        assert_eq!(block_rfq.mark_price, 44750.0);
        assert_eq!(block_rfq.timestamp, 1609459200000i64);
    }

    #[test]
    fn test_get_block_rfq_trades_response_structure() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": {
                "block_rfqs": [{
                    "amount": 1.0,
                    "combo_id": "combo_123",
                    "direction": "buy",
                    "hedge": {
                        "amount": 1000,
                        "direction": "buy",
                        "instrument_name": "BTC-PERPETUAL",
                        "price": 45000.0
                    },
                    "id": 12345,
                    "legs": [{
                        "direction": "sell",
                        "instrument_name": "BTC-PERPETUAL",
                        "price": 44500.0,
                        "ratio": 1
                    }],
                    "mark_price": 44750.0,
                    "timestamp": 1609459200000i64,
                    "trades": [{
                        "amount": 1.5,
                        "direction": "buy",
                        "hedge_amount": 2.0,
                        "price": 45000.0
                    }]
                }],
                "continuation": "1609459200000_12345"
            }
        });

        let response: GetBlockRfqTradesResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.block_rfqs.len(), 1);
        assert_eq!(response.result.continuation, Some("1609459200000_12345".to_string()));
    }

    #[test]
    fn test_response_with_no_continuation() {
        let response_json = json!({
            "id": 456,
            "jsonrpc": "2.0",
            "result": {
                "block_rfqs": [],
                "continuation": null
            }
        });

        let response: GetBlockRfqTradesResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 456);
        assert_eq!(response.result.block_rfqs.len(), 0);
        assert_eq!(response.result.continuation, None);
    }

    #[test]
    fn test_multiple_currencies() {
        // Test request with different currencies
        let currencies = vec![
            Currency::BTC,
            Currency::ETH,
            Currency::USDC,
            Currency::USDT,
            Currency::EURR,
            Currency::Any,
        ];

        for currency in currencies {
            let request = GetBlockRfqTradesRequest {
                currency: currency.clone(),
                continuation: None,
                count: None,
            };

            let json_value = serde_json::to_value(&request).unwrap();
            assert_eq!(json_value["currency"], currency.to_string());
        }
    }

    #[tokio::test]
    async fn test_endpoint_type_usage() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        // Test that we can create a request - this doesn't actually call the API
        let _request = GetBlockRfqTradesRequest {
            currency: Currency::BTC,
            continuation: None,
            count: Some(10),
        };

        // Test that rate limiting works for this endpoint type
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_rpc_compliance() {
        // Test that the response follows JSON-RPC 2.0 specification
        let response_json = json!({
            "id": 789,
            "jsonrpc": "2.0",
            "result": {
                "block_rfqs": [],
                "continuation": null
            }
        });

        let response: GetBlockRfqTradesResponse = serde_json::from_value(response_json).unwrap();
        
        // Verify JSON-RPC 2.0 compliance
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 789);
        
        // Verify result structure
        assert!(response.result.block_rfqs.is_empty());
        assert!(response.result.continuation.is_none());
    }
}