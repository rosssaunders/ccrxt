use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request parameters for getting open interest
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterestRequest {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Underlying (for FUTURES/SWAP/OPTION)
    #[serde(rename = "uly")]
    pub underlying: Option<String>,
    /// Instrument family (for FUTURES/SWAP/OPTION)
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
}

/// Open interest information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    /// Data return time (Unix timestamp in milliseconds)
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
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-open-interest
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The open interest request parameters
    ///
    /// # Returns
    /// Response containing the open interest information
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
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_open_interest_request_structure() {
        let request = GetOpenInterestRequest {
            inst_type: InstrumentType::Swap,
            underlying: None,
            inst_family: None,
            inst_id: Some("BTC-USD-SWAP".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instType").and_then(|v| v.as_str()), Some("SWAP"));
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USD-SWAP"));
    }

    #[test]
    fn test_open_interest_structure() {
        let open_interest_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USD-SWAP",
            "oi": "1000000",
            "oiCcy": "100",
            "oiUsd": "3100000",
            "ts": "1597026383085"
        });

        let open_interest: OpenInterest = serde_json::from_value(open_interest_json).unwrap();
        assert_eq!(open_interest.inst_type, "SWAP");
        assert_eq!(open_interest.inst_id, "BTC-USD-SWAP");
        assert_eq!(open_interest.oi, "1000000");
        assert_eq!(open_interest.oi_ccy, "100");
        assert_eq!(open_interest.oi_usd, "3100000");
    }

    #[test]
    fn test_get_open_interest_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USD-SWAP",
                    "oi": "1000000",
                    "oiCcy": "100",
                    "oiUsd": "3100000",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetOpenInterestResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        let open_interest = response.data.first().unwrap();
        assert_eq!(open_interest.inst_id, "BTC-USD-SWAP");
        assert_eq!(open_interest.oi, "1000000");
    }
}