use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const COLLATERALS_ENDPOINT: &str = "/loan/collateral/collaterals";

/// Query parameters for listing collateral adjustment records.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-adjustment-records)
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ListCollateralAdjustmentRecordsRequest {
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

    /// Borrowed currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_currency: Option<String>,

    /// Collateral currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_currency: Option<String>,
}

/// Collateral adjustment record item.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-adjustment-records)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralAdjustmentRecord {
    /// Order ID
    pub order_id: i64,

    /// Collateral record ID
    pub record_id: i64,

    /// Borrowed currency
    pub borrow_currency: String,

    /// Borrowed amount
    pub borrow_amount: String,

    /// Collateral currency
    pub collateral_currency: String,

    /// Collateral amount before adjustment
    pub before_collateral: String,

    /// Collateral amount after adjustment
    pub after_collateral: String,

    /// Collateral ratio before adjustment
    pub before_ltv: String,

    /// Collateral ratio after adjustment
    pub after_ltv: String,

    /// Operation time (timestamp in seconds)
    pub operate_time: i64,
}

impl RestClient {
    /// Query collateral adjustment records
    ///
    /// Returns a list of collateral adjustment records for the authenticated user.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-adjustment-records)
    ///
    /// # Arguments
    /// * `request` - Query parameters for listing collateral adjustment records
    ///
    /// # Returns
    /// List of collateral adjustment record items
    pub async fn list_collateral_adjustment_records(
        &self,
        request: ListCollateralAdjustmentRecordsRequest,
    ) -> RestResult<Vec<CollateralAdjustmentRecord>> {
        self.send_get_request(COLLATERALS_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_request() {
        let req = ListCollateralAdjustmentRecordsRequest {
            page: Some(1),
            limit: Some(20),
            from: Some(1620000000),
            to: Some(1620003600),
            borrow_currency: Some("USDT".to_string()),
            collateral_currency: Some("BTC".to_string()),
        };
        let qs = serde_urlencoded::to_string(&req).unwrap();
        assert!(qs.contains("page=1"));
        assert!(qs.contains("limit=20"));
        assert!(qs.contains("from=1620000000"));
        assert!(qs.contains("to=1620003600"));
        assert!(qs.contains("borrow_currency=USDT"));
        assert!(qs.contains("collateral_currency=BTC"));
    }

    #[test]
    fn test_deserialize_response() {
        let json = r#"[
            {
                "order_id": 10005578,
                "record_id": 1,
                "borrow_currency": "USDT",
                "borrow_amount": "1000",
                "collateral_currency": "BTC",
                "before_collateral": "1.0",
                "after_collateral": "1.5",
                "before_ltv": "0.5",
                "after_ltv": "0.4",
                "operate_time": 1620000000
            }
        ]"#;
        let resp: Vec<CollateralAdjustmentRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(resp[0].order_id, 10005578);
        assert_eq!(resp[0].collateral_currency, "BTC");
        assert_eq!(resp[0].borrow_currency, "USDT");
    }
}
