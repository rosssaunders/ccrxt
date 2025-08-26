use super::{
    RestClient, RestResult,
    unified_margin::{UnifiedMarginBorrowable, UnifiedMarginBorrowableRequest},
};

const MARGIN_UNI_BORROWABLE_ENDPOINT: &str = "/margin/uni/borrowable";

impl RestClient {
    /// Get unified margin borrowable amount
    ///
    /// This endpoint returns the amount that can be borrowed in unified margin.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-unified-margin-borrowable)
    pub async fn get_unified_margin_borrowable(
        &self,
        params: UnifiedMarginBorrowableRequest,
    ) -> RestResult<UnifiedMarginBorrowable> {
        self.get_with_query(MARGIN_UNI_BORROWABLE_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margin_uni_borrowable_endpoint() {
        assert_eq!(MARGIN_UNI_BORROWABLE_ENDPOINT, "/margin/uni/borrowable");
    }

    #[test]
    fn test_unified_margin_borrowable_request_serialization() {
        let request = UnifiedMarginBorrowableRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_unified_margin_borrowable_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "currency_pair": "BTC_USDT",
            "amount": "1.5"
        }"#;

        let borrowable: UnifiedMarginBorrowable = serde_json::from_str(json).unwrap();
        assert_eq!(borrowable.currency, "BTC");
        assert_eq!(borrowable.currency_pair, "BTC_USDT");
        assert_eq!(borrowable.amount, "1.5");
    }

    #[test]
    fn test_unified_margin_borrowable_zero_amount() {
        let json = r#"{
            "currency": "ETH",
            "currency_pair": "ETH_USDT",
            "amount": "0"
        }"#;

        let borrowable: UnifiedMarginBorrowable = serde_json::from_str(json).unwrap();
        assert_eq!(borrowable.currency, "ETH");
        assert_eq!(borrowable.currency_pair, "ETH_USDT");
        assert_eq!(borrowable.amount, "0");
    }
}
