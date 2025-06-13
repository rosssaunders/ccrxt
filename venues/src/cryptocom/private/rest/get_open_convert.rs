use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::cryptocom::RestResult;
use super::client::RestClient;

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

/// Response for get open convert endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenConvertResponse {
    /// Array of open convert data
    pub data: Vec<OpenConvertEntry>,
}

impl RestClient {
    /// Get convert request that status is not in final state
    ///
    /// Returns convert requests that are still pending or in progress.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html#private-staking-get-open-convert>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `start_time` - Optional start time in Unix time format (inclusive)
    /// * `end_time` - Optional end time in Unix time format (inclusive)
    /// * `limit` - Optional maximum number of requests returned (Default: 20, Max: 500)
    ///
    /// # Returns
    /// Open convert requests with status, rates, and timestamps
    pub async fn get_open_convert(
        &self,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<&str>,
    ) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        
        let mut params = json!({});
        
        if let Some(start) = start_time {
            params["start_time"] = json!(start);
        }
        if let Some(end) = end_time {
            params["end_time"] = json!(end);
        }
        if let Some(lmt) = limit {
            params["limit"] = json!(lmt);
        }
        
        let signature = self.sign_request("private/staking/get-open-convert", id, &params, nonce)?;
        
        let request_body = json!({
            "id": id,
            "method": "private/staking/get-open-convert",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self.client
            .post(&format!("{}/v1/private/staking/get-open-convert", self.base_url))
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
        assert_eq!(json_value["start_time"], 1691455454495_u64);
        assert_eq!(json_value["end_time"], 1691545277000_u64);
        assert_eq!(json_value["limit"], "10");
    }

    #[test]
    fn test_get_open_convert_request_partial_params() {
        let request = GetOpenConvertRequest {
            start_time: Some(1691455454495),
            end_time: None,
            limit: Some("50".to_string()),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value["start_time"], 1691455454495_u64);
        assert!(json_value.get("end_time").is_none());
        assert_eq!(json_value["limit"], "50");
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
        });

        let response: GetOpenConvertResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].from_instrument_name, "ETH.staked");
        assert_eq!(response.data[0].status, "COMPLETED");
    }

    #[test]
    fn test_open_convert_multiple_entries() {
        let response_json = json!({
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
        });

        let response: GetOpenConvertResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].status, "COMPLETED");
        assert_eq!(response.data[1].status, "NEW");
        assert_eq!(response.data[0].convert_id, 1);
        assert_eq!(response.data[1].convert_id, 2);
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