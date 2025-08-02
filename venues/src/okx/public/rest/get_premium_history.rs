use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const PUBLIC_PREMIUM_HISTORY_ENDPOINT: &str = "api/v5/public/premium-history";
/// Request parameters for getting premium history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPremiumHistoryRequest {
    /// Instrument ID, e.g. "BTC-USDT-SWAP". Applicable to SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,
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

/// Individual premium history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumHistory {
    /// Instrument ID, e.g. "BTC-USDT-SWAP"
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Premium index. Formula: [(Best bid + Best ask) / 2 – Index price] / Index price
    pub premium: String,
    /// Data generation time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    pub ts: String,
}

/// Response for getting premium history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPremiumHistoryResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Premium history data
    pub data: Vec<PremiumHistory>,
}

impl RestClient {
    /// Get premium history
    ///
    /// Returns premium data in the past 6 months.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-premium-history
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The premium history request parameters
    ///
    /// # Returns
    /// Response containing the list of premium history entries
    pub async fn get_premium_history(
        &self,
        request: &GetPremiumHistoryRequest,
    ) -> RestResult<GetPremiumHistoryResponse> {
        self.send_request(
            PUBLIC_PREMIUM_HISTORY_ENDPOINT,
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
    fn test_get_premium_history_request_structure() {
        let request = GetPremiumHistoryRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USDT-SWAP")
        );
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("50"));
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
    }

    #[test]
    fn test_get_premium_history_request_with_pagination() {
        let request = GetPremiumHistoryRequest {
            inst_id: "ETH-USDT-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: Some("1597026483085".to_string()),
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("ETH-USDT-SWAP")
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
    fn test_premium_history_structure() {
        let premium_history_json = json!({
            "instId": "BTC-USDT-SWAP",
            "premium": "0.000123",
            "ts": "1597026383085"
        });

        let premium_history: PremiumHistory = serde_json::from_value(premium_history_json).unwrap();
        assert_eq!(premium_history.inst_id, "BTC-USDT-SWAP");
        assert_eq!(premium_history.premium, "0.000123");
        assert_eq!(premium_history.ts, "1597026383085");
    }

    #[test]
    fn test_get_premium_history_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "premium": "0.000123",
                    "ts": "1597026383085"
                },
                {
                    "instId": "BTC-USDT-SWAP",
                    "premium": "0.000456",
                    "ts": "1597026383086"
                }
            ]
        });

        let response: GetPremiumHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USDT-SWAP");
        assert_eq!(response.data.first().unwrap().premium, "0.000123");
        assert_eq!(response.data.first().unwrap().ts, "1597026383085");
        assert_eq!(response.data.get(1).unwrap().premium, "0.000456");
        assert_eq!(response.data.get(1).unwrap().ts, "1597026383086");
    }

    #[test]
    fn test_premium_history_serialization_roundtrip() {
        let original = GetPremiumHistoryRequest {
            inst_id: "SOL-USDT-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: None,
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetPremiumHistoryRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.limit, deserialized.limit);
    }

    #[test]
    fn test_premium_history_minimal_request() {
        let request = GetPremiumHistoryRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USDT-SWAP")
        );
        // Optional fields should not be present when None
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
        assert!(serialized.get("limit").is_none());
    }
}
