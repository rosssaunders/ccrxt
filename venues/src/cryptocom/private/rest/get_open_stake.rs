use serde::{Deserialize, Serialize};

use crate::cryptocom::{ApiResult, PrivateRestClient as RestClient, RestResult};

const OPEN_STAKE_ENDPOINT: &str = "exchange/v1/private/staking/get-open-stake";

/// Request parameters for get open stake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenStakeRequest {
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

/// Open stake entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenStakeEntry {
    /// Staking instrument name, e.g. SOL.staked
    pub instrument_name: String,

    /// Underlying instrument name, e.g. SOL
    pub underlying_inst_name: String,

    /// Cycle id
    pub cycle_id: String,

    /// Request id
    pub staking_id: String,

    /// Request status: NEW, PENDING, PENDING_WITHDRAWAL, PENDING_UNSTAKING, STAKED
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

/// Open stake data result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenStakeResult {
    /// Array of open stake data
    pub data: Vec<OpenStakeEntry>,
}

/// Response wrapper for get open stake endpoint
pub type GetOpenStakeResponse = ApiResult<GetOpenStakeResult>;

impl RestClient {
    /// Get stake/unstake requests that status is not in final state
    ///
    /// Returns open stake and unstake requests that are still pending or in progress.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/index.html)
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `request` - Parameters for retrieving open stakes
    ///
    /// # Returns
    /// Open stake/unstake requests with status, timestamps, and other details
    pub async fn get_open_stake(
        &self,
        request: GetOpenStakeRequest,
    ) -> RestResult<GetOpenStakeResponse> {
        self.send_signed_request(OPEN_STAKE_ENDPOINT, request).await
    }
}

#[cfg(test)]
mod tests {
    use secrets::ExposableSecret;
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
    fn test_get_open_stake_request_empty() {
        let request = GetOpenStakeRequest {
            instrument_name: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_get_open_stake_request_with_all_params() {
        let request = GetOpenStakeRequest {
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
    fn test_get_open_stake_request_partial_params() {
        let request = GetOpenStakeRequest {
            instrument_name: Some("ETH.staked".to_string()),
            start_time: None,
            end_time: Some(1691545277000),
            limit: Some("50".to_string()),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "ETH.staked");
        assert!(json_value.get("start_time").is_none());
        assert_eq!(json_value.get("end_time").unwrap(), 1691545277000_u64);
        assert_eq!(json_value.get("limit").unwrap(), "50");
    }

    #[test]
    fn test_open_stake_entry_structure() {
        let entry_json = json!({
            "instrument_name": "SOL.staked",
            "underlying_inst_name": "SOL",
            "cycle_id": "1",
            "staking_id": "1",
            "status": "PENDING",
            "account": "12345678-9999-1234-9999-123456789999",
            "quantity": "1",
            "side": "STAKE",
            "create_timestamp_ms": "1668658093600"
        });

        let entry: OpenStakeEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.instrument_name, "SOL.staked");
        assert_eq!(entry.underlying_inst_name, "SOL");
        assert_eq!(entry.cycle_id, "1");
        assert_eq!(entry.staking_id, "1");
        assert_eq!(entry.status, "PENDING");
        assert_eq!(entry.account, "12345678-9999-1234-9999-123456789999");
        assert_eq!(entry.quantity, "1");
        assert_eq!(entry.side, "STAKE");
        assert_eq!(entry.create_timestamp_ms, "1668658093600");
    }

    #[test]
    fn test_open_stake_entry_unstake_side() {
        let entry_json = json!({
            "instrument_name": "SOL.staked",
            "underlying_inst_name": "SOL",
            "cycle_id": "2",
            "staking_id": "2",
            "status": "UNSTAKING",
            "account": "12345678-9999-1234-9999-123456789999",
            "quantity": "0.5",
            "side": "UNSTAKE",
            "create_timestamp_ms": "1668658093600"
        });

        let entry: OpenStakeEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.side, "UNSTAKE");
        assert_eq!(entry.status, "UNSTAKING");
        assert_eq!(entry.quantity, "0.5");
    }

    #[test]
    fn test_get_open_stake_response_structure() {
        let response_json = json!({
            "code": 0,
            "id": 1,
            "result": {
                "data": [
                    {
                        "instrument_name": "SOL.staked",
                        "underlying_inst_name": "SOL",
                        "cycle_id": "1",
                        "staking_id": "1",
                        "status": "PENDING",
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
                        "status": "UNSTAKING",
                        "account": "12345678-9999-1234-9999-123456789999",
                        "quantity": "0.5",
                        "side": "UNSTAKE",
                        "create_timestamp_ms": "1668658093600"
                    }
                ]
            }
        });

        let response: GetOpenStakeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.data.len(), 2);
        assert_eq!(response.result.data.first().unwrap().side, "STAKE");
        assert_eq!(response.result.data.get(1).unwrap().side, "UNSTAKE");
        assert_eq!(response.result.data.first().unwrap().status, "PENDING");
        assert_eq!(response.result.data.get(1).unwrap().status, "UNSTAKING");
    }

    #[test]
    fn test_open_stake_different_statuses() {
        let statuses = vec![
            "NEW",
            "PENDING",
            "PENDING_WITHDRAWAL",
            "PENDING_UNSTAKING",
            "STAKED",
        ];

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

            let entry: OpenStakeEntry = serde_json::from_value(entry_json).unwrap();
            assert_eq!(entry.status, status);
        }
    }
}
