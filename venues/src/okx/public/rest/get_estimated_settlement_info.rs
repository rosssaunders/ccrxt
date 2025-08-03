use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const PUBLIC_ESTIMATED_SETTLEMENT_INFO_ENDPOINT: &str = "api/v5/public/estimated-settlement-info";
/// Request parameters for getting estimated settlement info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEstimatedSettlementInfoRequest {
    /// Instrument ID, e.g. "XRP-USDT-250307", only applicable to FUTURES
    pub inst_id: String,
}

/// Individual estimated settlement info entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedSettlementInfo {
    /// Instrument ID, e.g. "XRP-USDT-250307"
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Next settlement time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    #[serde(rename = "nextSettleTime")]
    pub next_settle_time: String,
    /// Estimated settlement price
    #[serde(rename = "estSettlePx")]
    pub est_settle_px: String,
    /// Data return time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    pub ts: String,
}

/// Response for getting estimated settlement info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEstimatedSettlementInfoResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Estimated settlement info data
    pub data: Vec<EstimatedSettlementInfo>,
}

impl RestClient {
    /// Get estimated future settlement price
    ///
    /// Retrieve the estimated settlement price which will only have a return value one
    /// hour before the settlement.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-estimated-future-settlement-price
    ///
    /// Rate limit: 10 requests per 2 seconds
    /// Rate limit rule: IP + Instrument ID
    ///
    /// # Arguments
    /// * `request` - The estimated settlement info request parameters
    ///
    /// # Returns
    /// Response containing the estimated settlement info
    pub async fn get_estimated_settlement_info(
        &self,
        request: &GetEstimatedSettlementInfoRequest,
    ) -> RestResult<GetEstimatedSettlementInfoResponse> {
        self.send_request(
            PUBLIC_ESTIMATED_SETTLEMENT_INFO_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
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
    fn test_get_estimated_settlement_info_request_structure() {
        let request = GetEstimatedSettlementInfoRequest {
            inst_id: "XRP-USDT-250307".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("XRP-USDT-250307")
        );
    }

    #[test]
    fn test_estimated_settlement_info_structure() {
        let info_json = json!({
            "instId": "XRP-USDT-250307",
            "nextSettleTime": "1597026383085",
            "estSettlePx": "0.5234",
            "ts": "1597026383085"
        });

        let info: EstimatedSettlementInfo = serde_json::from_value(info_json).unwrap();
        assert_eq!(info.inst_id, "XRP-USDT-250307");
        assert_eq!(info.next_settle_time, "1597026383085");
        assert_eq!(info.est_settle_px, "0.5234");
        assert_eq!(info.ts, "1597026383085");
    }

    #[test]
    fn test_get_estimated_settlement_info_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "XRP-USDT-250307",
                    "nextSettleTime": "1597026383085",
                    "estSettlePx": "0.5234",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetEstimatedSettlementInfoResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "XRP-USDT-250307");
        assert_eq!(
            response.data.first().unwrap().next_settle_time,
            "1597026383085"
        );
        assert_eq!(response.data.first().unwrap().est_settle_px, "0.5234");
        assert_eq!(response.data.first().unwrap().ts, "1597026383085");
    }

    #[test]
    fn test_estimated_settlement_info_serialization_roundtrip() {
        let original = GetEstimatedSettlementInfoRequest {
            inst_id: "BTC-USDT-250307".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetEstimatedSettlementInfoRequest =
            serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_estimated_settlement_info_empty_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": []
        });

        let response: GetEstimatedSettlementInfoResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_estimated_settlement_info_request_different_instruments() {
        let btc_request = GetEstimatedSettlementInfoRequest {
            inst_id: "BTC-USD-230929".to_string(),
        };
        let eth_request = GetEstimatedSettlementInfoRequest {
            inst_id: "ETH-USD-230929".to_string(),
        };

        let btc_serialized = serde_json::to_value(&btc_request).unwrap();
        let eth_serialized = serde_json::to_value(&eth_request).unwrap();

        assert_eq!(
            btc_serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-230929")
        );
        assert_eq!(
            eth_serialized.get("instId").and_then(|v| v.as_str()),
            Some("ETH-USD-230929")
        );
    }
}
