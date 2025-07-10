use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request parameters for getting open interest
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterestRequest {
    /// Instrument type (required)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Underlying (for SWAP/FUTURES/OPTION)
    /// If instType is OPTION, either uly or instFamily is required
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// Instrument family (for FUTURES/SWAP/OPTION)
    /// If instType is OPTION, either uly or instFamily is required
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID (optional)
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Individual open interest data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenInterest {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Open interest in number of contracts
    pub oi: String,
    /// Open interest in number of coin
    #[serde(rename = "oiCcy")]
    pub oi_ccy: String,
    /// Open interest in number of USD
    #[serde(rename = "oiUsd")]
    pub oi_usd: String,
    /// Data return time, Unix timestamp format in milliseconds
    pub ts: String,
}

/// Response for getting open interest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenInterestResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Open interest data
    pub data: Vec<OpenInterest>,
}

impl RestClient {
    /// Get open interest
    ///
    /// Retrieve the total open interest for contracts on OKX.
    ///
    /// See: https://www.okx.com/docs-v5/en/#public-data-rest-api-get-open-interest
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The open interest request parameters
    ///
    /// # Returns
    /// Response containing the open interest data
    pub async fn get_open_interest(
        &self,
        request: GetOpenInterestRequest,
    ) -> RestResult<GetOpenInterestResponse> {
        self.send_request(
            "api/v5/public/open-interest",
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
    fn test_get_open_interest_request_structure() {
        let request = GetOpenInterestRequest {
            inst_type: InstrumentType::Swap,
            underlying: Some("BTC-USD".to_string()),
            inst_family: None,
            inst_id: Some("BTC-USDT-SWAP".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("SWAP")
        );
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USDT-SWAP")
        );
        // instFamily should be omitted since it's None
        assert!(serialized.get("instFamily").is_none());
    }

    #[test]
    fn test_get_open_interest_request_with_inst_family() {
        let request = GetOpenInterestRequest {
            inst_type: InstrumentType::Option,
            underlying: None,
            inst_family: Some("BTC-USD".to_string()),
            inst_id: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("OPTION")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        // uly and instId should be omitted since they're None
        assert!(serialized.get("uly").is_none());
        assert!(serialized.get("instId").is_none());
    }

    #[test]
    fn test_open_interest_structure() {
        let open_interest_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USDT-SWAP",
            "oi": "12345",
            "oiCcy": "123.45",
            "oiUsd": "1234567.89",
            "ts": "1597026383085"
        });

        let open_interest: OpenInterest = serde_json::from_value(open_interest_json).unwrap();
        assert_eq!(open_interest.inst_type, "SWAP");
        assert_eq!(open_interest.inst_id, "BTC-USDT-SWAP");
        assert_eq!(open_interest.oi, "12345");
        assert_eq!(open_interest.oi_ccy, "123.45");
        assert_eq!(open_interest.oi_usd, "1234567.89");
        assert_eq!(open_interest.ts, "1597026383085");
    }

    #[test]
    fn test_get_open_interest_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USDT-SWAP",
                    "oi": "12345",
                    "oiCcy": "123.45",
                    "oiUsd": "1234567.89",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetOpenInterestResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);

        let open_interest = &response.data[0];
        assert_eq!(open_interest.inst_type, "SWAP");
        assert_eq!(open_interest.inst_id, "BTC-USDT-SWAP");
        assert_eq!(open_interest.oi, "12345");
        assert_eq!(open_interest.oi_ccy, "123.45");
        assert_eq!(open_interest.oi_usd, "1234567.89");
        assert_eq!(open_interest.ts, "1597026383085");
    }

    #[test]
    fn test_get_open_interest_request_serialization_roundtrip() {
        let original = GetOpenInterestRequest {
            inst_type: InstrumentType::Futures,
            underlying: Some("ETH-USD".to_string()),
            inst_family: Some("ETH-USD".to_string()),
            inst_id: Some("ETH-USD-240329".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetOpenInterestRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.underlying, deserialized.underlying);
        assert_eq!(original.inst_family, deserialized.inst_family);
        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_minimal_open_interest_request() {
        let request = GetOpenInterestRequest {
            inst_type: InstrumentType::Swap,
            underlying: None,
            inst_family: None,
            inst_id: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("SWAP")
        );
        // All optional fields should be omitted
        assert!(serialized.get("uly").is_none());
        assert!(serialized.get("instFamily").is_none());
        assert!(serialized.get("instId").is_none());
    }

    #[test]
    fn test_multiple_open_interest_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USDT-SWAP",
                    "oi": "12345",
                    "oiCcy": "123.45",
                    "oiUsd": "1234567.89",
                    "ts": "1597026383085"
                },
                {
                    "instType": "FUTURES",
                    "instId": "ETH-USD-240329",
                    "oi": "67890",
                    "oiCcy": "678.90",
                    "oiUsd": "2345678.90",
                    "ts": "1597026383086"
                }
            ]
        });

        let response: GetOpenInterestResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);

        assert_eq!(response.data[0].inst_type, "SWAP");
        assert_eq!(response.data[0].inst_id, "BTC-USDT-SWAP");

        assert_eq!(response.data[1].inst_type, "FUTURES");
        assert_eq!(response.data[1].inst_id, "ETH-USD-240329");
    }
}
