use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting maximum loan amount
const ACCOUNT_MAX_LOAN_ENDPOINT: &str = "api/v5/account/max-loan";

/// Request parameters for getting maximum loan amount
#[derive(Debug, Clone, Serialize)]
pub struct GetMaxLoanRequest {
    /// Instrument ID, e.g. BTC-USDT
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Margin mode
    /// cross: cross margin
    /// isolated: isolated margin
    #[serde(rename = "mgnMode")]
    pub mgn_mode: String,

    /// Margin currency
    /// Only applicable to cross MARGIN orders in Spot and futures mode.
    #[serde(rename = "mgnCcy", skip_serializing_if = "Option::is_none")]
    pub mgn_ccy: Option<String>,
}

/// Maximum loan amount information
#[derive(Debug, Clone, Deserialize)]
pub struct MaxLoan {
    /// Instrument ID, e.g. BTC-USDT
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Margin mode
    #[serde(rename = "mgnMode")]
    pub mgn_mode: String,

    /// Margin currency
    #[serde(rename = "mgnCcy")]
    pub mgn_ccy: String,

    /// Max loan of base currency
    #[serde(rename = "maxLoan")]
    pub max_loan: String,

    /// Currency of base currency max loan
    pub ccy: String,

    /// Side
    pub side: String,
}

impl RestClient {
    /// Get maximum loan amount (trading account)
    ///
    /// Get the maximum loan amount for trading account.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-maximum-loan-of-instrument)
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The maximum loan request parameters
    ///
    /// # Returns
    /// A result containing the maximum loan amount information
    pub async fn get_trading_max_loan(&self, request: GetMaxLoanRequest) -> RestResult<MaxLoan> {
        self.send_get_request(
            ACCOUNT_MAX_LOAN_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_max_loan_request_serialization() {
        let request = GetMaxLoanRequest {
            inst_id: "BTC-USDT".to_string(),
            mgn_mode: "cross".to_string(),
            mgn_ccy: Some("USDT".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"mgnMode\":\"cross\""));
        assert!(json.contains("\"mgnCcy\":\"USDT\""));
    }

    #[test]
    fn test_max_loan_deserialization() {
        let max_loan_json = json!({
            "instId": "BTC-USDT",
            "mgnMode": "cross",
            "mgnCcy": "USDT",
            "maxLoan": "10000",
            "ccy": "USDT",
            "side": "buy"
        });

        let max_loan: MaxLoan = serde_json::from_value(max_loan_json).unwrap();
        assert_eq!(max_loan.inst_id, "BTC-USDT");
        assert_eq!(max_loan.mgn_mode, "cross");
        assert_eq!(max_loan.mgn_ccy, "USDT");
        assert_eq!(max_loan.max_loan, "10000");
        assert_eq!(max_loan.ccy, "USDT");
        assert_eq!(max_loan.side, "buy");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "mgnMode": "cross",
                    "mgnCcy": "USDT",
                    "maxLoan": "10000",
                    "ccy": "USDT",
                    "side": "buy"
                }
            ]
        });

        let response: ApiResponse<MaxLoan> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].inst_id, "BTC-USDT");
    }
}
