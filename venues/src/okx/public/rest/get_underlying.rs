use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request parameters for getting underlying assets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnderlyingRequest {
    /// Instrument type (required)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
}

/// Response for getting underlying assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUnderlyingResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Underlying assets data
    pub data: Vec<UnderlyingData>,
}

/// Individual underlying asset data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderlyingData {
    /// Underlying assets array
    pub uly: Vec<String>,
}

impl RestClient {
    /// Get underlying assets
    ///
    /// Retrieve underlying assets for specified instrument type.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-underlying
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The underlying request parameters
    ///
    /// # Returns
    /// Response containing the list of underlying assets
    pub async fn get_underlying(
        &self,
        request: GetUnderlyingRequest,
    ) -> RestResult<GetUnderlyingResponse> {
        self.send_request(
            "api/v5/public/underlying",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_underlying_request_structure() {
        let request = GetUnderlyingRequest {
            inst_type: InstrumentType::Swap,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("SWAP")
        );
    }

    #[test]
    fn test_get_underlying_request_futures() {
        let request = GetUnderlyingRequest {
            inst_type: InstrumentType::Futures,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("FUTURES")
        );
    }

    #[test]
    fn test_get_underlying_request_option() {
        let request = GetUnderlyingRequest {
            inst_type: InstrumentType::Option,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("OPTION")
        );
    }

    #[test]
    fn test_underlying_data_structure() {
        let underlying_json = json!({
            "uly": ["BTC-USD", "ETH-USD", "LTC-USD"]
        });

        let underlying_data: UnderlyingData = serde_json::from_value(underlying_json).unwrap();
        assert_eq!(underlying_data.uly.len(), 3);
        assert_eq!(underlying_data.uly[0], "BTC-USD");
        assert_eq!(underlying_data.uly[1], "ETH-USD");
        assert_eq!(underlying_data.uly[2], "LTC-USD");
    }

    #[test]
    fn test_get_underlying_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "uly": ["BTC-USD", "ETH-USD", "LTC-USD"]
                }
            ]
        });

        let response: GetUnderlyingResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].uly.len(), 3);
        assert_eq!(response.data[0].uly[0], "BTC-USD");
    }

    #[test]
    fn test_underlying_serialization_roundtrip() {
        let original = GetUnderlyingRequest {
            inst_type: InstrumentType::Swap,
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetUnderlyingRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
    }

    #[test]
    fn test_empty_underlying_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "uly": []
                }
            ]
        });

        let response: GetUnderlyingResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].uly.len(), 0);
    }
}
