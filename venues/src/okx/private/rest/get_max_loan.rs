use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};


const ACCOUNT_MAX_LOAN_ENDPOINT: &str = "api/v5/account/max-loan";
/// Request to get max loan
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxLoanRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Margin mode
    /// "cross", "isolated"
    pub mgn_mode: String,

    /// Margin currency
    /// Only applicable to cross MARGIN orders in Futures and Swap mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_ccy: Option<String>,
}

/// Max loan details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxLoan {
    /// Instrument ID
    pub inst_id: String,

    /// Margin mode
    pub mgn_mode: String,

    /// Margin currency
    pub mgn_ccy: String,

    /// Max loan amount
    pub max_loan: String,

    /// Currency
    pub ccy: String,

    /// Side
    pub side: String,
}

impl RestClient {
    /// Get max loan
    ///
    /// # Arguments
    /// * `request` - The get max loan request
    ///
    /// # Returns
    /// A result containing the max loan or an error
    pub async fn get_max_loan(
        &self,
        request: &GetMaxLoanRequest,
    ) -> RestResult<OkxApiResponse<MaxLoan>> {
        self.send_request(
            ACCOUNT_MAX_LOAN_ENDPOINT,
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
    fn test_get_max_loan_request_serialization() {
        let request = GetMaxLoanRequest {
            inst_id: "BTC-USDT".to_string(),
            mgn_mode: "cross".to_string(),
            mgn_ccy: Some("USDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("mgnMode=cross"));
        assert!(serialized.contains("mgnCcy=USDT"));
    }

    #[test]
    fn test_get_max_loan_minimal_request() {
        let request = GetMaxLoanRequest {
            inst_id: "BTC-USDT".to_string(),
            mgn_mode: "isolated".to_string(),
            mgn_ccy: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("mgnMode=isolated"));
        assert!(!serialized.contains("mgnCcy="));
    }

    #[test]
    fn test_max_loan_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "mgnMode": "cross",
                    "mgnCcy": "USDT",
                    "maxLoan": "1000.5",
                    "ccy": "BTC",
                    "side": "buy"
                }
            ]
        }"#;

        let response: OkxApiResponse<MaxLoan> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let max_loan = &response.data[0];
        assert_eq!(max_loan.inst_id, "BTC-USDT");
        assert_eq!(max_loan.mgn_mode, "cross");
        assert_eq!(max_loan.max_loan, "1000.5");
        assert_eq!(max_loan.side, "buy");
    }
}
