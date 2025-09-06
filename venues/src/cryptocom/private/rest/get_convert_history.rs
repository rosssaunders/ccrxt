use serde::{Deserialize, Serialize};

use crate::cryptocom::{ApiResult, PrivateRestClient as RestClient, RestResult};

/// Endpoint path for the get-convert-history API
const CONVERT_HISTORY_ENDPOINT: &str = "exchange/v1/private/staking/get-convert-history";

/// Request parameters for get convert history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConvertHistoryRequest {
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

/// Convert history entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertHistoryEntry {
    /// Instrument name to convert from: ETH.staked or CDCETH
    pub from_instrument_name: String,

    /// Instrument name to convert to: CDCETH or ETH.staked
    pub to_instrument_name: String,

    /// Expected conversion rate
    pub expected_rate: String,

    /// Quantity to be converted in from_instrument_name
    pub from_quantity: String,

    /// Maximum slippage allowed in basis point
    pub slippage_tolerance_bps: String,

    /// Actual conversion rate
    pub actual_rate: String,

    /// Quantity converted to to_instrument_name
    pub to_quantity: String,

    /// Convert request id
    pub convert_id: u64,

    /// Request status: COMPLETED or Reason of REJECTED
    pub status: String,

    /// Request creation timestamp in milliseconds in Unix time format
    pub create_timestamp_ms: String,
}

/// Convert history data result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConvertHistoryResult {
    /// Array of convert history data
    pub data: Vec<ConvertHistoryEntry>,
}

/// Response wrapper for get convert history endpoint
pub type GetConvertHistoryResponse = ApiResult<GetConvertHistoryResult>;

impl RestClient {
    /// Get convert request history
    ///
    /// Returns historical convert requests that have been completed or rejected.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#private-staking-get-convert-history)
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `request` - Parameters for retrieving convert history
    ///
    /// # Returns
    /// Historical convert requests with final status, rates, and timestamps
    pub async fn get_convert_history(
        &self,
        request: GetConvertHistoryRequest,
    ) -> RestResult<GetConvertHistoryResponse> {
        self.send_signed_request(CONVERT_HISTORY_ENDPOINT, request)
            .await
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
    fn test_get_convert_history_request_empty() {
        let request = GetConvertHistoryRequest {
            start_time: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_get_convert_history_request_with_all_params() {
        let request = GetConvertHistoryRequest {
            start_time: Some(1691455454495),
            end_time: Some(1691545277000),
            limit: Some("10".to_string()),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("start_time").unwrap(), 1691455454495_u64);
        assert_eq!(json_value.get("end_time").unwrap(), 1691545277000_u64);
        assert_eq!(json_value.get("limit").unwrap(), "10");
    }

    #[test]
    fn test_convert_history_entry_completed() {
        let entry_json = json!({
            "from_instrument_name": "ETH.staked",
            "to_instrument_name": "CDCETH",
            "expected_rate": "1.0203",
            "from_quantity": "3.14159265",
            "slippage_tolerance_bps": "3",
            "actual_rate": "1.0203",
            "to_quantity": "3.14159265",
            "convert_id": 1,
            "status": "COMPLETED",
            "create_timestamp_ms": "1688140984005"
        });

        let entry: ConvertHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.from_instrument_name, "ETH.staked");
        assert_eq!(entry.to_instrument_name, "CDCETH");
        assert_eq!(entry.expected_rate, "1.0203");
        assert_eq!(entry.from_quantity, "3.14159265");
        assert_eq!(entry.slippage_tolerance_bps, "3");
        assert_eq!(entry.actual_rate, "1.0203");
        assert_eq!(entry.to_quantity, "3.14159265");
        assert_eq!(entry.convert_id, 1);
        assert_eq!(entry.status, "COMPLETED");
        assert_eq!(entry.create_timestamp_ms, "1688140984005");
    }

    #[test]
    fn test_convert_history_entry_rejected() {
        let entry_json = json!({
            "from_instrument_name": "CDCETH",
            "to_instrument_name": "ETH.staked",
            "expected_rate": "0.9801",
            "from_quantity": "2.0",
            "slippage_tolerance_bps": "1",
            "actual_rate": "0.9750",
            "to_quantity": "0.0",
            "convert_id": 2,
            "status": "SLIPPAGE_TOO_HIGH",
            "create_timestamp_ms": "1688140984006"
        });

        let entry: ConvertHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.from_instrument_name, "CDCETH");
        assert_eq!(entry.to_instrument_name, "ETH.staked");
        assert_eq!(entry.status, "SLIPPAGE_TOO_HIGH");
        assert_eq!(entry.to_quantity, "0.0");
        assert_eq!(entry.convert_id, 2);
    }

    #[test]
    fn test_get_convert_history_response_structure() {
        let response_json = json!({
            "code": 0,
            "id": 1,
            "result": {
                "data": [
                    {
                        "from_instrument_name": "ETH.staked",
                        "to_instrument_name": "CDCETH",
                        "expected_rate": "1.0203",
                        "from_quantity": "3.14159265",
                        "slippage_tolerance_bps": "3",
                        "actual_rate": "1.0203",
                        "to_quantity": "3.14159265",
                        "convert_id": 1,
                        "status": "COMPLETED",
                        "create_timestamp_ms": "1688140984005"
                    }
                ]
            }
        });

        let response: GetConvertHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.data.len(), 1);
        assert_eq!(
            response.result.data.first().unwrap().from_instrument_name,
            "ETH.staked"
        );
        assert_eq!(response.result.data.first().unwrap().status, "COMPLETED");
    }

