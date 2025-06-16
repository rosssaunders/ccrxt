use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Request parameters for unstake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnstakeRequest {
    /// Staking instrument name, e.g. SOL.staked
    pub instrument_name: String,
    /// Unstake quantity
    pub quantity: String,
}

/// Unstake response information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnstakeResponse {
    /// Request id
    pub staking_id: String,
    /// Staking instrument name, e.g. SOL.staked
    pub instrument_name: String,
    /// Request status: NEW, PENDING, PENDING_WITHDRAWAL, PENDING_UNSTAKING, COMPLETED, REJECTED
    pub status: String,
    /// Unstake quantity
    pub quantity: String,
    /// Underlying instrument name, e.g. SOL
    pub underlying_inst_name: String,
    /// Reason for the status, e.g. "NO_ERROR"
    pub reason: String,
}

impl RestClient {
    /// Create a request to unlock staked token
    ///
    /// Creates an unstaking request for the specified instrument and quantity.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `instrument_name` - Staking instrument name, e.g. "SOL.staked"
    /// * `quantity` - Unstake quantity
    ///
    /// # Returns
    /// Unstake request information including staking ID, status, and reason
    pub async fn unstake(&self, instrument_name: &str, quantity: &str) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;

        let params = json!({
            "instrument_name": instrument_name,
            "quantity": quantity
        });

        self.send_signed_request("private/staking/unstake", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_unstake_request_structure() {
        let request = UnstakeRequest {
            instrument_name: "SOL.staked".to_string(),
            quantity: "1".to_string(),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "SOL.staked");
        assert_eq!(json_value.get("quantity").unwrap(), "1");
    }

    #[test]
    fn test_unstake_request_serialization() {
        let request = UnstakeRequest {
            instrument_name: "ETH.staked".to_string(),
            quantity: "0.25".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: UnstakeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.instrument_name, "ETH.staked");
        assert_eq!(deserialized.quantity, "0.25");
    }

    #[test]
    fn test_unstake_response_structure() {
        let response_json = json!({
            "staking_id": "1",
            "instrument_name": "SOL.staked",
            "status": "NEW",
            "quantity": "1",
            "underlying_inst_name": "SOL",
            "reason": "NO_ERROR"
        });

        let response: UnstakeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.staking_id, "1");
        assert_eq!(response.instrument_name, "SOL.staked");
        assert_eq!(response.status, "NEW");
        assert_eq!(response.quantity, "1");
        assert_eq!(response.underlying_inst_name, "SOL");
        assert_eq!(response.reason, "NO_ERROR");
    }

    #[test]
    fn test_unstake_response_different_statuses() {
        let statuses = vec![
            "NEW",
            "PENDING",
            "PENDING_WITHDRAWAL",
            "PENDING_UNSTAKING",
            "COMPLETED",
            "REJECTED",
        ];

        for status in statuses {
            let response_json = json!({
                "staking_id": "456",
                "instrument_name": "DYDX.staked",
                "status": status,
                "quantity": "50",
                "underlying_inst_name": "DYDX",
                "reason": "NO_ERROR"
            });

            let response: UnstakeResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.status, status);
        }
    }

    #[test]
    fn test_unstake_request_different_instruments() {
        let instruments = vec![
            ("SOL.staked", "2.5"),
            ("ETH.staked", "0.1"),
            ("TSTON.staked", "1000"),
            ("DOT.staked", "25.25"),
        ];

        for (instrument, qty) in instruments {
            let request = UnstakeRequest {
                instrument_name: instrument.to_string(),
                quantity: qty.to_string(),
            };

            let json_value = serde_json::to_value(request).unwrap();
            assert_eq!(json_value.get("instrument_name").unwrap(), instrument);
            assert_eq!(json_value.get("quantity").unwrap(), qty);
        }
    }

    #[test]
    fn test_unstake_response_yield_bearing_example() {
        // Test with TSTON.staked as mentioned in the documentation
        let response_json = json!({
            "staking_id": "789",
            "instrument_name": "TSTON.staked",
            "status": "PENDING_UNSTAKING",
            "quantity": "1000",
            "underlying_inst_name": "TON",
            "reason": "NO_ERROR"
        });

        let response: UnstakeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.instrument_name, "TSTON.staked");
        assert_eq!(response.underlying_inst_name, "TON");
        assert_eq!(response.status, "PENDING_UNSTAKING");
        assert_eq!(response.quantity, "1000");
    }
}
