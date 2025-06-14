use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request parameters for getting mark price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkPriceRequest {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Underlying (applicable to FUTURES/SWAP/OPTION)
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// Instrument family (applicable to FUTURES/SWAP/OPTION)
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Response for getting mark price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarkPriceResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Mark price data
    pub data: Vec<MarkPriceData>,
}

/// Mark price data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkPriceData {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Mark price
    #[serde(rename = "markPx")]
    pub mark_px: String,
    /// Data return time, Unix timestamp format in milliseconds
    pub ts: String,
}

impl RestClient {
    /// Get mark price
    ///
    /// Retrieve mark price.
    ///
    /// We set the mark price based on the SPOT index and at a reasonable basis to
    /// prevent individual users from manipulating the market and causing the contract
    /// price to fluctuate.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-mark-price
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The mark price request parameters
    ///
    /// # Returns
    /// Response containing the mark price data
    pub async fn get_mark_price(
        &self,
        request: GetMarkPriceRequest,
    ) -> RestResult<GetMarkPriceResponse> {
        self.send_request(
            "api/v5/public/mark-price",
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
    fn test_get_mark_price_request_structure() {
        let request = GetMarkPriceRequest {
            inst_type: InstrumentType::Swap,
            underlying: Some("BTC-USD".to_string()),
            inst_family: None,
            inst_id: Some("BTC-USD-SWAP".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instType").and_then(|v| v.as_str()), Some("SWAP"));
        assert_eq!(serialized.get("uly").and_then(|v| v.as_str()), Some("BTC-USD"));
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USD-SWAP"));
    }

    #[test]
    fn test_mark_price_data_structure() {
        let mark_price_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USD-SWAP",
            "markPx": "50000.5",
            "ts": "1597026383085"
        });

        let mark_price_data: MarkPriceData = serde_json::from_value(mark_price_json).unwrap();
        assert_eq!(mark_price_data.inst_type, "SWAP");
        assert_eq!(mark_price_data.inst_id, "BTC-USD-SWAP");
        assert_eq!(mark_price_data.mark_px, "50000.5");
        assert_eq!(mark_price_data.ts, "1597026383085");
    }

    #[test]
    fn test_get_mark_price_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USD-SWAP",
                    "markPx": "50000.5",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetMarkPriceResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-SWAP");
    }
}