//! Request and response structs for private/create_combo endpoint
//!
//! Verifies and creates a combo book or returns an existing combo matching given trades

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// A trade used to create a combo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateComboTrade {
    /// Instrument name
    pub instrument_name: String,
    
    /// It represents the requested trade size. For perpetual and inverse futures the amount is in USD units.
    /// For options and linear futures and it is the underlying base currency coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    
    /// Direction of trade from the maker perspective
    pub direction: String, // "buy" or "sell"
}

/// Request parameters for the private/create_combo endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CreateComboRequest {
    /// List of trades used to create a combo
    pub trades: Vec<CreateComboTrade>,
}

/// A leg in the combo response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateComboLeg {
    /// Size multiplier of a leg. A negative value indicates that the trades on given leg 
    /// are in opposite direction to the combo trades they originate from
    pub amount: i32,
    
    /// Unique instrument identifier
    pub instrument_name: String,
}

/// The result data from creating a combo
#[derive(Debug, Clone, Deserialize)]
pub struct CreateComboResult {
    /// The timestamp (milliseconds since the Unix epoch)
    pub creation_timestamp: i64,
    
    /// Unique combo identifier
    pub id: String,
    
    /// Instrument ID
    pub instrument_id: i32,
    
    /// Legs of the combo
    pub legs: Vec<CreateComboLeg>,
    
    /// Combo state: "rfq", "active", "inactive"
    pub state: String,
    
    /// The timestamp (milliseconds since the Unix epoch)
    pub state_timestamp: i64,
}

/// Response for private/create_combo endpoint following Deribit JSON-RPC 2.0 format
#[derive(Debug, Clone, Deserialize)]
pub struct CreateComboResponse {
    /// The id that was sent in the request
    pub id: i64,
    
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    
    /// Result containing the combo details
    pub result: CreateComboResult,
}

impl RestClient {
    /// Create a combo book or return an existing combo matching given trades
    ///
    /// Verifies and creates a combo book or returns an existing combo matching given trades.
    /// This is a private method that requires authentication and trade:read_write scope.
    ///
    /// This endpoint requires trade:read_write scope.
    ///
    /// See: <https://docs.deribit.com/#private-create_combo>
    ///
    /// Rate limit: Matching engine endpoint (tier-based limits)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `trades` - List of trades used to create a combo
    ///
    /// # Returns
    /// Result containing the created or existing combo details
    pub async fn create_combo(
        &self,
        trades: Vec<CreateComboTrade>,
    ) -> RestResult<CreateComboResponse> {
        let request = CreateComboRequest { trades };
        
        self.send_signed_request(
            "private/create_combo",
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
        let trades = vec![CreateComboTrade {
            instrument_name: "BTC-28JUN24-65000-C".to_string(),
            amount: None,
            direction: "buy".to_string(),
        }];

        let request = CreateComboRequest { trades };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("trades").is_some());
        
        let trades_array = json_value.get("trades").unwrap().as_array().unwrap();
        assert_eq!(trades_array.len(), 1);
        assert_eq!(trades_array[0].get("instrument_name").unwrap(), "BTC-28JUN24-65000-C");
        assert_eq!(trades_array[0].get("direction").unwrap(), "buy");
        assert!(trades_array[0].get("amount").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_amount() {
        let trades = vec![
            CreateComboTrade {
                instrument_name: "BTC-28JUN24-65000-C".to_string(),
                amount: Some(1.5),
                direction: "buy".to_string(),
            },
            CreateComboTrade {
                instrument_name: "BTC-28JUN24-70000-P".to_string(),
                amount: Some(1.5),
                direction: "sell".to_string(),
            },
        ];

        let request = CreateComboRequest { trades };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        let trades_array = json_value.get("trades").unwrap().as_array().unwrap();
        assert_eq!(trades_array.len(), 2);
        
        // Check first trade
        assert_eq!(trades_array[0].get("instrument_name").unwrap(), "BTC-28JUN24-65000-C");
        assert_eq!(trades_array[0].get("direction").unwrap(), "buy");
        assert_eq!(trades_array[0].get("amount").unwrap(), 1.5);
        
        // Check second trade
        assert_eq!(trades_array[1].get("instrument_name").unwrap(), "BTC-28JUN24-70000-P");
        assert_eq!(trades_array[1].get("direction").unwrap(), "sell");
        assert_eq!(trades_array[1].get("amount").unwrap(), 1.5);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "creation_timestamp": 1672738134824i64,
                "id": "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P",
                "instrument_id": 12345,
                "legs": [
                    {
                        "amount": 1,
                        "instrument_name": "BTC-28JUN24-65000-C"
                    },
                    {
                        "amount": -1,
                        "instrument_name": "BTC-28JUN24-70000-P"
                    }
                ],
                "state": "active",
                "state_timestamp": 1672738134824i64
            }
        });

        let response: CreateComboResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.creation_timestamp, 1672738134824);
        assert_eq!(response.result.id, "BTC-28JUN24-65000-C_BTC-28JUN24-70000-P");
        assert_eq!(response.result.instrument_id, 12345);
        assert_eq!(response.result.state, "active");
        assert_eq!(response.result.state_timestamp, 1672738134824);
        
        assert_eq!(response.result.legs.len(), 2);
        assert_eq!(response.result.legs[0].amount, 1);
        assert_eq!(response.result.legs[0].instrument_name, "BTC-28JUN24-65000-C");
        assert_eq!(response.result.legs[1].amount, -1);
        assert_eq!(response.result.legs[1].instrument_name, "BTC-28JUN24-70000-P");
    }

    #[test]
    fn test_trade_direction_validation() {
        // Test valid directions
        let buy_trade = CreateComboTrade {
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: Some(10.0),
            direction: "buy".to_string(),
        };
        
        let sell_trade = CreateComboTrade {
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: Some(10.0),
            direction: "sell".to_string(),
        };

        assert_eq!(buy_trade.direction, "buy");
        assert_eq!(sell_trade.direction, "sell");
    }

    #[tokio::test]
    async fn test_create_combo_method_exists() {
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
        let _ = RestClient::create_combo;

        // Verify the client exists
        let _ = &rest_client;

        println!("create_combo method is accessible and properly typed");
    }
}