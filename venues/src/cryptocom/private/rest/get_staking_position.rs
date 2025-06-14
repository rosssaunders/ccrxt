use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Request parameters for get staking position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStakingPositionRequest {
    /// Staking instrument name, e.g. SOL.staked (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
}

/// Staking position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingPosition {
    /// Staking instrument name, e.g. SOL.staked
    pub instrument_name: String,
    /// Underlying instrument name, e.g. SOL
    pub underlying_inst_name: String,
    /// Total staked quantity
    pub staked_quantity: String,
    /// Total pending staked quantity
    pub pending_staked_quantity: String,
    /// Total pending unstaked quantity
    pub pending_unstaked_quantity: String,
    /// Total reward eligible quantity, quantity can be unstaked/convert
    pub reward_eligible_quantity: String,
}

/// Response for get staking position endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStakingPositionResponse {
    /// Array of staking position data
    pub data: Vec<StakingPosition>,
}

impl RestClient {
    /// Get the total staking position for a user/token
    ///
    /// Returns the user's staking positions including staked, pending, and reward eligible quantities.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `instrument_name` - Optional staking instrument name to filter results, e.g. "SOL.staked"
    ///
    /// # Returns
    /// Staking position information including quantities and underlying instrument details
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_staking_position(&self, instrument_name: Option<&str>) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;

        let params = if let Some(instrument) = instrument_name {
            json!({
                "instrument_name": instrument
            })
        } else {
            json!({})
        };

        let signature =
            self.sign_request("private/staking/get-staking-position", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/staking/get-staking-position",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(format!(
                "{}/v1/private/staking/get-staking-position",
                self.base_url
            ))
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
    fn test_get_staking_position_request_without_instrument() {
        let request = GetStakingPositionRequest {
            instrument_name: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_get_staking_position_request_with_instrument() {
        let request = GetStakingPositionRequest {
            instrument_name: Some("SOL.staked".to_string()),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "SOL.staked");
    }

    #[test]
    fn test_staking_position_structure() {
        let position_json = json!({
            "instrument_name": "SOL.staked",
            "underlying_inst_name": "SOL",
            "staked_quantity": "30000.00",
            "pending_staked_quantity": "20000.00",
            "pending_unstaked_quantity": "10000.00",
            "reward_eligible_quantity": "10000.00"
        });

        let position: StakingPosition = serde_json::from_value(position_json).unwrap();
        assert_eq!(position.instrument_name, "SOL.staked");
        assert_eq!(position.underlying_inst_name, "SOL");
        assert_eq!(position.staked_quantity, "30000.00");
        assert_eq!(position.pending_staked_quantity, "20000.00");
        assert_eq!(position.pending_unstaked_quantity, "10000.00");
        assert_eq!(position.reward_eligible_quantity, "10000.00");
    }

    #[test]
    fn test_get_staking_position_response_structure() {
        let response_json = json!({
            "data": [
                {
                    "instrument_name": "SOL.staked",
                    "underlying_inst_name": "SOL",
                    "staked_quantity": "30000.00",
                    "pending_staked_quantity": "20000.00",
                    "pending_unstaked_quantity": "10000.00",
                    "reward_eligible_quantity": "10000.00"
                }
            ]
        });

        let response: GetStakingPositionResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().instrument_name, "SOL.staked");
        assert_eq!(response.data.first().unwrap().underlying_inst_name, "SOL");
    }

    #[test]
    fn test_get_staking_position_response_multiple_positions() {
        let response_json = json!({
            "data": [
                {
                    "instrument_name": "SOL.staked",
                    "underlying_inst_name": "SOL",
                    "staked_quantity": "30000.00",
                    "pending_staked_quantity": "20000.00",
                    "pending_unstaked_quantity": "10000.00",
                    "reward_eligible_quantity": "10000.00"
                },
                {
                    "instrument_name": "ETH.staked",
                    "underlying_inst_name": "ETH",
                    "staked_quantity": "5.50",
                    "pending_staked_quantity": "2.25",
                    "pending_unstaked_quantity": "1.00",
                    "reward_eligible_quantity": "4.50"
                }
            ]
        });

        let response: GetStakingPositionResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().instrument_name, "SOL.staked");
        assert_eq!(response.data.get(1).unwrap().instrument_name, "ETH.staked");
    }
}
