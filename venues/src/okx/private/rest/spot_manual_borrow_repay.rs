use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to spot manual borrow repay
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotManualBorrowRepayRequest {
    /// Currency
    pub ccy: String,

    /// Side: borrow, repay
    pub side: String,

    /// Amount
    pub amt: String,
}

/// Response for spot manual borrow repay
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotManualBorrowRepayResponse {
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
    /// Spot manual borrow repay
    ///
    /// # Arguments
    /// * `request` - The spot manual borrow repay request
    ///
    /// # Returns
    /// A result containing the spot manual borrow repay response or an error
    pub async fn spot_manual_borrow_repay(&self, request: &SpotManualBorrowRepayRequest) -> RestResult<OkxApiResponse<SpotManualBorrowRepayResponse>> {
        self.send_request(
            "api/v5/account/spot-manual-borrow-repay",
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
    fn test_spot_manual_borrow_repay_request_serialization() {
        let request = SpotManualBorrowRepayRequest {
            ccy: "BTC".to_string(),
            side: "borrow".to_string(),
            amt: "0.1".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"side\":\"borrow\""));
        assert!(json.contains("\"amt\":\"0.1\""));
    }

    #[test]
    fn test_spot_manual_borrow_repay_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "side": "borrow",
                    "amt": "0.1",
                    "result": true
                }
            ]
        }"#;

        let response: OkxApiResponse<SpotManualBorrowRepayResponse> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        match response.data.first() {
            Some(result) => {
                assert_eq!(result.ccy, "BTC");
                assert_eq!(result.side, "borrow");
                assert_eq!(result.amt, "0.1");
                assert!(result.result);
            }
            None => {
                unreachable!("Expected at least one result in response");
            }
        }
    }
}
