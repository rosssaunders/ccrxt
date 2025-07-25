use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{ApiResult, RestResult};

const OPEN_CONVERT_ENDPOINT: &str = "private/get-open-convert";
/// Request parameters for get open convert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenConvertRequest {
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

/// Open convert entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenConvertEntry {
    /// Instrument name to convert from: ETH.staked or CDCETH
    pub from_instrument_name: String,
    /// Instrument name to convert to, e.g. CDCETH
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
    /// Request status: NEW
    pub status: String,
    /// Request creation timestamp in milliseconds in Unix time format
    pub create_timestamp_ms: String,
}

/// Open convert data result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenConvertResult {
    /// Array of open convert data
    pub data: Vec<OpenConvertEntry>,
}

/// Response wrapper for get open convert endpoint
pub type GetOpenConvertResponse = ApiResult<GetOpenConvertResult>;

impl RestClient {
    /// Get convert request that status is not in final state
    ///
    /// Returns convert requests that are still pending or in progress.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `params` - Request parameters including optional start_time, end_time, and limit
    ///
    /// # Returns
    /// Open convert requests with status, rates, and timestamps
    pub async fn get_open_convert(
        &self,
        params: GetOpenConvertRequest,
    ) -> RestResult<GetOpenConvertResponse> {
        self.send_signed_request(OPEN_CONVERT_ENDPOINT, params)
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
    fn test_get_open_convert_request_empty() {
        let request = GetOpenConvertRequest {
            start_time: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_get_open_convert_request_with_all_params() {
        let request = GetOpenConvertRequest {
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
    fn test_get_open_convert_request_partial_params() {
        let request = GetOpenConvertRequest {
            start_time: Some(1691455454495),
            end_time: None,
            limit: Some("50".to_string()),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("start_time").unwrap(), 1691455454495_u64);
        assert!(json_value.get("end_time").is_none());
        assert_eq!(json_value.get("limit").unwrap(), "50");
    }

    #[test]
    fn test_open_convert_entry_structure() {
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

        let entry: OpenConvertEntry = serde_json::from_value(entry_json).unwrap();
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
    fn test_open_convert_entry_reverse_direction() {
        let entry_json = json!({
            "from_instrument_name": "CDCETH",
            "to_instrument_name": "ETH.staked",
            "expected_rate": "0.9801",
            "from_quantity": "2.0",
            "slippage_tolerance_bps": "5",
            "actual_rate": "0.9802",
            "to_quantity": "1.9604",
            "convert_id": 2,
            "status": "NEW",
            "create_timestamp_ms": "1688140984006"
        });

        let entry: OpenConvertEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.from_instrument_name, "CDCETH");
        assert_eq!(entry.to_instrument_name, "ETH.staked");
        assert_eq!(entry.status, "NEW");
        assert_eq!(entry.convert_id, 2);
    }

    #[test]
    fn test_get_open_convert_response_structure() {
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

        let response: GetOpenConvertResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.data.len(), 1);
        assert_eq!(
            response.result.data.first().unwrap().from_instrument_name,
            "ETH.staked"
        );
        assert_eq!(response.result.data.first().unwrap().status, "COMPLETED");
    }

    #[test]
    fn test_open_convert_multiple_entries() {
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
                        "slippage_tolerance_bps": "5",
                        "actual_rate": "0.9802",
                        "to_quantity": "1.9604",
                        "convert_id": 2,
                        "status": "NEW",
                        "create_timestamp_ms": "1688140984006"
                    }
                ]
            }
        });

        let response: GetOpenConvertResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.data.len(), 2);
        assert_eq!(response.result.data.first().unwrap().status, "COMPLETED");
        assert_eq!(response.result.data.get(1).unwrap().status, "NEW");
        assert_eq!(response.result.data.first().unwrap().convert_id, 1);
        assert_eq!(response.result.data.get(1).unwrap().convert_id, 2);
    }

    #[test]
    fn test_open_convert_rate_differences() {
        // Test case where actual rate differs from expected rate
        let entry_json = json!({
            "from_instrument_name": "ETH.staked",
            "to_instrument_name": "CDCETH",
            "expected_rate": "1.0200",
            "from_quantity": "1.0",
            "slippage_tolerance_bps": "10",
            "actual_rate": "1.0205",
            "to_quantity": "1.0205",
            "convert_id": 3,
            "status": "COMPLETED",
            "create_timestamp_ms": "1688140984007"
        });

        let entry: OpenConvertEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.expected_rate, "1.0200");
        assert_eq!(entry.actual_rate, "1.0205");
        assert_eq!(entry.slippage_tolerance_bps, "10");
    }
}
