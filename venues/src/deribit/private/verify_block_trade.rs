use super::rest::RestClient;
use crate::deribit::{Errors, EndpointType};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Role in block trade
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TradeRole {
    /// User wants to be maker of trades
    Maker,
    /// User wants to be taker of trades
    Taker,
}

/// Direction of trade from maker perspective
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TradeDirection {
    /// Buy direction
    Buy,
    /// Sell direction
    Sell,
}

/// Individual trade in a block trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Instrument name
    pub instrument_name: String,
    /// Price for trade
    pub price: f64,
    /// Trade size (optional)
    /// For perpetual and inverse futures the amount is in USD units.
    /// For options and linear futures it is the underlying base currency coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Direction of trade from the maker perspective
    pub direction: TradeDirection,
}

/// Request parameters for verify_block_trade
#[derive(Debug, Clone, Serialize)]
pub struct VerifyBlockTradeRequest {
    /// Timestamp, shared with other party (milliseconds since the UNIX epoch)
    pub timestamp: i64,
    /// Nonce, shared with other party
    pub nonce: String,
    /// Describes if user wants to be maker or taker of trades
    pub role: TradeRole,
    /// List of trades for block trade
    pub trades: Vec<Trade>,
}

/// Result data for verify_block_trade response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyBlockTradeResult {
    /// Signature of block trade
    /// It is valid only for 5 minutes around given timestamp
    pub signature: String,
}

/// JSON-RPC response for verify_block_trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyBlockTradeResponse {
    /// The id that was sent in the request
    pub id: serde_json::Value,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result containing the signature
    pub result: VerifyBlockTradeResult,
}

impl RestClient {
    /// Verifies and creates block trade signature
    ///
    /// This endpoint verifies a block trade and returns a signature that is valid
    /// for 5 minutes around the given timestamp. This is a matching engine method
    /// that requires authentication and the `block_trade:read` scope.
    ///
    /// # Arguments
    /// * `request` - The verify block trade request parameters
    ///
    /// # Returns
    /// A result containing the verification response with signature or an error
    ///
    /// # Example
    /// ```rust,no_run
    /// use venues::deribit::private::{
    ///     VerifyBlockTradeRequest, Trade, TradeRole, TradeDirection
    /// };
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = todo!(); // RestClient instance
    /// let request = VerifyBlockTradeRequest {
    ///     timestamp: 1672534800000,
    ///     nonce: "unique_nonce_123".to_string(),
    ///     role: TradeRole::Maker,
    ///     trades: vec![
    ///         Trade {
    ///             instrument_name: "BTC-PERPETUAL".to_string(),
    ///             price: 50000.0,
    ///             amount: Some(0.1),
    ///             direction: TradeDirection::Buy,
    ///         }
    ///     ],
    /// };
    /// 
    /// let response = client.verify_block_trade(&request).await?;
    /// println!("Block trade signature: {}", response.result.signature);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify_block_trade(
        &self,
        request: &VerifyBlockTradeRequest,
    ) -> Result<VerifyBlockTradeResponse, Errors> {
        // Check rate limits first
        self.rate_limiter
            .check_limits(EndpointType::MatchingEngine)
            .await?;

        // Prepare JSON-RPC request
        let id = self.next_request_id().await;
        let method = "private/verify_block_trade";
        
        let params = json!({
            "timestamp": request.timestamp,
            "nonce": request.nonce,
            "role": request.role,
            "trades": request.trades
        });

        // Send request
        let response = self.send_request(method, id, Some(params)).await?;

        // Record successful request
        self.rate_limiter
            .record_request(EndpointType::MatchingEngine)
            .await;

        // Parse response
        serde_json::from_value(response).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_role_serialization() {
        let maker = TradeRole::Maker;
        let taker = TradeRole::Taker;

        assert_eq!(serde_json::to_string(&maker).unwrap(), r#""maker""#);
        assert_eq!(serde_json::to_string(&taker).unwrap(), r#""taker""#);
    }

    #[test]
    fn test_trade_direction_serialization() {
        let buy = TradeDirection::Buy;
        let sell = TradeDirection::Sell;

        assert_eq!(serde_json::to_string(&buy).unwrap(), r#""buy""#);
        assert_eq!(serde_json::to_string(&sell).unwrap(), r#""sell""#);
    }

    #[test]
    fn test_trade_serialization() {
        let trade = Trade {
            instrument_name: "BTC-PERPETUAL".to_string(),
            price: 50000.0,
            amount: Some(0.1),
            direction: TradeDirection::Buy,
        };

        let serialized = serde_json::to_value(&trade).unwrap();
        assert_eq!(serialized["instrument_name"], "BTC-PERPETUAL");
        assert_eq!(serialized["price"], 50000.0);
        assert_eq!(serialized["amount"], 0.1);
        assert_eq!(serialized["direction"], "buy");
    }

    #[test]
    fn test_trade_without_amount() {
        let trade = Trade {
            instrument_name: "BTC-PERPETUAL".to_string(),
            price: 50000.0,
            amount: None,
            direction: TradeDirection::Sell,
        };

        let serialized = serde_json::to_value(&trade).unwrap();
        assert_eq!(serialized["instrument_name"], "BTC-PERPETUAL");
        assert_eq!(serialized["price"], 50000.0);
        assert_eq!(serialized["direction"], "sell");
        // amount should not be present when None
        assert!(!serialized.as_object().unwrap().contains_key("amount"));
    }

    #[test]
    fn test_verify_block_trade_request_serialization() {
        let request = VerifyBlockTradeRequest {
            timestamp: 1672534800000,
            nonce: "test_nonce".to_string(),
            role: TradeRole::Maker,
            trades: vec![
                Trade {
                    instrument_name: "BTC-PERPETUAL".to_string(),
                    price: 50000.0,
                    amount: Some(0.1),
                    direction: TradeDirection::Buy,
                },
                Trade {
                    instrument_name: "ETH-PERPETUAL".to_string(),
                    price: 3000.0,
                    amount: None,
                    direction: TradeDirection::Sell,
                },
            ],
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized["timestamp"], 1672534800000_i64);
        assert_eq!(serialized["nonce"], "test_nonce");
        assert_eq!(serialized["role"], "maker");
        
        let trades = serialized["trades"].as_array().unwrap();
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0]["instrument_name"], "BTC-PERPETUAL");
        assert_eq!(trades[1]["instrument_name"], "ETH-PERPETUAL");
    }

    #[test]
    fn test_verify_block_trade_response_deserialization() {
        let response_json = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": {
                "signature": "test_signature_abcdef123456"
            }
        });

        let response: VerifyBlockTradeResponse = 
            serde_json::from_value(response_json).unwrap();
        
        assert_eq!(response.id, serde_json::Value::Number(42.into()));
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.signature, "test_signature_abcdef123456");
    }

    #[test]
    fn test_verify_block_trade_response_with_string_id() {
        let response_json = json!({
            "id": "test-request-id",
            "jsonrpc": "2.0",
            "result": {
                "signature": "signature_xyz789"
            }
        });

        let response: VerifyBlockTradeResponse = 
            serde_json::from_value(response_json).unwrap();
        
        assert_eq!(response.id, serde_json::Value::String("test-request-id".to_string()));
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.signature, "signature_xyz789");
    }
}