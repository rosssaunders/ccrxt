use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request parameters for getting underlying
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnderlyingRequest {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
}

/// Response for getting underlying
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUnderlyingResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Underlying data
    pub data: Vec<String>,
}

impl RestClient {
    /// Get underlying
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-underlying
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The underlying request parameters
    ///
    /// # Returns
    /// Response containing the underlying data
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
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_underlying_request_structure() {
        let request = GetUnderlyingRequest {
            inst_type: InstrumentType::Swap,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instType").and_then(|v| v.as_str()), Some("SWAP"));
    }

    #[test]
    fn test_get_underlying_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": ["BTC-USD", "ETH-USD", "LTC-USD"]
        });

        let response: GetUnderlyingResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 3);
        assert!(response.data.contains(&"BTC-USD".to_string()));
        assert!(response.data.contains(&"ETH-USD".to_string()));
        assert!(response.data.contains(&"LTC-USD".to_string()));
    }
}