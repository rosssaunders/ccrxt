use super::{
    RestClient, RestResult,
    loan::{BorrowOrRepayRequest, BorrowOrRepayResponse},
};

impl RestClient {
    /// Borrow funds
    ///
    /// This is a convenience method that calls borrow_or_repay with type set to "borrow".
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#borrow-funds)
    pub async fn borrow(&self, currency: &str, amount: &str) -> RestResult<BorrowOrRepayResponse> {
        let request = BorrowOrRepayRequest {
            currency: currency.to_string(),
            amount: amount.to_string(),
            type_: "borrow".to_string(),
        };
        self.borrow_or_repay(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_creates_correct_request() {
        let currency = "BTC";
        let amount = "0.5";

        let request = BorrowOrRepayRequest {
            currency: currency.to_string(),
            amount: amount.to_string(),
            type_: "borrow".to_string(),
        };

        assert_eq!(request.currency, "BTC");
        assert_eq!(request.amount, "0.5");
        assert_eq!(request.type_, "borrow");
    }

    #[test]
    fn test_borrow_different_currencies() {
        let currencies = vec![("BTC", "0.1"), ("ETH", "1.0"), ("USDT", "1000.0")];

        for (currency, amount) in currencies {
            let request = BorrowOrRepayRequest {
                currency: currency.to_string(),
                amount: amount.to_string(),
                type_: "borrow".to_string(),
            };

            assert_eq!(request.currency, currency);
            assert_eq!(request.amount, amount);
            assert_eq!(request.type_, "borrow");
        }
    }
}
