use super::RestClient;
use super::unified_margin::{CreateUnifiedMarginLoanRequest, UnifiedMarginLoan};

const MARGIN_UNI_LOANS_ENDPOINT: &str = "/margin/uni/loans";

impl RestClient {
    /// Create unified margin loan
    ///
    /// This endpoint creates a new unified margin loan.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#create-unified-margin-loan>
    pub async fn create_unified_margin_loan(
        &self,
        request: CreateUnifiedMarginLoanRequest,
    ) -> crate::gateio::unified::Result<UnifiedMarginLoan> {
        self.post(MARGIN_UNI_LOANS_ENDPOINT, &request).await
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
    fn test_create_unified_margin_loan_request_serialization() {
        let request = CreateUnifiedMarginLoanRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "0.5".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["amount"], "0.5");
    }

    #[test]
    fn test_unified_margin_loan_deserialization() {
        let json = r#"{
            "id": "789012",
            "currency": "ETH",
            "currency_pair": "ETH_USDT",
            "amount": "10.0",
            "used": "0.0",
            "repaid": "0.0",
            "interest": "0.0",
            "rate": "0.0001",
            "create_time": 1640995400,
            "update_time": 1640995400,
            "status": "active"
        }"#;

        let loan: UnifiedMarginLoan = serde_json::from_str(json).unwrap();
        assert_eq!(loan.id, "789012");
        assert_eq!(loan.currency, "ETH");
        assert_eq!(loan.currency_pair, "ETH_USDT");
        assert_eq!(loan.amount, "10.0");
        assert_eq!(loan.used, "0.0");
        assert_eq!(loan.repaid, "0.0");
        assert_eq!(loan.interest, "0.0");
        assert_eq!(loan.rate, "0.0001");
        assert_eq!(loan.status, "active");
    }
}