use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Request parameters for get stake history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStakeHistoryRequest {
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

/// Stake history entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeHistoryEntry {
    /// Staking instrument name, e.g. SOL.staked
    pub instrument_name: String,
    /// Underlying instrument name, e.g. SOL
    pub underlying_inst_name: String,
    /// Cycle id
    pub cycle_id: String,
    /// Request id
    pub staking_id: String,
    /// Request status: COMPLETED, REJECTED
    pub status: String,
    /// Account id
    pub account: String,
    /// Stake/unstake quantity
    pub quantity: String,
    /// Stake or Unstake
    pub side: String,
    /// Request creation timestamp in milliseconds in Unix time format
    pub create_timestamp_ms: String,
}

/// Response for get stake history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStakeHistoryResponse {
    /// Array of stake history data
    pub data: Vec<StakeHistoryEntry>,
}

impl RestClient {
    /// Get stake/unstake request history
    ///
    /// Returns historical stake and unstake requests that have been completed or rejected.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `request` - Parameters for retrieving stake history
    ///
    /// # Returns
    /// Historical stake/unstake requests with final status and timestamps
    pub async fn get_stake_history(
        &self,
        request: GetStakeHistoryRequest,
    ) -> RestResult<GetStakeHistoryResponse> {
        self.send_signed_request("private/staking/get-stake-history", request)
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
    fn test_get_stake_history_request_empty() {
        let request = GetStakeHistoryRequest {
            instrument_name: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_get_stake_history_request_with_all_params() {
        let request = GetStakeHistoryRequest {
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
    fn test_stake_history_entry_completed() {
        let entry_json = json!({
            "instrument_name": "SOL.staked",
            "underlying_inst_name": "SOL",
            "cycle_id": "1",
            "staking_id": "1",
            "status": "COMPLETED",
            "account": "12345678-9999-1234-9999-123456789999",
            "quantity": "1",
            "side": "STAKE",
            "create_timestamp_ms": "1668658093600"
        });

        let entry: StakeHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.instrument_name, "SOL.staked");
        assert_eq!(entry.underlying_inst_name, "SOL");
        assert_eq!(entry.cycle_id, "1");
        assert_eq!(entry.staking_id, "1");
        assert_eq!(entry.status, "COMPLETED");
        assert_eq!(entry.account, "12345678-9999-1234-9999-123456789999");
        assert_eq!(entry.quantity, "1");
        assert_eq!(entry.side, "STAKE");
        assert_eq!(entry.create_timestamp_ms, "1668658093600");
    }

    #[test]
    fn test_stake_history_entry_rejected() {
        let entry_json = json!({
            "instrument_name": "SOL.staked",
            "underlying_inst_name": "SOL",
            "cycle_id": "2",
            "staking_id": "2",
            "status": "REJECTED",
            "account": "12345678-9999-1234-9999-123456789999",
            "quantity": "0.5",
            "side": "UNSTAKE",
            "create_timestamp_ms": "1668658093600"
        });

        let entry: StakeHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.side, "UNSTAKE");
        assert_eq!(entry.status, "REJECTED");
        assert_eq!(entry.quantity, "0.5");
    }

    #[test]
    fn test_get_stake_history_response_structure() {
        let response_json = json!({
            "data": [
                {
                    "instrument_name": "SOL.staked",
                    "underlying_inst_name": "SOL",
                    "cycle_id": "1",
                    "staking_id": "1",
                    "status": "COMPLETED",
                    "account": "12345678-9999-1234-9999-123456789999",
                    "quantity": "1",
                    "side": "STAKE",
                    "create_timestamp_ms": "1668658093600"
                },
                {
                    "instrument_name": "SOL.staked",
                    "underlying_inst_name": "SOL",
                    "cycle_id": "2",
                    "staking_id": "2",
                    "status": "REJECTED",
                    "account": "12345678-9999-1234-9999-123456789999",
                    "quantity": "0.5",
                    "side": "UNSTAKE",
                    "create_timestamp_ms": "1668658093600"
                }
            ]
        });

        let response: GetStakeHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().status, "COMPLETED");
        assert_eq!(response.data.get(1).unwrap().status, "REJECTED");
        assert_eq!(response.data.first().unwrap().side, "STAKE");
        assert_eq!(response.data.get(1).unwrap().side, "UNSTAKE");
    }

    #[test]
    fn test_stake_history_final_statuses() {
        let statuses = vec!["COMPLETED", "REJECTED"];

        for status in statuses {
            let entry_json = json!({
                "instrument_name": "DYDX.staked",
                "underlying_inst_name": "DYDX",
                "cycle_id": "3",
                "staking_id": "3",
                "status": status,
                "account": "test-account",
                "quantity": "100",
                "side": "STAKE",
                "create_timestamp_ms": "1668658093600"
            });

            let entry: StakeHistoryEntry = serde_json::from_value(entry_json).unwrap();
            assert_eq!(entry.status, status);
        }
    }

    #[test]
    fn test_stake_history_yield_bearing_example() {
        // Test with TSTON.staked as mentioned in the documentation
        let entry_json = json!({
            "instrument_name": "TSTON.staked",
            "underlying_inst_name": "TON",
            "cycle_id": "5",
            "staking_id": "123",
            "status": "COMPLETED",
            "account": "test-account-456",
            "quantity": "1000",
            "side": "UNSTAKE",
            "create_timestamp_ms": "1668658093600"
        });

        let entry: StakeHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.instrument_name, "TSTON.staked");
        assert_eq!(entry.underlying_inst_name, "TON");
        assert_eq!(entry.status, "COMPLETED");
        assert_eq!(entry.side, "UNSTAKE");
        assert_eq!(entry.quantity, "1000");
    }
}
