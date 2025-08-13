use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_QUICK_MARGIN_BORROW_REPAY_HISTORY_ENDPOINT: &str =
    "api/v5/account/quick-margin-borrow-repay-history";

/// Request to get quick margin borrow repay history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetQuickMarginBorrowRepayHistoryRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Side: borrow, repay
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,

    /// Pagination of data to return records newer than the requested timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records earlier than the requested timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Filter with a begin timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// Filter with an end timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Number of results per request. The maximum is 100; the default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Quick margin borrow repay history record
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickMarginBorrowRepayHistory {
    /// Instrument ID
    pub inst_id: String,

    /// Currency
    pub ccy: String,

    /// Side: borrow, repay
    pub side: String,

    /// Amount
    pub acc_borrowed: String,

    /// Amount borrowed from quick margin
    pub borrowed: String,

    /// Interest
    pub interest: String,

    /// Timestamp
    pub ts: String,
}

impl RestClient {
    /// Get quick margin borrow repay history
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-borrow-and-repay-history-in-quick-margin-mode)
    ///
    /// # Arguments
    /// * `request` - The get quick margin borrow repay history request
    ///
    /// # Returns
    /// A result containing the quick margin borrow repay history or an error
    pub async fn get_quick_margin_borrow_repay_history(
        &self,
        request: &GetQuickMarginBorrowRepayHistoryRequest,
    ) -> RestResult<QuickMarginBorrowRepayHistory> {
        self.send_get_request(
            ACCOUNT_QUICK_MARGIN_BORROW_REPAY_HISTORY_ENDPOINT,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_quick_margin_borrow_repay_history_request_serialization() {
        let request = GetQuickMarginBorrowRepayHistoryRequest {
            inst_id: Some("BTC-USDT".to_string()),
            ccy: Some("BTC".to_string()),
            side: Some("borrow".to_string()),
            after: None,
            before: None,
            begin: None,
            end: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("side=borrow"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_quick_margin_borrow_repay_history_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "ccy": "BTC",
                    "side": "borrow",
                    "accBorrowed": "1.5",
                    "borrowed": "0.1",
                    "interest": "0.001",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: OkxApiResponse<QuickMarginBorrowRepayHistory> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let history = &response.data[0];
        assert_eq!(history.inst_id, "BTC-USDT");
        assert_eq!(history.ccy, "BTC");
        assert_eq!(history.side, "borrow");
        assert_eq!(history.acc_borrowed, "1.5");
        assert_eq!(history.borrowed, "0.1");
        assert_eq!(history.interest, "0.001");
        assert_eq!(history.ts, "1597026383085");
    }
}
