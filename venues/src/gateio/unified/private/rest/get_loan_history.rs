use super::{
    RestClient,
    loan::{ListLoansRequest, LoanRecord},
};

impl RestClient {
    /// Get loan history for a specific currency
    ///
    /// This is a convenience method that calls list_loans with a currency filter.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-loan-history>
    pub async fn get_loan_history(
        &self,
        currency: &str,
        limit: Option<u32>,
    ) -> crate::gateio::unified::RestResult<Vec<LoanRecord>> {
        let request = ListLoansRequest {
            currency: Some(currency.to_string()),
            limit,
            ..Default::default()
        };
        self.list_loans(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loan_history_request() {
        let currency = "BTC";
        let limit = Some(50);

        let request = ListLoansRequest {
            currency: Some(currency.to_string()),
            limit,
            ..Default::default()
        };

        assert_eq!(request.currency, Some("BTC".to_string()));
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.page, None);
    }

    #[test]
    fn test_get_loan_history_no_limit() {
        let currency = "ETH";

        let request = ListLoansRequest {
            currency: Some(currency.to_string()),
            limit: None,
            ..Default::default()
        };

        assert_eq!(request.currency, Some("ETH".to_string()));
        assert_eq!(request.limit, None);
    }

    #[test]
    fn test_get_loan_history_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "BNB", "SOL"];

        for currency in currencies {
            let request = ListLoansRequest {
                currency: Some(currency.to_string()),
                limit: Some(100),
                ..Default::default()
            };

            assert_eq!(request.currency, Some(currency.to_string()));
        }
    }
}
