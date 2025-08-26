use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const REPAY_RECORDS_ENDPOINT: &str = "/loan/collateral/repay_records";

/// Query parameters for listing collateral loan repayment records.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-repayment-records)
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ListCollateralLoanRepayRecordsRequest {
    /// Operation type: "repay" for regular repayment, "liquidate" for liquidation (required)
    pub source: String,

    /// Borrowed currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_currency: Option<String>,

    /// Collateral currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_currency: Option<String>,

    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Maximum number of records returned in a single list (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start timestamp for the query (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End timestamp for the query (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Collateral loan repayment record item.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-repayment-records)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralLoanRepayRecord {
    /// Order ID
    pub order_id: i64,

    /// Repayment record ID
    pub record_id: i64,

    /// Repayment amount
    pub repaid_amount: String,

    /// Borrowed currency
    pub borrow_currency: String,

    /// Collateral currency
    pub collateral_currency: String,

    /// Initial collateralization rate
    pub init_ltv: String,

    /// Borrowing time (timestamp)
    pub borrow_time: i64,

    /// Repayment time (timestamp)
    pub repay_time: i64,

    /// Total interest
    pub total_interest: String,

    /// Principal to be repaid before repayment
    pub before_left_principal: String,

    /// Principal to be repaid after repayment
    pub after_left_principal: String,

    /// Collateral amount before repayment
    pub before_left_collateral: String,

    /// Collateral amount after repayment
    pub after_left_collateral: String,
}

impl RestClient {
    /// Query collateral loan repayment records
    ///
    /// Returns a list of collateral loan repayment records for the authenticated user.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-repayment-records)
    ///
    /// # Arguments
    /// * `request` - Query parameters for listing collateral loan repayment records
    ///
    /// # Returns
    /// List of collateral loan repayment record items
    pub async fn list_collateral_loan_repay_records(
        &self,
        request: ListCollateralLoanRepayRecordsRequest,
    ) -> RestResult<Vec<CollateralLoanRepayRecord>> {
        self.send_get_request(REPAY_RECORDS_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = ListCollateralLoanRepayRecordsRequest {
            source: "repay".to_string(),
            borrow_currency: Some("USDT".to_string()),
            collateral_currency: Some("BTC".to_string()),
            page: Some(1),
            limit: Some(20),
            from: Some(1620000000),
            to: Some(1620003600),
        };
        let qs = serde_urlencoded::to_string(&req).unwrap();
        assert!(qs.contains("source=repay"));
        assert!(qs.contains("borrow_currency=USDT"));
        assert!(qs.contains("collateral_currency=BTC"));
        assert!(qs.contains("page=1"));
        assert!(qs.contains("limit=20"));
        assert!(qs.contains("from=1620000000"));
        assert!(qs.contains("to=1620003600"));
    }

    #[test]
    fn test_deserialize_response() {
        let json = r#"[
            {
                "order_id": 10005578,
                "record_id": 1,
                "repaid_amount": "100",
                "borrow_currency": "USDT",
                "collateral_currency": "BTC",
                "init_ltv": "0.5",
                "borrow_time": 1620000000,
                "repay_time": 1620003600,
                "total_interest": "10",
                "before_left_principal": "1000",
                "after_left_principal": "900",
                "before_left_collateral": "1.0",
                "after_left_collateral": "0.9"
            }
        ]"#;
        let resp: Vec<CollateralLoanRepayRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(resp[0].order_id, 10005578);
        assert_eq!(resp[0].collateral_currency, "BTC");
        assert_eq!(resp[0].borrow_currency, "USDT");
    }
}
