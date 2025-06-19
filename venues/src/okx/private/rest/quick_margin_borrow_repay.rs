use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to quick margin borrow repay
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickMarginBorrowRepayRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Currency
    pub ccy: String,

    /// Side: borrow, repay
    pub side: String,

    /// Amount
    pub amt: String,
}

/// Response for quick margin borrow repay
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickMarginBorrowRepayResponse {
    /// Instrument ID
    pub inst_id: String,

    /// Currency
    pub ccy: String,

    /// Side: borrow, repay
    pub side: String,

    /// Amount
    pub amt: String,

    /// Result
    pub result: bool,
}

impl RestClient {
    /// Quick margin borrow repay
    ///
    /// # Arguments
    /// * `request` - The quick margin borrow repay request
    ///
    /// # Returns
    /// A result containing the quick margin borrow repay response or an error
    pub async fn quick_margin_borrow_repay(
        &self,
        request: &QuickMarginBorrowRepayRequest,
    ) -> RestResult<OkxApiResponse<QuickMarginBorrowRepayResponse>> {
        self.send_request(
            "api/v5/account/quick-margin-borrow-repay",
            reqwest::Method::POST,
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
    fn test_quick_margin_borrow_repay_request_serialization() {
        let request = QuickMarginBorrowRepayRequest {
            inst_id: "BTC-USDT".to_string(),
            ccy: "BTC".to_string(),
            side: "borrow".to_string(),
            amt: "0.1".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"side\":\"borrow\""));
        assert!(json.contains("\"amt\":\"0.1\""));
    }

    #[test]
    fn test_quick_margin_borrow_repay_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "ccy": "BTC",
                    "side": "borrow",
                    "amt": "0.1",
                    "result": true
                }
            ]
        }"#;

        let response: OkxApiResponse<QuickMarginBorrowRepayResponse> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = &response.data[0];
        assert_eq!(result.inst_id, "BTC-USDT");
        assert_eq!(result.ccy, "BTC");
        assert_eq!(result.side, "borrow");
        assert_eq!(result.amt, "0.1");
        assert!(result.result);
    }
}