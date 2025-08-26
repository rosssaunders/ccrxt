use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/loan/collateral/orders";

/// Query parameters for listing collateral loan orders.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-order-list)
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ListCollateralLoanOrdersRequest {
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Maximum number of records returned in a single list (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Collateral currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_currency: Option<String>,

    /// Borrowed currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_currency: Option<String>,
}

/// Collateral loan order summary item.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-order-list)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralLoanOrderSummary {
    /// Order ID
    pub order_id: i64,

    /// Collateral currency
    pub collateral_currency: String,

    /// Collateral amount
    pub collateral_amount: String,

    /// Borrowed currency
    pub borrow_currency: String,

    /// Borrowed amount
    pub borrow_amount: String,

    /// Repaid amount
    pub repaid_amount: String,

    /// Repaid principal
    pub repaid_principal: String,

    /// Repaid interest
    pub repaid_interest: String,

    /// Initial collateralization rate
    pub init_ltv: String,

    /// Current collateralization rate
    pub current_ltv: String,

    /// Liquidation collateralization rate
    pub liquidate_ltv: String,

    /// Order status
    pub status: String,

    /// Borrowing time (timestamp in seconds)
    pub borrow_time: i64,

    /// Outstanding principal and interest
    pub left_repay_total: String,

    /// Outstanding principal
    pub left_repay_principal: String,

    /// Outstanding interest
    pub left_repay_interest: String,
}

impl RestClient {
    /// Query collateral loan order list
    ///
    /// Returns a list of collateral loan orders for the authenticated user.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-order-list)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - Query parameters for listing collateral loan orders
    ///
    /// # Returns
    /// List of collateral loan order summary items
    pub async fn list_collateral_loan_orders(
        &self,
        request: ListCollateralLoanOrdersRequest,
    ) -> RestResult<Vec<CollateralLoanOrderSummary>> {
        self.send_get_request(ORDERS_ENDPOINT, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_request() {
        let req = ListCollateralLoanOrdersRequest {
            page: Some(1),
            limit: Some(20),
            collateral_currency: Some("BTC".to_string()),
            borrow_currency: Some("USDT".to_string()),
        };
        let qs = serde_urlencoded::to_string(&req).unwrap();
        assert!(qs.contains("page=1"));
        assert!(qs.contains("limit=20"));
        assert!(qs.contains("collateral_currency=BTC"));
        assert!(qs.contains("borrow_currency=USDT"));
    }

    #[test]
    fn test_deserialize_response() {
        let json = r#"[
            {
                "order_id": 10005578,
                "collateral_currency": "BTC",
                "collateral_amount": "1.0",
                "borrow_currency": "USDT",
                "borrow_amount": "1000",
                "repaid_amount": "0",
                "repaid_principal": "0",
                "repaid_interest": "0",
                "init_ltv": "0.5",
                "current_ltv": "0.6",
                "liquidate_ltv": "0.8",
                "status": "initial",
                "borrow_time": 1620000000,
                "left_repay_total": "1000",
                "left_repay_principal": "1000",
                "left_repay_interest": "0"
            }
        ]"#;
        let resp: Vec<CollateralLoanOrderSummary> = serde_json::from_str(json).unwrap();
        assert_eq!(resp[0].order_id, 10005578);
        assert_eq!(resp[0].collateral_currency, "BTC");
        assert_eq!(resp[0].borrow_currency, "USDT");
    }
}
