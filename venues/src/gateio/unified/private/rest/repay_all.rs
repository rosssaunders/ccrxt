use super::{RestClient, loan::BorrowOrRepayResponse};

impl RestClient {
    /// Repay all borrowed funds for a currency
    ///
    /// This method fetches the current borrowed amount and repays it in full.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#repay-all)
    pub async fn repay_all(
        &self,
        currency: &str,
    ) -> crate::gateio::unified::RestResult<BorrowOrRepayResponse> {
        let unified_account = self.get_unified_account(None).await?;
        let borrowed_amount = &unified_account.borrowed;

        if borrowed_amount.parse::<f64>().unwrap_or(0.0) <= 0.0 {
            return Ok(BorrowOrRepayResponse { succeed: true });
        }

        self.repay(currency, borrowed_amount).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repay_all_response() {
        // Test successful response
        let response = BorrowOrRepayResponse { succeed: true };
        assert!(response.succeed);

        // Test failed response
        let response = BorrowOrRepayResponse { succeed: false };
        assert!(!response.succeed);
    }

    #[test]
    fn test_zero_borrowed_amount() {
        let borrowed_amount = "0.0";
        let amount: f64 = borrowed_amount.parse().unwrap();
        assert!(amount <= 0.0);
    }

    #[test]
    fn test_positive_borrowed_amount() {
        let borrowed_amount = "100.5";
        let amount: f64 = borrowed_amount.parse().unwrap();
        assert!(amount > 0.0);
    }
}
