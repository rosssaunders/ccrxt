use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const SIMULATE_BLOCK_TRADE_ENDPOINT: &str = "private/simulate_block_trade";

/// Role enum for simulate block trade requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Maker role
    Maker,
    /// Taker role
    Taker,
}

/// Direction enum for trade direction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// Buy direction
    Buy,
    /// Sell direction
    Sell,
}

/// Trade data for a single trade in the block trade
#[derive(Debug, Clone, Serialize)]
pub struct Trade {
    /// Instrument name
    pub instrument_name: String,

    /// Price for trade
    pub price: f64,

    /// Trade size (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// Direction of trade from the maker perspective
    pub direction: Direction,
}

/// Request parameters for simulate block trade
#[derive(Debug, Clone, Serialize)]
pub struct SimulateBlockTradeRequest {
    /// Describes if user wants to be maker or taker of trades (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,

    /// List of trades for block trade
    pub trades: Vec<Trade>,
}

/// Response for simulate block trade endpoint
pub type SimulateBlockTradeResponse = JsonRpcResult<bool>;

impl RestClient {
    /// Simulate a block trade to check if it can be executed.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-simulate_block_trade)
    ///
    /// Rate limit: Matching engine endpoint (tier-based limits)
    /// Scope: block_trade:read
    ///
    /// # Arguments
    /// * `params` - Parameters for the simulate block trade request
    ///
    /// # Returns
    /// Result with boolean indicating if block trade can be executed
    pub async fn simulate_block_trade(
        &self,
        params: SimulateBlockTradeRequest,
    ) -> RestResult<SimulateBlockTradeResponse> {
        self.send_signed_request(
            SIMULATE_BLOCK_TRADE_ENDPOINT,
            &params,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::ExposableSecret;
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;
    use crate::deribit::private::rest::credentials::Credentials;
    use rest::secrets::SecretString;

    #[test]
    fn test_role_serialization() {
        let maker = Role::Maker;
        let taker = Role::Taker;

        let maker_json = serde_json::to_string(&maker).unwrap();
        let taker_json = serde_json::to_string(&taker).unwrap();

        assert_eq!(maker_json, "\"maker\"");
        assert_eq!(taker_json, "\"taker\"");
    }

    #[test]
    fn test_role_deserialization() {
        let maker: Role = serde_json::from_str("\"maker\"").unwrap();
        let taker: Role = serde_json::from_str("\"taker\"").unwrap();

        matches!(maker, Role::Maker);
        matches!(taker, Role::Taker);
    }

    #[test]
    fn test_direction_serialization() {
        let buy = Direction::Buy;
        let sell = Direction::Sell;

        let buy_json = serde_json::to_string(&buy).unwrap();
        let sell_json = serde_json::to_string(&sell).unwrap();

        assert_eq!(buy_json, "\"buy\"");
        assert_eq!(sell_json, "\"sell\"");
    }

    #[test]
    fn test_direction_deserialization() {
        let buy: Direction = serde_json::from_str("\"buy\"").unwrap();
        let sell: Direction = serde_json::from_str("\"sell\"").unwrap();

        matches!(buy, Direction::Buy);
        matches!(sell, Direction::Sell);
    }

    #[test]
    fn test_trade_serialization_minimal() {
        let trade = Trade {
            instrument_name: "BTCUSD-PERP".to_string(),
            price: 50000.0,
            amount: None,
            direction: Direction::Buy,
        };

        let json_str = serde_json::to_string(&trade).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(json_value.get("price").unwrap(), 50000.0);
        assert!(json_value.get("amount").is_none());
        assert_eq!(json_value.get("direction").unwrap(), "buy");
    }

    #[test]
    fn test_trade_serialization_full() {
        let trade = Trade {
            instrument_name: "ETHUSD-PERP".to_string(),
            price: 3000.0,
            amount: Some(1.5),
            direction: Direction::Sell,
        };

        let json_str = serde_json::to_string(&trade).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "ETHUSD-PERP");
        assert_eq!(json_value.get("price").unwrap(), 3000.0);
        assert_eq!(json_value.get("amount").unwrap(), 1.5);
        assert_eq!(json_value.get("direction").unwrap(), "sell");
    }

    #[test]
    fn test_request_serialization_minimal() {
        let trades = vec![Trade {
            instrument_name: "BTCUSD-PERP".to_string(),
            price: 50000.0,
            amount: None,
            direction: Direction::Buy,
        }];

        let request = SimulateBlockTradeRequest { role: None, trades };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("role").is_none());
        assert!(json_value.get("trades").unwrap().is_array());
        assert_eq!(
            json_value.get("trades").unwrap().as_array().unwrap().len(),
            1
        );
    }

    #[test]
    fn test_request_serialization_full() {
        let trades = vec![
            Trade {
                instrument_name: "BTCUSD-PERP".to_string(),
                price: 50000.0,
                amount: Some(0.1),
                direction: Direction::Buy,
            },
            Trade {
                instrument_name: "ETHUSD-PERP".to_string(),
                price: 3000.0,
                amount: Some(1.0),
                direction: Direction::Sell,
            },
        ];

        let request = SimulateBlockTradeRequest {
            role: Some(Role::Maker),
            trades,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("role").unwrap(), "maker");
        assert!(json_value.get("trades").unwrap().is_array());
        assert_eq!(
            json_value.get("trades").unwrap().as_array().unwrap().len(),
            2
        );
    }

    #[test]
    fn test_response_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": true
        });

        let response: SimulateBlockTradeResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result);
    }

    #[test]
    fn test_response_deserialization_false() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": false
        });

        let response: SimulateBlockTradeResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert!(!response.result);
    }

    #[tokio::test]
    async fn test_simulate_block_trade_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::simulate_block_trade;

        // Verify the client exists
        let _ = &rest_client;

        println!("simulate_block_trade method is accessible and properly typed");
    }
}
