use super::{
    RestClient,
    borrowable::{UnifiedBorrowableRequest, UnifiedBorrowableResponse},
};

const UNIFIED_BORROWABLE_ENDPOINT: &str = "/unified/borrowable";

impl RestClient {
    /// Get unified borrowable amount
    ///
    /// This endpoint returns the amount that can be borrowed for a currency.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-unified-borrowable>
    pub async fn get_unified_borrowable(
        &self,
        params: UnifiedBorrowableRequest,
    ) -> crate::gateio::unified::RestResult<UnifiedBorrowableResponse> {
        self.get_with_query(UNIFIED_BORROWABLE_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_borrowable_request_serialization() {
        let request = UnifiedBorrowableRequest {
            currency: "BTC".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
    }

    #[test]
    fn test_unified_borrowable_response_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "borrowable": "0.5"
        }"#;

        let response: UnifiedBorrowableResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.currency, "BTC");
        assert_eq!(response.borrowable, "0.5");
    }

    #[test]
    fn test_unified_borrowable_endpoint() {
        assert_eq!(UNIFIED_BORROWABLE_ENDPOINT, "/unified/borrowable");
    }

    #[test]
    fn test_unified_borrowable_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "BNB", "SOL"];

        for currency in currencies {
            let request = UnifiedBorrowableRequest {
                currency: currency.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
        }
    }
}
