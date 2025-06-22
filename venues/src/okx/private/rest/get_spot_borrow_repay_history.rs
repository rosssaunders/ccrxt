use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to get spot borrow repay history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpotBorrowRepayHistoryRequest {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Pagination of data to return records newer than the requested timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records earlier than the requested timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 100; the default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Spot borrow repay history record
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotBorrowRepayHistory {
    /// Currency
    pub ccy: String,

    /// Side: borrow, repay
    pub side: String,

    /// Amount
    pub amt: String,

    /// Interest
    pub interest: String,

    /// Timestamp
    pub ts: String,
}

impl RestClient {
    /// Get spot borrow repay history
    ///
    /// # Arguments
    /// * `request` - The get spot borrow repay history request
    ///
    /// # Returns
    /// A result containing the spot borrow repay history or an error
    pub async fn get_spot_borrow_repay_history(&self, request: &GetSpotBorrowRepayHistoryRequest) -> RestResult<OkxApiResponse<SpotBorrowRepayHistory>> {
        self.send_request(
            "api/v5/account/spot-borrow-repay-history",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_spot_borrow_repay_history_request_serialization() {
        let request = GetSpotBorrowRepayHistoryRequest {
            ccy: Some("BTC".to_string()),
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_spot_borrow_repay_history_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "side": "borrow",
                    "amt": "0.1",
                    "interest": "0.001",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: OkxApiResponse<SpotBorrowRepayHistory> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let history = &response.data[0];
        assert_eq!(history.ccy, "BTC");
        assert_eq!(history.side, "borrow");
        assert_eq!(history.amt, "0.1");
        assert_eq!(history.interest, "0.001");
        assert_eq!(history.ts, "1597026383085");
    }
}
