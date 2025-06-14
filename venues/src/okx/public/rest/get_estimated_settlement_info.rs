use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for getting estimated settlement information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEstimatedSettlementInfoRequest {
    /// Instrument ID, e.g. "XRP-USDT-250307". Only applicable to FUTURES
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Individual estimated settlement information entry
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

/// Response for getting estimated settlement information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEstimatedSettlementInfoResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Estimated settlement information data
    pub data: Vec<EstimatedSettlementInfo>,
}

impl RestClient {
    /// Get estimated settlement information
    ///
    /// Retrieve the estimated settlement price which will only have a return value one
    /// hour before the settlement.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-estimated-settlement-price
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The estimated settlement information request parameters
    ///
    /// # Returns
    /// Response containing the estimated settlement information
    pub async fn get_estimated_settlement_info(
        &self,
        request: &GetEstimatedSettlementInfoRequest,
    ) -> RestResult<GetEstimatedSettlementInfoResponse> {
        self.send_request(
            "api/v5/public/estimated-settlement-info",
            reqwest::Method::GET,
            Some(request),
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
            "estSettlePx": "0.5123",
            "ts": "1597026383085"
        });

        let info: EstimatedSettlementInfo = serde_json::from_value(info_json).unwrap();
        assert_eq!(info.inst_id, "XRP-USDT-250307");
        assert_eq!(info.next_settle_time, "1597026383085");
        assert_eq!(info.est_settle_px, "0.5123");
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
                    "estSettlePx": "0.5123",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetEstimatedSettlementInfoResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "XRP-USDT-250307");
        assert_eq!(response.data.first().unwrap().next_settle_time, "1597026383085");
        assert_eq!(response.data.first().unwrap().est_settle_px, "0.5123");
        assert_eq!(response.data.first().unwrap().ts, "1597026383085");
    }

    #[test]
    fn test_estimated_settlement_info_serialization_roundtrip() {
        let original = GetEstimatedSettlementInfoRequest {
            inst_id: "BTC-USDT-250307".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetEstimatedSettlementInfoRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_estimated_settlement_info_response_empty_data() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": []
        });

        let response: GetEstimatedSettlementInfoResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_estimated_settlement_info_response_with_multiple_entries() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "XRP-USDT-250307",
                    "nextSettleTime": "1597026383085",
                    "estSettlePx": "0.5123",
                    "ts": "1597026383085"
                },
                {
                    "instId": "BTC-USDT-250307",
                    "nextSettleTime": "1597026383086",
                    "estSettlePx": "45000.123",
                    "ts": "1597026383086"
                }
            ]
        });

        let response: GetEstimatedSettlementInfoResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);
        
        assert_eq!(response.data[0].inst_id, "XRP-USDT-250307");
        assert_eq!(response.data[0].est_settle_px, "0.5123");
        
        assert_eq!(response.data[1].inst_id, "BTC-USDT-250307");
        assert_eq!(response.data[1].est_settle_px, "45000.123");
    }
}