use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ORDERS_ENDPOINT: &str = "/loan/collateral/orders";

/// Request parameters for placing a collateral loan order.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#place-collateral-loan-order)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCollateralLoanOrderRequest {
    /// Collateral amount (string decimal, required)
    pub collateral_amount: String,

    /// Collateral currency (required)
    pub collateral_currency: String,

    /// Borrowed amount (string decimal, required)
    pub borrow_amount: String,

    /// Borrowed currency (required)
    pub borrow_currency: String,
}

/// Response for a successfully placed collateral loan order.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#place-collateral-loan-order)
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCollateralLoanOrderResponse {
    /// Order ID
    pub order_id: i64,
}

impl RestClient {
    /// Place collateral loan order
    ///
    /// Places a new collateral-backed loan order.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#place-collateral-loan-order)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - Parameters for placing the collateral loan order
    ///
    /// # Returns
    /// The created order ID
    pub async fn create_collateral_loan_order(
        &self,
        request: CreateCollateralLoanOrderRequest,
    ) -> RestResult<CreateCollateralLoanOrderResponse> {
        self.send_post_request(ORDERS_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_request() {
        let req = CreateCollateralLoanOrderRequest {
            collateral_amount: "1.0".to_string(),
            collateral_currency: "BTC".to_string(),
            borrow_amount: "1000".to_string(),
            borrow_currency: "USDT".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("collateral_amount"));
        assert!(json.contains("collateral_currency"));
        assert!(json.contains("borrow_amount"));
        assert!(json.contains("borrow_currency"));
    }

    #[test]
    fn test_deserialize_response() {
        let json = r#"{ "order_id": 10005578 }"#;
        let resp: CreateCollateralLoanOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.order_id, 10005578);
    }
}
