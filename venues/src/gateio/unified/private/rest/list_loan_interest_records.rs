use super::{
    RestClient,
    loan::{ListLoanInterestRecordsRequest, LoanInterestRecord},
};

const UNIFIED_INTEREST_RECORD_ENDPOINT: &str = "/unified/interest_record";

impl RestClient {
    /// List loan interest records
    ///
    /// This endpoint returns the interest records for loans.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#list-loan-interest-records>
    pub async fn list_loan_interest_records(
        &self,
        request: ListLoanInterestRecordsRequest,
    ) -> crate::gateio::unified::Result<Vec<LoanInterestRecord>> {
        self.get_with_query(UNIFIED_INTEREST_RECORD_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_interest_record_endpoint() {
        assert_eq!(UNIFIED_INTEREST_RECORD_ENDPOINT, "/unified/interest_record");
    }

    #[test]
    fn test_list_loan_interest_records_request_serialization() {
        let request = ListLoanInterestRecordsRequest {
            currency: Some("BTC".to_string()),
            page: Some(1),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_list_loan_interest_records_request_default() {
        let request = ListLoanInterestRecordsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        assert!(!json.as_object().unwrap().contains_key("currency"));
        assert!(!json.as_object().unwrap().contains_key("page"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
    }

    #[test]
    fn test_loan_interest_record_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "interest": "0.00012345",
            "create_time": 1640995200
        }"#;

        let record: LoanInterestRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.currency, "BTC");
        assert_eq!(record.interest, "0.00012345");
        assert_eq!(record.create_time, 1640995200);
    }

    #[test]
    fn test_loan_interest_records_array_deserialization() {
        let json = r#"[
            {
                "currency": "BTC",
                "interest": "0.00012345",
                "create_time": 1640995200
            },
            {
                "currency": "ETH",
                "interest": "0.002",
                "create_time": 1640995300
            },
            {
                "currency": "USDT",
                "interest": "0.5",
                "create_time": 1640995400
            }
        ]"#;

        let records: Vec<LoanInterestRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].currency, "BTC");
        assert_eq!(records[1].currency, "ETH");
        assert_eq!(records[2].currency, "USDT");
    }

    #[test]
    fn test_list_loan_interest_records_pagination() {
        let pages = vec![1, 2, 5, 10];
        let limits = vec![10, 50, 100, 500];

        for (page, limit) in pages.into_iter().zip(limits) {
            let request = ListLoanInterestRecordsRequest {
                currency: Some("BTC".to_string()),
                page: Some(page),
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["page"], page);
            assert_eq!(json["limit"], limit);
        }
    }
}
