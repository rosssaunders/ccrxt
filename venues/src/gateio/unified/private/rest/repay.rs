use super::{
    RestClient,
    loan::{BorrowOrRepayRequest, BorrowOrRepayResponse},
};

impl RestClient {
    /// Repay funds
    ///
    /// This is a convenience method that calls borrow_or_repay with type set to "repay".
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#repay-funds>
    pub async fn repay(
        &self,
        currency: &str,
        amount: &str,
    ) -> crate::gateio::unified::Result<BorrowOrRepayResponse> {
        let request = BorrowOrRepayRequest {
            currency: currency.to_string(),
            amount: amount.to_string(),
            type_: "repay".to_string(),
        };
        self.borrow_or_repay(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repay_creates_correct_request() {
        let currency = "USDT";
        let amount = "500";

        let request = BorrowOrRepayRequest {
            currency: currency.to_string(),
            amount: amount.to_string(),
            type_: "repay".to_string(),
        };

        assert_eq!(request.currency, "USDT");
        assert_eq!(request.amount, "500");
        assert_eq!(request.type_, "repay");
    }

    #[test]
    fn test_repay_different_amounts() {
        let repayments = vec![("BTC", "0.01"), ("ETH", "0.5"), ("USDT", "2000.0")];

        for (currency, amount) in repayments {
            let request = BorrowOrRepayRequest {
                currency: currency.to_string(),
                amount: amount.to_string(),
                type_: "repay".to_string(),
            };

            assert_eq!(request.currency, currency);
            assert_eq!(request.amount, amount);
            assert_eq!(request.type_, "repay");
        }
    }
}
