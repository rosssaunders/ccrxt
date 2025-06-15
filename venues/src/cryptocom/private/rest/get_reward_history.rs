use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Request parameters for get reward history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRewardHistoryRequest {
    /// Staking instrument name, e.g. SOL.staked (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    /// Start time in Unix time format (inclusive) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time in Unix time format (inclusive) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// The maximum number of requests returned (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Reward history entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardHistoryEntry {
    /// Staking instrument name, e.g. SOL.staked
    pub staking_inst_name: String,
    /// Underlying instrument name, e.g. SOL
    pub underlying_inst_name: String,
    /// Reward instrument name, e.g. SOL.staked
    pub reward_inst_name: String,
    /// Reward quantity
    pub reward_quantity: String,
    /// Staked balance
    pub staked_balance: String,
    /// Event timestamp in milliseconds in Unix time format
    pub event_timestamp_ms: String,
}

/// Response for get reward history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRewardHistoryResponse {
    /// Array of reward history data
    pub data: Vec<RewardHistoryEntry>,
}

impl RestClient {
    /// Get staking reward history
    ///
    /// Returns historical staking rewards earned over time.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `instrument_name` - Optional staking instrument name to filter results, e.g. "SOL.staked"
    /// * `start_time` - Optional start time in Unix time format (inclusive)
    /// * `end_time` - Optional end time in Unix time format (inclusive)
    /// * `limit` - Optional maximum number of requests returned (Default: 20, Max: 500)
    ///
    /// # Returns
    /// Historical staking rewards with quantities, balances, and timestamps
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_reward_history(
        &self,
        instrument_name: Option<&str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<&str>,
    ) -> RestResult<Value> {
        let mut params = json!({});

        if let Some(instrument) = instrument_name {
            params["instrument_name"] = json!(instrument);
        }
        if let Some(start) = start_time {
            params["start_time"] = json!(start);
        }
        if let Some(end) = end_time {
            params["end_time"] = json!(end);
        }
        if let Some(lmt) = limit {
            params["limit"] = json!(lmt);
        }

        self.send_signed_request("private/staking/get-reward-history", params).await
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
    fn test_get_reward_history_request_empty() {
        let request = GetRewardHistoryRequest {
            instrument_name: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_get_reward_history_request_with_all_params() {
        let request = GetRewardHistoryRequest {
            instrument_name: Some("SOL.staked".to_string()),
            start_time: Some(1691455454495),
            end_time: Some(1691545277000),
            limit: Some("10".to_string()),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "SOL.staked");
        assert_eq!(json_value.get("start_time").unwrap(), 1691455454495_u64);
        assert_eq!(json_value.get("end_time").unwrap(), 1691545277000_u64);
        assert_eq!(json_value.get("limit").unwrap(), "10");
    }

    #[test]
    fn test_reward_history_entry_structure() {
        let entry_json = json!({
            "staking_inst_name": "SOL.staked",
            "underlying_inst_name": "SOL",
            "reward_inst_name": "SOL.staked",
            "reward_quantity": "123.4567",
            "staked_balance": "1234567",
            "event_timestamp_ms": "1667795832609"
        });

        let entry: RewardHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.staking_inst_name, "SOL.staked");
        assert_eq!(entry.underlying_inst_name, "SOL");
        assert_eq!(entry.reward_inst_name, "SOL.staked");
        assert_eq!(entry.reward_quantity, "123.4567");
        assert_eq!(entry.staked_balance, "1234567");
        assert_eq!(entry.event_timestamp_ms, "1667795832609");
    }

    #[test]
    fn test_get_reward_history_response_structure() {
        let response_json = json!({
            "data": [
                {
                    "staking_inst_name": "SOL.staked",
                    "underlying_inst_name": "SOL",
                    "reward_inst_name": "SOL.staked",
                    "reward_quantity": "123.4567",
                    "staked_balance": "1234567",
                    "event_timestamp_ms": "1667795832609"
                }
            ]
        });

        let response: GetRewardHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(
            response.data.first().unwrap().staking_inst_name,
            "SOL.staked"
        );
        assert_eq!(response.data.first().unwrap().reward_quantity, "123.4567");
    }

    #[test]
    fn test_reward_history_multiple_entries() {
        let response_json = json!({
            "data": [
                {
                    "staking_inst_name": "SOL.staked",
                    "underlying_inst_name": "SOL",
                    "reward_inst_name": "SOL.staked",
                    "reward_quantity": "50.0",
                    "staked_balance": "1000.0",
                    "event_timestamp_ms": "1667795832609"
                },
                {
                    "staking_inst_name": "ETH.staked",
                    "underlying_inst_name": "ETH",
                    "reward_inst_name": "ETH.staked",
                    "reward_quantity": "0.125",
                    "staked_balance": "5.0",
                    "event_timestamp_ms": "1667795832610"
                }
            ]
        });

        let response: GetRewardHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(
            response.data.first().unwrap().staking_inst_name,
            "SOL.staked"
        );
        assert_eq!(
            response.data.get(1).unwrap().staking_inst_name,
            "ETH.staked"
        );
        assert_eq!(response.data.first().unwrap().reward_quantity, "50.0");
        assert_eq!(response.data.get(1).unwrap().reward_quantity, "0.125");
    }

    #[test]
    fn test_reward_history_different_reward_instruments() {
        // Test case where reward instrument differs from staking instrument
        let entry_json = json!({
            "staking_inst_name": "DYDX.staked",
            "underlying_inst_name": "DYDX",
            "reward_inst_name": "USD_Stable_Coin",
            "reward_quantity": "25.50",
            "staked_balance": "1000.0",
            "event_timestamp_ms": "1667795832609"
        });

        let entry: RewardHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.staking_inst_name, "DYDX.staked");
        assert_eq!(entry.underlying_inst_name, "DYDX");
        assert_eq!(entry.reward_inst_name, "USD_Stable_Coin");
        assert_eq!(entry.reward_quantity, "25.50");
    }

    #[test]
    fn test_reward_history_request_serialization() {
        let request = GetRewardHistoryRequest {
            instrument_name: Some("ETH.staked".to_string()),
            start_time: Some(1691455454495),
            end_time: None,
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetRewardHistoryRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.instrument_name, Some("ETH.staked".to_string()));
        assert_eq!(deserialized.start_time, Some(1691455454495));
        assert_eq!(deserialized.end_time, None);
        assert_eq!(deserialized.limit, Some("100".to_string()));
    }
}
