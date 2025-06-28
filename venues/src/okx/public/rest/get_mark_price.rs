use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Request parameters for getting mark price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarkPriceRequest {
    /// Instrument type. Required.
    /// MARGIN, SWAP, FUTURES, OPTION
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Underlying. Optional.
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family. Optional.
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID, e.g. "BTC-USD-SWAP". Optional.
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Individual mark price entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkPrice {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID, e.g. "BTC-USD-200214"
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Mark price
    #[serde(rename = "markPx")]
    pub mark_px: String,
    /// Data return time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    pub ts: String,
}

/// Response for getting mark price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarkPriceResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Mark price data
    pub data: Vec<MarkPrice>,
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
    /// Rate limit rule: IP + Instrument ID
    ///
    /// # Arguments
    /// * `request` - The mark price request parameters
    ///
    /// # Returns
    /// Response containing mark price data
    pub async fn get_mark_price(
        &self,
        request: &GetMarkPriceRequest,
    ) -> RestResult<GetMarkPriceResponse> {
        self.send_request(
            "api/v5/public/mark-price",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PublicMarketDataHistory,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_mark_price_request_minimal() {
        let request = GetMarkPriceRequest {
            inst_type: "SWAP".to_string(),
            uly: None,
            inst_family: None,
            inst_id: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("SWAP")
        );
        // Optional fields should not be present when None
        assert!(serialized.get("uly").is_none());
        assert!(serialized.get("instFamily").is_none());
        assert!(serialized.get("instId").is_none());
        // Only instType should be present
        assert_eq!(serialized.as_object().unwrap().len(), 1);
    }

    #[test]
    fn test_get_mark_price_request_full() {
        let request = GetMarkPriceRequest {
            inst_type: "FUTURES".to_string(),
            uly: Some("BTC-USD".to_string()),
            inst_family: Some("BTC-USD".to_string()),
            inst_id: Some("BTC-USD-SWAP".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("FUTURES")
        );
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
    }

    #[test]
    fn test_mark_price_structure() {
        let mark_price_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USD-SWAP",
            "markPx": "50000.5",
            "ts": "1597026383085"
        });

        let mark_price: MarkPrice = serde_json::from_value(mark_price_json).unwrap();
        assert_eq!(mark_price.inst_type, "SWAP");
        assert_eq!(mark_price.inst_id, "BTC-USD-SWAP");
        assert_eq!(mark_price.mark_px, "50000.5");
        assert_eq!(mark_price.ts, "1597026383085");
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
                },
                {
                    "instType": "FUTURES",
                    "instId": "BTC-USD-200214",
                    "markPx": "50001.2",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetMarkPriceResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].inst_id, "BTC-USD-SWAP");
        assert_eq!(response.data[0].mark_px, "50000.5");
        assert_eq!(response.data[1].inst_id, "BTC-USD-200214");
        assert_eq!(response.data[1].mark_px, "50001.2");
    }

    #[test]
    fn test_get_mark_price_request_serialization_roundtrip() {
        let original = GetMarkPriceRequest {
            inst_type: "OPTION".to_string(),
            uly: Some("ETH-USD".to_string()),
            inst_family: Some("ETH-USD".to_string()),
            inst_id: Some("ETH-USD-230630-2000-C".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetMarkPriceRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.uly, deserialized.uly);
        assert_eq!(original.inst_family, deserialized.inst_family);
        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_mark_price_different_instrument_types() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "MARGIN",
                    "instId": "BTC-USDT",
                    "markPx": "50000.5",
                    "ts": "1597026383085"
                },
                {
                    "instType": "SWAP",
                    "instId": "BTC-USD-SWAP",
                    "markPx": "50001.2",
                    "ts": "1597026383085"
                },
                {
                    "instType": "FUTURES",
                    "instId": "BTC-USD-240329",
                    "markPx": "50002.8",
                    "ts": "1597026383085"
                },
                {
                    "instType": "OPTION",
                    "instId": "BTC-USD-230630-50000-C",
                    "markPx": "1500.5",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetMarkPriceResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 4);

        // Check different instrument types
        assert_eq!(response.data[0].inst_type, "MARGIN");
        assert_eq!(response.data[1].inst_type, "SWAP");
        assert_eq!(response.data[2].inst_type, "FUTURES");
        assert_eq!(response.data[3].inst_type, "OPTION");

        // Check mark prices
        assert_eq!(response.data[0].mark_px, "50000.5");
        assert_eq!(response.data[1].mark_px, "50001.2");
        assert_eq!(response.data[2].mark_px, "50002.8");
        assert_eq!(response.data[3].mark_px, "1500.5");
    }

    #[test]
    fn test_empty_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": []
        });

        let response: GetMarkPriceResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 0);
    }
}
