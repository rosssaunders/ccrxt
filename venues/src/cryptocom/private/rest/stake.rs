use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Request parameters for stake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeRequest {
    /// Staking instrument name, e.g. SOL.staked
    pub instrument_name: String,
    /// Stake quantity
    pub quantity: String,
}

/// Stake response information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeResponse {
    /// Request id
    pub staking_id: String,
    /// Staking instrument name, e.g. SOL.staked
    pub instrument_name: String,
    /// Request status: NEW, PENDING, STAKED, COMPLETED, REJECTED
    pub status: String,
    /// Stake quantity
    pub quantity: String,
    /// Underlying instrument name of staking, e.g. SOL
    pub underlying_inst_name: String,
    /// Pre stake charge rate in basis point
    pub pre_stake_charge_rate_in_bps: String,
    /// Pre stake charge value
    pub pre_stake_charge: String,
    /// Reason for the status, e.g. "NO_ERROR"
    pub reason: String,
}

impl RestClient {
    /// Create a request to earn token rewards by staking on-chain in the Exchange
    ///
    /// Creates a staking request for the specified instrument and quantity.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `instrument_name` - Staking instrument name, e.g. "SOL.staked"
    /// * `quantity` - Stake quantity
    ///
    /// # Returns
    /// Stake request information including staking ID, status, and charge details
    pub async fn stake(&self, instrument_name: &str, quantity: &str) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;

        let params = json!({
            "instrument_name": instrument_name,
            "quantity": quantity
        });

        self.send_signed_request("private/staking/stake", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;

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
    fn test_stake_request_structure() {
        let request = StakeRequest {
            instrument_name: "SOL.staked".to_string(),
            quantity: "1".to_string(),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "SOL.staked");
        assert_eq!(json_value.get("quantity").unwrap(), "1");
    }

    #[test]
    fn test_stake_request_serialization() {
        let request = StakeRequest {
            instrument_name: "ETH.staked".to_string(),
            quantity: "0.5".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: StakeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.instrument_name, "ETH.staked");
        assert_eq!(deserialized.quantity, "0.5");
    }

    #[test]
    fn test_stake_response_structure() {
        let response_json = json!({
            "staking_id": "1",
            "instrument_name": "SOL.staked",
            "status": "NEW",
            "quantity": "1",
            "underlying_inst_name": "SOL",
            "pre_stake_charge_rate_in_bps": "50",
            "pre_stake_charge": "0.5",
            "reason": "NO_ERROR"
        });

        let response: StakeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.staking_id, "1");
        assert_eq!(response.instrument_name, "SOL.staked");
        assert_eq!(response.status, "NEW");
        assert_eq!(response.quantity, "1");
        assert_eq!(response.underlying_inst_name, "SOL");
        assert_eq!(response.pre_stake_charge_rate_in_bps, "50");
        assert_eq!(response.pre_stake_charge, "0.5");
        assert_eq!(response.reason, "NO_ERROR");
    }

    #[test]
    fn test_stake_response_different_statuses() {
        let statuses = vec!["NEW", "PENDING", "STAKED", "COMPLETED", "REJECTED"];

        for status in statuses {
            let response_json = json!({
                "staking_id": "123",
                "instrument_name": "DYDX.staked",
                "status": status,
                "quantity": "100",
                "underlying_inst_name": "DYDX",
                "pre_stake_charge_rate_in_bps": "0",
                "pre_stake_charge": "0",
                "reason": "NO_ERROR"
            });

            let response: StakeResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.status, status);
        }
    }

    #[test]
    fn test_stake_request_different_instruments() {
        let instruments = vec![
            ("SOL.staked", "1.5"),
            ("ETH.staked", "0.25"),
            ("DYDX.staked", "100"),
            ("DOT.staked", "50.75"),
        ];

        for (instrument, qty) in instruments {
            let request = StakeRequest {
                instrument_name: instrument.to_string(),
                quantity: qty.to_string(),
            };

            let json_value = serde_json::to_value(request).unwrap();
            assert_eq!(json_value.get("instrument_name").unwrap(), instrument);
            assert_eq!(json_value.get("quantity").unwrap(), qty);
        }
    }
}
