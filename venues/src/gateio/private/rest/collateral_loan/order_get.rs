use serde::Deserialize;

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/loan/collateral/orders";

/// Detailed collateral loan order information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralLoanOrderDetail {
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
    /// Query single collateral loan order details
    ///
    /// Returns detailed information for a single collateral loan order.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-5)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `order_id` - The order ID to query
    ///
    /// # Returns
    /// Detailed collateral loan order information
    pub async fn get_collateral_loan_order(
        &self,
        order_id: i64,
    ) -> RestResult<CollateralLoanOrderDetail> {
        let endpoint = format!("{}/{}", ORDERS_ENDPOINT, order_id);
        self.send_get_request::<CollateralLoanOrderDetail, ()>(&endpoint, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_deserialize_response() {
        let json = r#"{
            "order_id": 10000421,
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
        }"#;
        let resp: CollateralLoanOrderDetail = serde_json::from_str(json).unwrap();
        assert_eq!(resp.order_id, 10000421);
        assert_eq!(resp.collateral_currency, "BTC");
        assert_eq!(resp.borrow_currency, "USDT");
    }
}
