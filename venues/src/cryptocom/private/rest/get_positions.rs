use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Account ID
    pub account_id: String,
    /// Position quantity
    pub quantity: String,
    /// Position cost or value in USD
    pub cost: String,
    /// Profit and loss for the open position
    pub open_position_pnl: String,
    /// Open position cost
    pub open_pos_cost: String,
    /// Profit and loss in the current trading session
    pub session_pnl: String,
    /// Updated time (Unix timestamp)
    pub update_timestamp_ms: u64,
    /// e.g. BTCUSD-PERP
    pub instrument_name: String,
    /// e.g. Perpetual Swap
    #[serde(rename = "type")]
    pub position_type: String,
}

/// Response for get positions endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionsResponse {
    /// Array of position data
    pub data: Vec<Position>,
}

/// Request parameters for get positions endpoint
#[derive(Debug, Clone, Serialize)]
pub struct GetPositionsRequest {
    /// e.g. BTCUSD-PERP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
}

impl RestClient {
    /// Get user's positions with optional instrument filtering
    ///
    /// Returns the user's positions with optional filtering by instrument.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-get-positions>
    ///
    /// Rate limit: No rate limit
    ///
    /// # Arguments
    /// * `instrument_name` - Optional instrument name filter (e.g. BTCUSD-PERP)
    ///
    /// # Returns
    /// Position information for all or specified instruments
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_positions(&self, instrument_name: Option<&str>) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;

        let mut params = json!({});
        if let Some(instrument) = instrument_name {
            params["instrument_name"] = Value::String(instrument.to_string());
        }

        let signature = self.sign_request("private/get-positions", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/get-positions",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(&format!("{}/v1/private/get-positions", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_position_structure() {
        let position_json = json!({
            "account_id": "858dbc8b-22fd-49fa-bff4-d342d98a8acb",
            "quantity": "-0.1984",
            "cost": "-10159.573500",
            "open_position_pnl": "-497.743736",
            "open_pos_cost": "-10159.352200",
            "session_pnl": "2.236145",
            "update_timestamp_ms": 1613552240770_u64,
            "instrument_name": "BTCUSD-PERP",
            "type": "PERPETUAL_SWAP"
        });

        let position: Position = serde_json::from_value(position_json).unwrap();
        assert_eq!(position.account_id, "858dbc8b-22fd-49fa-bff4-d342d98a8acb");
        assert_eq!(position.instrument_name, "BTCUSD-PERP");
        assert_eq!(position.position_type, "PERPETUAL_SWAP");
        assert_eq!(position.quantity, "-0.1984");
        assert_eq!(position.cost, "-10159.573500");
        assert_eq!(position.open_position_pnl, "-497.743736");
        assert_eq!(position.session_pnl, "2.236145");
        assert_eq!(position.update_timestamp_ms, 1613552240770_u64);
    }

    #[test]
    fn test_position_positive_quantity() {
        let position_json = json!({
            "account_id": "test-account-id",
            "quantity": "1.5000",
            "cost": "45000.0000",
            "open_position_pnl": "1500.0000",
            "open_pos_cost": "43500.0000",
            "session_pnl": "500.0000",
            "update_timestamp_ms": 1640995200000_u64,
            "instrument_name": "ETHUSD-PERP",
            "type": "PERPETUAL_SWAP"
        });

        let position: Position = serde_json::from_value(position_json).unwrap();
        assert_eq!(position.quantity, "1.5000");
        assert_eq!(position.instrument_name, "ETHUSD-PERP");
        assert_eq!(position.open_position_pnl, "1500.0000");
    }

    #[test]
    fn test_get_positions_response_structure() {
        let response_json = json!({
            "data": [
                {
                    "account_id": "account-1",
                    "quantity": "0.5000",
                    "cost": "25000.0000",
                    "open_position_pnl": "500.0000",
                    "open_pos_cost": "24500.0000",
                    "session_pnl": "100.0000",
                    "update_timestamp_ms": 1640995200000_u64,
                    "instrument_name": "BTCUSD-PERP",
                    "type": "PERPETUAL_SWAP"
                },
                {
                    "account_id": "account-2",
                    "quantity": "-1.0000",
                    "cost": "-3000.0000",
                    "open_position_pnl": "-50.0000",
                    "open_pos_cost": "-2950.0000",
                    "session_pnl": "-25.0000",
                    "update_timestamp_ms": 1640995300000_u64,
                    "instrument_name": "ETHUSD-PERP",
                    "type": "PERPETUAL_SWAP"
                }
            ]
        });

        let response: GetPositionsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].account_id, "account-1");
        assert_eq!(response.data[0].instrument_name, "BTCUSD-PERP");
        assert_eq!(response.data[0].quantity, "0.5000");
        assert_eq!(response.data[1].account_id, "account-2");
        assert_eq!(response.data[1].instrument_name, "ETHUSD-PERP");
        assert_eq!(response.data[1].quantity, "-1.0000");
    }

    #[test]
    fn test_get_positions_request_with_instrument() {
        let request = GetPositionsRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
    }

    #[test]
    fn test_get_positions_request_without_instrument() {
        let request = GetPositionsRequest {
            instrument_name: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_get_positions_request_different_instruments() {
        let btc_request = GetPositionsRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
        };

        let json_value = serde_json::to_value(btc_request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");

        let eth_request = GetPositionsRequest {
            instrument_name: Some("ETHUSD-PERP".to_string()),
        };

        let json_value = serde_json::to_value(eth_request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "ETHUSD-PERP");
    }

    #[test]
    fn test_position_type_field_mapping() {
        // Test that the "type" field is properly mapped to position_type
        let position_json = json!({
            "account_id": "test-account",
            "quantity": "1.0000",
            "cost": "50000.0000",
            "open_position_pnl": "0.0000",
            "open_pos_cost": "50000.0000",
            "session_pnl": "0.0000",
            "update_timestamp_ms": 1640995200000_u64,
            "instrument_name": "BTCUSD-PERP",
            "type": "FUTURES"
        });

        let position: Position = serde_json::from_value(position_json).unwrap();
        assert_eq!(position.position_type, "FUTURES");

        // Test serialization preserves the rename
        let serialized = serde_json::to_value(&position).unwrap();
        assert_eq!(serialized.get("type").unwrap(), "FUTURES");
        assert!(!serialized
            .as_object()
            .unwrap()
            .contains_key("position_type"));
    }
}
