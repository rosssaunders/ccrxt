use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting interest limits
const ACCOUNT_INTEREST_LIMITS_ENDPOINT: &str = "api/v5/account/interest-limits";

/// Request parameters for getting interest limits
#[derive(Debug, Clone, Serialize)]
pub struct GetInterestLimitsRequest {
    /// Instrument type
    /// MARGIN: Spot and margin
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,

    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Interest limits information
#[derive(Debug, Clone, Deserialize)]
pub struct InterestLimits {
    /// Debt amount, In the unit of currency
    pub debt: String,

    /// Interest, In the unit of currency
    pub interest: String,

    /// Next discount time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "nextDiscountTime")]
    pub next_discount_time: String,

    /// Next interest time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "nextInterestTime")]
    pub next_interest_time: String,

    /// Loan allocation
    #[serde(rename = "loanAlloc")]
    pub loan_alloc: String,

    /// Records
    pub records: Vec<InterestLimitRecord>,
}

/// Interest limit record
#[derive(Debug, Clone, Deserialize)]
pub struct InterestLimitRecord {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Interest rate
    pub rate: String,

    /// Loan quota, In the unit of currency
    #[serde(rename = "loanQuota")]
    pub loan_quota: String,

    /// Surplus limit after borrowing, In the unit of currency
    #[serde(rename = "surplusLmt")]
    pub surplus_lmt: String,

    /// Surplus used by user, In the unit of currency
    #[serde(rename = "surplusUsed")]
    pub surplus_used: String,

    /// Max borrowable
    #[serde(rename = "maxLoan")]
    pub max_loan: String,

    /// Interest, In the unit of currency
    pub interest: String,

    /// Position loan, In the unit of currency
    #[serde(rename = "posLoan")]
    pub pos_loan: String,

    /// Available loan, In the unit of currency
    #[serde(rename = "availLoan")]
    pub avail_loan: String,

    /// Used loan, In the unit of currency
    #[serde(rename = "usedLoan")]
    pub used_loan: String,

    /// Average interest rate
    #[serde(rename = "avgRate")]
    pub avg_rate: String,
}

impl RestClient {
    /// Get interest limits (trading account)
    ///
    /// Get the user's leveraged currency borrowing interest rate and limit for trading account.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-interest-limits)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The interest limits request parameters
    ///
    /// # Returns
    /// A result containing the interest limits information
    pub async fn get_trading_interest_limits(
        &self,
        request: GetInterestLimitsRequest,
    ) -> RestResult<InterestLimits> {
        self.send_get_request(
            ACCOUNT_INTEREST_LIMITS_ENDPOINT,
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
    fn test_get_interest_limits_request_serialization() {
        let request = GetInterestLimitsRequest {
            inst_type: Some("MARGIN".to_string()),
            ccy: Some("BTC".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"type\":\"MARGIN\""));
        assert!(json.contains("\"ccy\":\"BTC\""));
    }

    #[test]
    fn test_interest_limits_deserialization() {
        let limits_json = json!({
            "debt": "100.5",
            "interest": "0.5",
            "nextDiscountTime": "1597026383085",
            "nextInterestTime": "1597026383085",
            "loanAlloc": "10000",
            "records": [
                {
                    "ccy": "BTC",
                    "rate": "0.00098",
                    "loanQuota": "1000",
                    "surplusLmt": "500",
                    "surplusUsed": "100",
                    "maxLoan": "400",
                    "interest": "0.1",
                    "posLoan": "50",
                    "availLoan": "350",
                    "usedLoan": "50",
                    "avgRate": "0.00095"
                }
            ]
        });

        let limits: InterestLimits = serde_json::from_value(limits_json).unwrap();
        assert_eq!(limits.debt, "100.5");
        assert_eq!(limits.interest, "0.5");
        assert_eq!(limits.records.len(), 1);
        assert_eq!(limits.records[0].ccy, "BTC");
        assert_eq!(limits.records[0].rate, "0.00098");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "debt": "100.5",
                    "interest": "0.5",
                    "nextDiscountTime": "1597026383085",
                    "nextInterestTime": "1597026383085",
                    "loanAlloc": "10000",
                    "records": []
                }
            ]
        });

        let response: ApiResponse<InterestLimits> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].debt, "100.5");
    }
}
