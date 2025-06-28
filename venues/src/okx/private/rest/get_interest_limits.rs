use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request to get interest limits
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestLimitsRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Interest limits information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestLimits {
    /// Currency
    pub ccy: String,

    /// Amount limit for loan in tier 1
    pub loan_quota: String,

    /// Used loan amount in USD value
    pub used_loan: String,

    /// Remaining loan amount in USD value
    pub remaining_loan: String,

    /// Interest rate for Spot/Futures trading
    pub interest: String,

    /// Rate for the next tier
    pub next_discount_time: String,

    /// Interest rate discount applicable quantity in USD
    pub next_interest_time: String,

    /// Level
    pub loan_quota_ccy: String,

    /// Whether currency can be borrowed in spot
    pub spot_in_use_amt: Option<String>,

    /// Borrowed amount for Multi-currency margin
    pub spot_loan_quota: Option<String>,

    /// Used loan amount for Multi-currency margin
    pub spot_used_loan: Option<String>,

    /// Remaining loan amount for Multi-currency margin
    pub spot_remaining_loan: Option<String>,
}

impl RestClient {
    /// Get interest limits
    ///
    /// # Arguments
    /// * `request` - The get interest limits request
    ///
    /// # Returns
    /// A result containing the interest limits or an error
    pub async fn get_interest_limits(
        &self,
        request: &GetInterestLimitsRequest,
    ) -> RestResult<OkxApiResponse<InterestLimits>> {
        self.send_request(
            "api/v5/account/interest-limits",
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
    fn test_get_interest_limits_request_serialization() {
        let request = GetInterestLimitsRequest {
            inst_type: Some(InstrumentType::Spot),
            ccy: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("ccy=BTC"));
    }

    #[test]
    fn test_interest_limits_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "loanQuota": "1000",
                    "usedLoan": "100",
                    "remainingLoan": "900",
                    "interest": "0.0001",
                    "nextDiscountTime": "1597026383085",
                    "nextInterestTime": "1597026383085",
                    "loanQuotaCcy": "USD",
                    "spotInUseAmt": "50",
                    "spotLoanQuota": "2000",
                    "spotUsedLoan": "200",
                    "spotRemainingLoan": "1800"
                }
            ]
        }"#;

        let response: OkxApiResponse<InterestLimits> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let limits = &response.data[0];
        assert_eq!(limits.ccy, "BTC");
        assert_eq!(limits.loan_quota, "1000");
        assert_eq!(limits.used_loan, "100");
        assert_eq!(limits.remaining_loan, "900");
        assert_eq!(limits.interest, "0.0001");
    }
}
