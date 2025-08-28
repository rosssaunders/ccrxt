use serde::{Deserialize, Serialize};

use crate::cryptocom::{ApiResult, PrivateRestClient as RestClient, RestResult};

/// Endpoint path for the convert API
const CONVERT_ENDPOINT: &str = "exchange/v1/private/staking/convert";

/// Request parameters for the convert endpoint.
///
/// Use this struct to specify the parameters for converting between staked and liquid staking tokens.
#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct ConvertRequest {
    /// Instrument name to convert from: "ETH.staked" or "CDCETH".
    #[serde(rename = "from_instrument_name")]
    pub from_instrument_name: String,

    /// Instrument name to convert to: "CDCETH" if from is "ETH.staked", "ETH.staked" if from is "CDCETH".
    #[serde(rename = "to_instrument_name")]
    pub to_instrument_name: String,

    /// Expected conversion rate, received from public/staking/get-conversion-rate.
    #[serde(rename = "expected_rate")]
    pub expected_rate: String,

    /// Quantity to be converted in from_instrument_name.
    #[serde(rename = "from_quantity")]
    pub from_quantity: String,

    /// Maximum slippage allowed in basis point.
    #[serde(rename = "slippage_tolerance_bps")]
    pub slippage_tolerance_bps: String,
}

/// Result data for the convert endpoint.
///
/// Contains details of the conversion request and result.
#[derive(Debug, Clone, Deserialize)]
pub struct ConvertResult {
    /// Instrument name to convert from, e.g. "ETH.staked".
    #[serde(rename = "from_instrument_name")]
    pub from_instrument_name: String,

    /// Instrument name to convert to, e.g. "CDCETH".
    #[serde(rename = "to_instrument_name")]
    pub to_instrument_name: String,

    /// Expected conversion rate.
    #[serde(rename = "expected_rate")]
    pub expected_rate: String,

    /// Quantity to be converted in from_instrument_name.
    #[serde(rename = "from_quantity")]
    pub from_quantity: String,

    /// Maximum slippage allowed in basis point.
    #[serde(rename = "slippage_tolerance_bps")]
    pub slippage_tolerance_bps: String,

    /// Convert request id.
    #[serde(rename = "convert_id")]
    pub convert_id: String,

    /// Reason for the status, e.g. "NO_ERROR".
    #[serde(rename = "reason")]
    pub reason: String,
}

/// Response wrapper for endpoint
pub type ConvertResponse = ApiResult<ConvertResult>;

impl RestClient {
    /// Create a request to convert between staked token and liquid staking token.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/index.html)
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `params` - Parameters for the convert request.
    ///
    /// # Returns
    /// Convert request information including convert ID, rates, and reason.
    pub async fn convert(&self, params: ConvertRequest) -> RestResult<ConvertResponse> {
        self.send_signed_request(CONVERT_ENDPOINT, params).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_convert_request_structure() {
        let request = ConvertRequest {
            from_instrument_name: "ETH.staked".to_string(),
            to_instrument_name: "CDCETH".to_string(),
            expected_rate: "1.0203".to_string(),
            from_quantity: "3.14159265".to_string(),
            slippage_tolerance_bps: "3".to_string(),
        };
        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(
            json_value.get("from_instrument_name").unwrap(),
            "ETH.staked"
        );
        assert_eq!(json_value.get("to_instrument_name").unwrap(), "CDCETH");
        assert_eq!(json_value.get("expected_rate").unwrap(), "1.0203");
        assert_eq!(json_value.get("from_quantity").unwrap(), "3.14159265");
        assert_eq!(json_value.get("slippage_tolerance_bps").unwrap(), "3");
    }

    #[test]
    fn test_convert_request_reverse_direction() {
        let request = ConvertRequest {
            from_instrument_name: "CDCETH".to_string(),
            to_instrument_name: "ETH.staked".to_string(),
            expected_rate: "0.9801".to_string(),
            from_quantity: "2.0".to_string(),
            slippage_tolerance_bps: "5".to_string(),
        };
        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value.get("from_instrument_name").unwrap(), "CDCETH");
        assert_eq!(json_value.get("to_instrument_name").unwrap(), "ETH.staked");
        assert_eq!(json_value.get("expected_rate").unwrap(), "0.9801");
        assert_eq!(json_value.get("from_quantity").unwrap(), "2.0");
        assert_eq!(json_value.get("slippage_tolerance_bps").unwrap(), "5");
    }

    #[test]
    fn test_convert_request_serialization() {
        let request = ConvertRequest {
            from_instrument_name: "ETH.staked".to_string(),
            to_instrument_name: "CDCETH".to_string(),
            expected_rate: "1.0203".to_string(),
            from_quantity: "3.14159265".to_string(),
            slippage_tolerance_bps: "3".to_string(),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ConvertRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.from_instrument_name, "ETH.staked");
        assert_eq!(deserialized.to_instrument_name, "CDCETH");
        assert_eq!(deserialized.expected_rate, "1.0203");
    }

    #[test]
    fn test_convert_response_structure() {
        let response_json = json!({
            "code": 0,
            "id": 1,
            "result": {
                "from_instrument_name": "ETH.staked",
                "to_instrument_name": "CDCETH",
                "expected_rate": "1.0203",
                "from_quantity": "3.14159265",
                "slippage_tolerance_bps": "3",
                "convert_id": "1",
                "reason": "NO_ERROR"
            }
        });
        let response: ConvertResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.from_instrument_name, "ETH.staked");
        assert_eq!(response.result.to_instrument_name, "CDCETH");
        assert_eq!(response.result.expected_rate, "1.0203");
        assert_eq!(response.result.from_quantity, "3.14159265");
        assert_eq!(response.result.slippage_tolerance_bps, "3");
        assert_eq!(response.result.convert_id, "1");
        assert_eq!(response.result.reason, "NO_ERROR");
    }

    #[test]
    fn test_convert_response_different_convert_ids() {
        let response_json = json!({
            "code": 0,
            "id": 2,
            "result": {
                "from_instrument_name": "CDCETH",
                "to_instrument_name": "ETH.staked",
                "expected_rate": "0.9801",
                "from_quantity": "2.0",
                "slippage_tolerance_bps": "5",
                "convert_id": "42",
                "reason": "NO_ERROR"
            }
        });
        let response: ConvertResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.convert_id, "42");
    }

    #[test]
    fn test_convert_different_slippage_tolerances() {
        let response_json = json!({
            "code": 0,
            "id": 3,
            "result": {
                "from_instrument_name": "ETH.staked",
                "to_instrument_name": "CDCETH",
                "expected_rate": "1.0203",
                "from_quantity": "3.14159265",
                "slippage_tolerance_bps": "7",
                "convert_id": "99",
                "reason": "NO_ERROR"
            }
        });
        let response: ConvertResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.slippage_tolerance_bps, "7");
    }
}
