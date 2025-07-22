use super::RestClient;
use super::unified_margin::{UnifiedMarginLoansRequest, UnifiedMarginLoan};

const MARGIN_UNI_LOANS_ENDPOINT: &str = "/margin/uni/loans";

impl RestClient {
    /// Get unified margin loans
    ///
    /// This endpoint returns unified margin loans for the authenticated user.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-unified-margin-loans>
    pub async fn get_unified_margin_loans(
        &self,
        params: UnifiedMarginLoansRequest,
    ) -> crate::gateio::unified::Result<Vec<UnifiedMarginLoan>> {
        self.get_with_query(MARGIN_UNI_LOANS_ENDPOINT, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margin_uni_loans_endpoint() {
        assert_eq!(MARGIN_UNI_LOANS_ENDPOINT, "/margin/uni/loans");
    }

    #[test]
    fn test_unified_margin_loans_request_serialization() {
        let request = UnifiedMarginLoansRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            currency: Some("BTC".to_string()),
            page: Some(1),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_unified_margin_loans_request_default() {
        let request = UnifiedMarginLoansRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        assert!(!json.as_object().unwrap().contains_key("currency_pair"));
        assert!(!json.as_object().unwrap().contains_key("currency"));
        assert!(!json.as_object().unwrap().contains_key("page"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
    }

    #[test]
    fn test_unified_margin_loan_deserialization() {
        let json = r#"{
            "id": "123456",
            "currency": "BTC",
            "currency_pair": "BTC_USDT",
            "amount": "0.5",
            "used": "0.3",
            "repaid": "0.1",
            "interest": "0.001",
            "rate": "0.0001",
            "create_time": 1640995200,
            "update_time": 1640995300,
            "status": "active"
        }"#;

        let loan: UnifiedMarginLoan = serde_json::from_str(json).unwrap();
        assert_eq!(loan.id, "123456");
        assert_eq!(loan.currency, "BTC");
        assert_eq!(loan.currency_pair, "BTC_USDT");
        assert_eq!(loan.amount, "0.5");
        assert_eq!(loan.used, "0.3");
        assert_eq!(loan.repaid, "0.1");
        assert_eq!(loan.interest, "0.001");
        assert_eq!(loan.rate, "0.0001");
        assert_eq!(loan.status, "active");
    }
}