use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};

/// Trade data for position move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovePositionTrade {
    /// Instrument name
    pub instrument_name: String,
    /// Price for trade - if not provided average price of the position is used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// It represents the requested trade size. For perpetual and inverse futures the amount is in USD units.
    /// For options and linear futures and it is the underlying base currency coin. Amount can't exceed position size.
    pub amount: f64,
}

/// Request parameters for moving positions between subaccounts
#[derive(Debug, Clone, Serialize)]
pub struct MovePositionsRequest {
    /// The currency symbol (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Id of source subaccount. Can be found in My Account >> Subaccounts tab
    pub source_uid: i32,
    /// Id of target subaccount. Can be found in My Account >> Subaccounts tab
    pub target_uid: i32,
    /// List of trades for position move
    pub trades: Vec<MovePositionTrade>,
}

/// Trade result data in response
#[derive(Debug, Clone, Deserialize)]
pub struct MovePositionTradeResult {
    /// Trade amount. For perpetual and inverse futures the amount is in USD units.
    /// For options and linear futures and it is the underlying base currency coin.
    pub amount: f64,
    /// Direction: buy or sell
    pub direction: String,
    /// Unique instrument identifier
    pub instrument_name: String,
    /// Price in base currency
    pub price: f64,
    /// Trade source uid
    pub source_uid: i32,
    /// Trade target uid
    pub target_uid: i32,
}

/// Response for move positions endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct MovePositionsResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result containing the trades
    pub result: MovePositionsResult,
}

/// Result data for move positions response
#[derive(Debug, Clone, Deserialize)]
pub struct MovePositionsResult {
    /// Array of trade results
    pub trades: Vec<MovePositionTradeResult>,
}

impl RestClient {
    /// Moves positions from source subaccount to target subaccount
    ///
    /// In rare cases, the request may return an internal_server_error. This does not
    /// necessarily mean the operation failed entirely. Part or all of the position
    /// transfer might have still been processed successfully.
    ///
    /// See: <https://docs.deribit.com/v2/#private-move_positions>
    ///
    /// Rate limit: Matching engine method (varies by account tier)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `params` - Parameters for the position move (source_uid, target_uid, trades, optional currency)
    ///
    /// # Returns
    /// Move positions result with trade details
    pub async fn move_positions(&self, params: MovePositionsRequest) -> RestResult<MovePositionsResponse> {
        self.send_signed_request(
            "private/move_positions",
            &params,
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
    fn test_move_position_trade_serialization() {
        let trade = MovePositionTrade {
            instrument_name: "BTC-PERPETUAL".to_string(),
            price: Some(50000.0),
            amount: 100.0,
        };

        let json_str = serde_json::to_string(&trade).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "BTC-PERPETUAL");
        assert_eq!(json_value.get("price").unwrap(), 50000.0);
        assert_eq!(json_value.get("amount").unwrap(), 100.0);
    }

    #[test]
    fn test_move_position_trade_without_price() {
        let trade = MovePositionTrade {
            instrument_name: "ETH-PERPETUAL".to_string(),
            price: None,
            amount: 200.0,
        };

        let json_str = serde_json::to_string(&trade).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "ETH-PERPETUAL");
        assert!(json_value.get("price").is_none()); // Should be omitted when None
        assert_eq!(json_value.get("amount").unwrap(), 200.0);
    }

    #[test]
    fn test_request_parameters_serialization() {
        let request = MovePositionsRequest {
            currency: Some("BTC".to_string()),
            source_uid: 12345,
            target_uid: 67890,
            trades: vec![
                MovePositionTrade {
                    instrument_name: "BTC-PERPETUAL".to_string(),
                    price: Some(50000.0),
                    amount: 100.0,
                },
                MovePositionTrade {
                    instrument_name: "ETH-PERPETUAL".to_string(),
                    price: None,
                    amount: 200.0,
                },
            ],
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("source_uid").unwrap(), 12345);
        assert_eq!(json_value.get("target_uid").unwrap(), 67890);

        let trades = json_value.get("trades").unwrap().as_array().unwrap();
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].get("instrument_name").unwrap(), "BTC-PERPETUAL");
        assert_eq!(trades[1].get("instrument_name").unwrap(), "ETH-PERPETUAL");
    }

    #[test]
    fn test_request_parameters_without_currency() {
        let request = MovePositionsRequest {
            currency: None,
            source_uid: 12345,
            target_uid: 67890,
            trades: vec![MovePositionTrade {
                instrument_name: "BTC-PERPETUAL".to_string(),
                price: Some(50000.0),
                amount: 100.0,
            }],
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("currency").is_none()); // Should be omitted when None
        assert_eq!(json_value.get("source_uid").unwrap(), 12345);
        assert_eq!(json_value.get("target_uid").unwrap(), 67890);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "trades": [
                    {
                        "amount": 100.0,
                        "direction": "buy",
                        "instrument_name": "BTC-PERPETUAL",
                        "price": 50000.0,
                        "source_uid": 12345,
                        "target_uid": 67890
                    },
                    {
                        "amount": 200.0,
                        "direction": "sell",
                        "instrument_name": "ETH-PERPETUAL",
                        "price": 3000.0,
                        "source_uid": 12345,
                        "target_uid": 67890
                    }
                ]
            }
        });

        let response: MovePositionsResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.trades.len(), 2);

        let first_trade = &response.result.trades[0];
        assert_eq!(first_trade.amount, 100.0);
        assert_eq!(first_trade.direction, "buy");
        assert_eq!(first_trade.instrument_name, "BTC-PERPETUAL");
        assert_eq!(first_trade.price, 50000.0);
        assert_eq!(first_trade.source_uid, 12345);
        assert_eq!(first_trade.target_uid, 67890);

        let second_trade = &response.result.trades[1];
        assert_eq!(second_trade.amount, 200.0);
        assert_eq!(second_trade.direction, "sell");
        assert_eq!(second_trade.instrument_name, "ETH-PERPETUAL");
    }

    #[tokio::test]
    async fn test_move_positions_method_exists() {
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
        let _ = RestClient::move_positions;

        // Verify the client exists
        let _ = &rest_client;

        println!("move_positions method is accessible and properly typed");
    }

    #[test]
    fn test_all_supported_currencies() {
        // Test that all supported currencies work in serialization
        let currencies = ["BTC", "ETH", "USDC", "USDT", "EURR"];

        for currency in currencies {
            let request = MovePositionsRequest {
                currency: Some(currency.to_string()),
                source_uid: 12345,
                target_uid: 67890,
                trades: vec![MovePositionTrade {
                    instrument_name: format!("{}-PERPETUAL", currency),
                    price: Some(1000.0),
                    amount: 100.0,
                }],
            };

            let json_str = serde_json::to_string(&request).unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();

            assert_eq!(json_value.get("currency").unwrap(), currency);
            println!("Currency {} serializes correctly", currency);
        }
    }

    #[test]
    fn test_trade_directions() {
        // Test that both buy and sell directions work in deserialization
        let directions = ["buy", "sell"];

        for direction in directions {
            let trade_json = json!({
                "amount": 100.0,
                "direction": direction,
                "instrument_name": "BTC-PERPETUAL",
                "price": 50000.0,
                "source_uid": 12345,
                "target_uid": 67890
            });

            let trade: MovePositionTradeResult = serde_json::from_value(trade_json).unwrap();
            assert_eq!(trade.direction, direction);
            println!("Direction {} deserializes correctly", direction);
        }
    }
}