    #[test]
    fn test_convert_history_multiple_entries() {
        let response_json = json!({
            "code": 0,
            "id": 1,
            "result": {
                "data": [
                    {
                        "from_instrument_name": "ETH.staked",
                        "to_instrument_name": "CDCETH",
                        "expected_rate": "1.0203",
                        "from_quantity": "1.0",
                        "slippage_tolerance_bps": "3",
                        "actual_rate": "1.0203",
                        "to_quantity": "1.0203",
                        "convert_id": 1,
                        "status": "COMPLETED",
                        "create_timestamp_ms": "1688140984005"
                    },
                    {
                        "from_instrument_name": "CDCETH",
                        "to_instrument_name": "ETH.staked",
                        "expected_rate": "0.9801",
                        "from_quantity": "2.0",
                        "slippage_tolerance_bps": "1",
                        "actual_rate": "0.9750",
                        "to_quantity": "0.0",
                        "convert_id": 2,
                        "status": "SLIPPAGE_TOO_HIGH",
                        "create_timestamp_ms": "1688140984006"
                    }
                ]
            }
        });

        let response: GetConvertHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.data.len(), 2);
        assert_eq!(response.result.data.first().unwrap().status, "COMPLETED");
        assert_eq!(
            response.result.data.get(1).unwrap().status,
            "SLIPPAGE_TOO_HIGH"
        );
        assert_eq!(response.result.data.first().unwrap().convert_id, 1);
        assert_eq!(response.result.data.get(1).unwrap().convert_id, 2);
    }

    #[test]
    fn test_convert_history_final_statuses() {
        let statuses = vec![
            "COMPLETED",
            "INSUFFICIENT_BALANCE",
            "SLIPPAGE_TOO_HIGH",
            "MARKET_CLOSED",
        ];

        for status in statuses {
            let entry_json = json!({
                "from_instrument_name": "ETH.staked",
                "to_instrument_name": "CDCETH",
                "expected_rate": "1.0203",
                "from_quantity": "1.0",
                "slippage_tolerance_bps": "3",
                "actual_rate": "1.0203",
                "to_quantity": if status == "COMPLETED" { "1.0203" } else { "0.0" },
                "convert_id": 3,
                "status": status,
                "create_timestamp_ms": "1688140984007"
            });

            let entry: ConvertHistoryEntry = serde_json::from_value(entry_json).unwrap();
            assert_eq!(entry.status, status);
        }
    }

    #[test]
    fn test_convert_history_rate_differences() {
        // Test case where actual rate differs significantly from expected rate
        let entry_json = json!({
            "from_instrument_name": "ETH.staked",
            "to_instrument_name": "CDCETH",
            "expected_rate": "1.0200",
            "from_quantity": "5.0",
            "slippage_tolerance_bps": "20",
            "actual_rate": "1.0180",
            "to_quantity": "5.090",
            "convert_id": 4,
            "status": "COMPLETED",
            "create_timestamp_ms": "1688140984008"
        });

        let entry: ConvertHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.expected_rate, "1.0200");
        assert_eq!(entry.actual_rate, "1.0180");
        assert_eq!(entry.slippage_tolerance_bps, "20");
        assert_eq!(entry.status, "COMPLETED");
    }
}
