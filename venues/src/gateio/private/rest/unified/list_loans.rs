use super::{
    RestClient, RestResult,
    loan::{ListLoansRequest, LoanRecord},
};

const UNIFIED_LOANS_ENDPOINT: &str = "/unified/loans";

impl RestClient {
    /// List loan records
    ///
    /// This endpoint returns the borrowing history.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#list-loans)
    pub async fn list_loans(&self, request: ListLoansRequest) -> RestResult<Vec<LoanRecord>> {
        self.get_with_query(UNIFIED_LOANS_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_loans_endpoint() {
        assert_eq!(UNIFIED_LOANS_ENDPOINT, "/unified/loans");
    }

    #[test]
    fn test_list_loans_request_serialization() {
        let request = ListLoansRequest {
            currency: Some("BTC".to_string()),
            page: Some(1),
            limit: Some(10),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 10);
    }

    #[test]
    fn test_list_loans_request_default() {
        let request = ListLoansRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        assert!(!json.as_object().unwrap().contains_key("currency"));
        assert!(!json.as_object().unwrap().contains_key("page"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
    }

    #[test]
    fn test_loan_record_deserialization() {
        let json = r#"{
            "id": "123456",
            "create_time": 1640995200,
            "update_time": 1640995300,
            "currency": "BTC",
            "amount": "0.5",
            "interest": "0.001"
        }"#;

        let record: LoanRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "123456");
        assert_eq!(record.create_time, 1640995200);
        assert_eq!(record.update_time, 1640995300);
        assert_eq!(record.currency, "BTC");
        assert_eq!(record.amount, "0.5");
        assert_eq!(record.interest, "0.001");
    }

    #[test]
    fn test_loan_records_array_deserialization() {
        let json = r#"[
            {
                "id": "123456",
                "create_time": 1640995200,
                "update_time": 1640995300,
                "currency": "BTC",
                "amount": "0.5",
                "interest": "0.001"
            },
            {
                "id": "789012",
                "create_time": 1640995400,
                "update_time": 1640995500,
                "currency": "ETH",
                "amount": "10.0",
                "interest": "0.02"
            }
        ]"#;

        let records: Vec<LoanRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].currency, "BTC");
        assert_eq!(records[1].currency, "ETH");
    }
}
