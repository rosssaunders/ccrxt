use super::{
    RestClient,
    loan::{ListLoansRequest, LoanRecord},
};

impl RestClient {
    /// Get all loan history
    ///
    /// This is a convenience method that calls list_loans without a currency filter.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#get-all-loan-history)
    pub async fn get_all_loan_history(
        &self,
        limit: Option<u32>,
    ) -> crate::gateio::unified::RestResult<Vec<LoanRecord>> {
        let request = ListLoansRequest {
            currency: None,
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
    fn test_get_all_loan_history_request() {
        let limit = Some(100);

        let request = ListLoansRequest {
            currency: None,
            limit,
            ..Default::default()
        };

        assert_eq!(request.currency, None);
        assert_eq!(request.limit, Some(100));
        assert_eq!(request.page, None);
    }

    #[test]
    fn test_get_all_loan_history_no_limit() {
        let request = ListLoansRequest {
            currency: None,
            limit: None,
            ..Default::default()
        };

        assert_eq!(request.currency, None);
        assert_eq!(request.limit, None);
        assert_eq!(request.page, None);
    }

    #[test]
    fn test_get_all_loan_history_different_limits() {
        let limits = vec![10, 50, 100, 500, 1000];

        for limit in limits {
            let request = ListLoansRequest {
                currency: None,
                limit: Some(limit),
                ..Default::default()
            };

            assert_eq!(request.limit, Some(limit));
        }
    }
}
