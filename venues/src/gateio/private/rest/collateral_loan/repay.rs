use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const REPAY_ENDPOINT: &str = "/loan/collateral/repay";

/// Request to repay a collateral loan order.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#collateral-loan-repayment)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralLoanRepayRequest {
    /// Order ID (required)
    pub order_id: i64,

    /// Repayment amount (required for partial repayment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_amount: Option<String>,

    /// Repayment method: true for full repayment, false for partial (required)
    pub repaid_all: bool,
}

/// Response for collateral loan repayment.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#collateral-loan-repayment)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralLoanRepayResponse {
    /// Principal repaid
    pub repaid_principal: String,

    /// Interest repaid
    pub repaid_interest: String,
}

impl RestClient {
    /// Collateral loan repayment
    ///
    /// Repay a collateral loan order.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#collateral-loan-repayment)
    ///
    /// # Arguments
    /// * `request` - Parameters for collateral loan repayment
    ///
    /// # Returns
    /// Repayment result
    pub async fn repay_collateral_loan(
        &self,
        request: CollateralLoanRepayRequest,
    ) -> RestResult<CollateralLoanRepayResponse> {
        self.send_post_request(REPAY_ENDPOINT, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_full_repay_request() {
        let req = CollateralLoanRepayRequest {
            order_id: 37438962,
            repay_amount: None,
            repaid_all: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("order_id"));
        assert!(json.contains("repaid_all"));
        assert!(!json.contains("repay_amount"));
    }

    #[test]
    fn test_serialize_partial_repay_request() {
        let req = CollateralLoanRepayRequest {
            order_id: 37438962,
            repay_amount: Some("100".to_string()),
            repaid_all: false,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("repay_amount"));
        assert!(json.contains("repaid_all"));
    }

    #[test]
    fn test_deserialize_response() {
        let json = r#"{ "repaid_principal": "11", "repaid_interest": "111" }"#;
        let resp: CollateralLoanRepayResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.repaid_principal, "11");
        assert_eq!(resp.repaid_interest, "111");
    }
}
