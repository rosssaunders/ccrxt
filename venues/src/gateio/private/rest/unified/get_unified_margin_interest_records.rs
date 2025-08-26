use super::{
    RestClient, RestResult,
    unified_margin::{UnifiedMarginInterestRecord, UnifiedMarginInterestRecordsRequest},
};

const MARGIN_UNI_INTEREST_RECORDS_ENDPOINT: &str = "/margin/uni/interest_records";

impl RestClient {
    /// Get unified margin interest records
    ///
    /// This endpoint returns unified margin interest records.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-unified-margin-interest-records)
    pub async fn get_unified_margin_interest_records(
        &self,
        params: UnifiedMarginInterestRecordsRequest,
    ) -> RestResult<Vec<UnifiedMarginInterestRecord>> {
        self.get_with_query(MARGIN_UNI_INTEREST_RECORDS_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margin_uni_interest_records_endpoint() {
        assert_eq!(
            MARGIN_UNI_INTEREST_RECORDS_ENDPOINT,
            "/margin/uni/interest_records"
        );
    }

    #[test]
    fn test_unified_margin_interest_records_request_serialization() {
        let request = UnifiedMarginInterestRecordsRequest {
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
    fn test_unified_margin_interest_records_request_default() {
        let request = UnifiedMarginInterestRecordsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        assert!(!json.as_object().unwrap().contains_key("currency_pair"));
        assert!(!json.as_object().unwrap().contains_key("currency"));
        assert!(!json.as_object().unwrap().contains_key("page"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
    }

    #[test]
    fn test_unified_margin_interest_record_deserialization() {
        let json = r#"{
            "id": "int123",
            "currency": "BTC",
            "currency_pair": "BTC_USDT",
            "interest": "0.001",
            "rate": "0.0001",
            "loan_amount": "10.0",
            "interest_time": 1640995200
        }"#;

        let record: UnifiedMarginInterestRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "int123");
        assert_eq!(record.currency, "BTC");
        assert_eq!(record.currency_pair, "BTC_USDT");
        assert_eq!(record.interest, "0.001");
        assert_eq!(record.rate, "0.0001");
        assert_eq!(record.loan_amount, "10.0");
        assert_eq!(record.interest_time, 1640995200);
    }

    #[test]
    fn test_unified_margin_interest_records_array_deserialization() {
        let json = r#"[
            {
                "id": "int123",
                "currency": "BTC",
                "currency_pair": "BTC_USDT",
                "interest": "0.001",
                "rate": "0.0001",
                "loan_amount": "10.0",
                "interest_time": 1640995200
            },
            {
                "id": "int456",
                "currency": "ETH",
                "currency_pair": "ETH_USDT",
                "interest": "0.02",
                "rate": "0.0001",
                "loan_amount": "100.0",
                "interest_time": 1640995300
            }
        ]"#;

        let records: Vec<UnifiedMarginInterestRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].currency, "BTC");
        assert_eq!(records[1].currency, "ETH");
    }
}
