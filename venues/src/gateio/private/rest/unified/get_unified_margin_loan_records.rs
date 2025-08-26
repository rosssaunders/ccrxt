use super::{
    RestClient, RestResult,
    unified_margin::{UnifiedMarginLoanRecord, UnifiedMarginLoanRecordsRequest},
};

const MARGIN_UNI_LOAN_RECORDS_ENDPOINT: &str = "/margin/uni/loan_records";

impl RestClient {
    /// Get unified margin loan records
    ///
    /// This endpoint returns unified margin loan records.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-unified-margin-loan-records)
    pub async fn get_unified_margin_loan_records(
        &self,
        params: UnifiedMarginLoanRecordsRequest,
    ) -> RestResult<Vec<UnifiedMarginLoanRecord>> {
        self.get_with_query(MARGIN_UNI_LOAN_RECORDS_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margin_uni_loan_records_endpoint() {
        assert_eq!(MARGIN_UNI_LOAN_RECORDS_ENDPOINT, "/margin/uni/loan_records");
    }

    #[test]
    fn test_unified_margin_loan_records_request_serialization() {
        let request = UnifiedMarginLoanRecordsRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            currency: Some("BTC".to_string()),
            page: Some(2),
            limit: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["page"], 2);
        assert_eq!(json["limit"], 100);
    }

    #[test]
    fn test_unified_margin_loan_record_deserialization() {
        let json = r#"{
            "id": "rec123",
            "loan_id": "loan456",
            "currency": "BTC",
            "currency_pair": "BTC_USDT",
            "amount": "0.5",
            "rate": "0.0001",
            "interest": "0.001",
            "status": "repaid",
            "borrow_time": 1640995200,
            "repay_time": 1640995500
        }"#;

        let record: UnifiedMarginLoanRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "rec123");
        assert_eq!(record.loan_id, "loan456");
        assert_eq!(record.currency, "BTC");
        assert_eq!(record.currency_pair, "BTC_USDT");
        assert_eq!(record.amount, "0.5");
        assert_eq!(record.rate, "0.0001");
        assert_eq!(record.interest, "0.001");
        assert_eq!(record.status, "repaid");
        assert_eq!(record.borrow_time, 1640995200);
        assert_eq!(record.repay_time, Some(1640995500));
    }

    #[test]
    fn test_unified_margin_loan_record_active_deserialization() {
        let json = r#"{
            "id": "rec789",
            "loan_id": "loan012",
            "currency": "ETH",
            "currency_pair": "ETH_USDT",
            "amount": "10.0",
            "rate": "0.0001",
            "interest": "0.01",
            "status": "active",
            "borrow_time": 1640995300
        }"#;

        let record: UnifiedMarginLoanRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "rec789");
        assert_eq!(record.status, "active");
        assert_eq!(record.repay_time, None);
    }
}
