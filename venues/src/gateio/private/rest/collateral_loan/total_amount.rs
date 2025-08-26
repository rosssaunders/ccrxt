use serde::Deserialize;

use super::{RestClient, RestResult};

const TOTAL_AMOUNT_ENDPOINT: &str = "/loan/collateral/total_amount";

/// No parameters required for querying user's total borrowing and collateral amount.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-total-borrowing-and-collateral-amount)
#[derive(Debug, Clone, Default)]
pub struct QueryUserTotalBorrowingAndCollateralAmountRequest;

/// Response for user's total borrowing and collateral amount.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-total-borrowing-and-collateral-amount)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserTotalBorrowingAndCollateralAmountResponse {
    /// Total borrowing amount in USDT
    pub borrow_amount: String,

    /// Total collateral amount in USDT
    pub collateral_amount: String,
}

impl RestClient {
    /// Query user's total borrowing and collateral amount
    ///
    /// Returns user's total borrowing and collateral amount.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-total-borrowing-and-collateral-amount)
    ///
    /// # Returns
    /// User's total borrowing and collateral amount response
    pub async fn get_user_total_borrowing_and_collateral_amount(
        &self,
        _request: QueryUserTotalBorrowingAndCollateralAmountRequest,
    ) -> RestResult<UserTotalBorrowingAndCollateralAmountResponse> {
        self.send_get_request::<UserTotalBorrowingAndCollateralAmountResponse, ()>(
            TOTAL_AMOUNT_ENDPOINT,
            None,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_response() {
        let json = r#"{ "borrow_amount": "11", "collateral_amount": "111" }"#;
        let resp: UserTotalBorrowingAndCollateralAmountResponse =
            serde_json::from_str(json).unwrap();
        assert_eq!(resp.borrow_amount, "11");
        assert_eq!(resp.collateral_amount, "111");
    }
}
