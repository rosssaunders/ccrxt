use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const PUBLIC_BORROW_HISTORY_ENDPOINT: &str = "/api/v5/finance/savings/lending-rate-history";

/// Request parameters for public borrow history
#[derive(Debug, Clone, Serialize)]
pub struct GetPublicBorrowHistoryRequest {
    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Pagination of data to return records earlier than the requested ts,
    /// Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ts,
    /// Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100.
    /// If ccy is not specified, all data under the same ts will be returned, not limited by limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response data for public borrow history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicBorrowHistoryData {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Lending amount (deprecated)
    pub amt: String,

    /// Lending annual interest rate
    pub rate: String,

    /// Time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: String,
}

impl RestClient {
    /// Get public borrow history
    ///
    /// Public endpoint that retrieves lending rate history. Only returns records after December 14, 2021.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-simple-earn-flexible-get-public-borrow-history-public)
    ///
    /// Rate limit: 6 requests per second
    /// Rate limit rule: IP
    ///
    /// # Arguments
    /// * `request` - Request parameters including optional currency filter and pagination
    ///
    /// # Returns
    /// A vector of public borrow history data
    pub async fn get_public_borrow_history(
        &self,
        request: GetPublicBorrowHistoryRequest,
    ) -> RestResult<Vec<PublicBorrowHistoryData>> {
        self.send_get_request(
            PUBLIC_BORROW_HISTORY_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_public_borrow_history_request_serialization() {
        let request = GetPublicBorrowHistoryRequest {
            ccy: Some("BTC".to_string()),
            after: Some("1597026383085".to_string()),
            before: Some("1597112783085".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("after=1597026383085"));
        assert!(serialized.contains("before=1597112783085"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_public_borrow_history_request_minimal() {
        let request = GetPublicBorrowHistoryRequest {
            ccy: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_public_borrow_history_data_serialization() {
        let data = PublicBorrowHistoryData {
            ccy: "BTC".to_string(),
            amt: "100.5".to_string(),
            rate: "0.05".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: PublicBorrowHistoryData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_public_borrow_history_data_deserialization_from_api() {
        let json_response = r#"[
            {
                "ccy": "BTC",
                "amt": "150.0",
                "rate": "0.045",
                "ts": "1597026383085"
            },
            {
                "ccy": "ETH", 
                "amt": "3000.0",
                "rate": "0.038",
                "ts": "1597112783085"
            }
        ]"#;

        let data: Vec<PublicBorrowHistoryData> = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].ccy, "BTC");
        assert_eq!(data[0].rate, "0.045");
        assert_eq!(data[0].ts, "1597026383085");
        assert_eq!(data[1].ccy, "ETH");
        assert_eq!(data[1].rate, "0.038");
        assert_eq!(data[1].ts, "1597112783085");
    }

    #[test]
    fn test_public_borrow_history_data_empty_array() {
        let json_response = r#"[]"#;

        let data: Vec<PublicBorrowHistoryData> = serde_json::from_str(json_response).unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_get_public_borrow_history_request_limit_defaults() {
        let request = GetPublicBorrowHistoryRequest {
            ccy: Some("ETH".to_string()),
            after: None,
            before: None,
            limit: Some("100".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=ETH"));
        assert!(serialized.contains("limit=100"));
    }
}
