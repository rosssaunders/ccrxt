use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const PUBLIC_SETTLEMENT_HISTORY_ENDPOINT: &str = "/api/v5/public/settlement-history";

/// Request parameters for getting settlement history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettlementHistoryRequest {
    /// Instrument family
    #[serde(rename = "instFamily")]
    pub inst_family: String,
    /// Pagination of data to return records earlier than the requested ts (not included)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ts (not included)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. The maximum is 100. The default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Individual settlement detail entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementDetail {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Settlement price
    #[serde(rename = "settlePx")]
    pub settle_px: String,
}

/// Individual settlement history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementHistory {
    /// Settlement time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    pub ts: String,
    /// Settlement info
    pub details: Vec<SettlementDetail>,
}

/// Response for getting settlement history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettlementHistoryResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Settlement history data
    pub data: Vec<SettlementHistory>,
}

impl RestClient {
    /// Get futures settlement history
    ///
    /// Retrieve settlement records of futures in the last 3 months.
    ///
    /// See: https://www.okx.com/docs-v5/en/#public-data-rest-api-get-futures-settlement-history
    ///
    /// Rate limit: 40 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The settlement history request parameters
    ///
    /// # Returns
    /// Response containing the list of settlement history entries
    pub async fn get_settlement_history(
        &self,
        request: &GetSettlementHistoryRequest,
    ) -> RestResult<GetSettlementHistoryResponse> {
        self.send_request(
            PUBLIC_SETTLEMENT_HISTORY_ENDPOINT,
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
    fn test_get_settlement_history_request_structure() {
        let request = GetSettlementHistoryRequest {
            inst_family: "BTC-USD".to_string(),
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("50"));
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
    }

    #[test]
    fn test_get_settlement_history_request_with_pagination() {
        let request = GetSettlementHistoryRequest {
            inst_family: "ETH-USD".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597026483085".to_string()),
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("ETH-USD")
        );
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1597026383085")
        );
        assert_eq!(
            serialized.get("before").and_then(|v| v.as_str()),
            Some("1597026483085")
        );
        assert_eq!(
            serialized.get("limit").and_then(|v| v.as_str()),
            Some("100")
        );
    }

    #[test]
    fn test_settlement_detail_structure() {
        let settlement_detail_json = json!({
            "instId": "BTC-USD-230630",
            "settlePx": "30250.5"
        });

        let settlement_detail: SettlementDetail =
            serde_json::from_value(settlement_detail_json).unwrap();
        assert_eq!(settlement_detail.inst_id, "BTC-USD-230630");
        assert_eq!(settlement_detail.settle_px, "30250.5");
    }

    #[test]
    fn test_settlement_history_structure() {
        let settlement_history_json = json!({
            "ts": "1597026383085",
            "details": [
                {
                    "instId": "BTC-USD-230630",
                    "settlePx": "30250.5"
                },
                {
                    "instId": "BTC-USD-230929",
                    "settlePx": "30255.0"
                }
            ]
        });

        let settlement_history: SettlementHistory =
            serde_json::from_value(settlement_history_json).unwrap();
        assert_eq!(settlement_history.ts, "1597026383085");
        assert_eq!(settlement_history.details.len(), 2);
        assert_eq!(
            settlement_history.details.first().unwrap().inst_id,
            "BTC-USD-230630"
        );
        assert_eq!(
            settlement_history.details.first().unwrap().settle_px,
            "30250.5"
        );
        assert_eq!(
            settlement_history.details.get(1).unwrap().inst_id,
            "BTC-USD-230929"
        );
        assert_eq!(
            settlement_history.details.get(1).unwrap().settle_px,
            "30255.0"
        );
    }

    #[test]
    fn test_get_settlement_history_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ts": "1597026383085",
                    "details": [
                        {
                            "instId": "BTC-USD-230630",
                            "settlePx": "30250.5"
                        }
                    ]
                },
                {
                    "ts": "1597026483085",
                    "details": [
                        {
                            "instId": "ETH-USD-230630",
                            "settlePx": "1890.75"
                        }
                    ]
                }
            ]
        });

        let response: GetSettlementHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().ts, "1597026383085");
        assert_eq!(response.data.first().unwrap().details.len(), 1);
        assert_eq!(
            response
                .data
                .first()
                .unwrap()
                .details
                .first()
                .unwrap()
                .inst_id,
            "BTC-USD-230630"
        );
        assert_eq!(
            response
                .data
                .first()
                .unwrap()
                .details
                .first()
                .unwrap()
                .settle_px,
            "30250.5"
        );
        assert_eq!(response.data.get(1).unwrap().ts, "1597026483085");
        assert_eq!(
            response
                .data
                .get(1)
                .unwrap()
                .details
                .first()
                .unwrap()
                .inst_id,
            "ETH-USD-230630"
        );
        assert_eq!(
            response
                .data
                .get(1)
                .unwrap()
                .details
                .first()
                .unwrap()
                .settle_px,
            "1890.75"
        );
    }

    #[test]
    fn test_settlement_history_serialization_roundtrip() {
        let original = GetSettlementHistoryRequest {
            inst_family: "SOL-USD".to_string(),
            after: Some("1597026383085".to_string()),
            before: None,
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetSettlementHistoryRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_family, deserialized.inst_family);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.limit, deserialized.limit);
    }

    #[test]
    fn test_settlement_history_minimal_request() {
        let request = GetSettlementHistoryRequest {
            inst_family: "BTC-USD".to_string(),
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        // Optional fields should not be present when None
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
        assert!(serialized.get("limit").is_none());
    }
}
