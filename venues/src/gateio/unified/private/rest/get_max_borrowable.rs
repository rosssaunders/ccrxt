use super::{
    RestClient,
    loan::{GetMaxBorrowableRequest, MaxBorrowable},
};

const UNIFIED_BORROWABLE_ENDPOINT: &str = "/unified/borrowable";

impl RestClient {
    /// Get max borrowable amount
    ///
    /// This endpoint returns the maximum borrowable amount for a specific currency.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#get-max-borrowable)
    pub async fn get_max_borrowable(
        &self,
        currency: &str,
    ) -> crate::gateio::unified::RestResult<MaxBorrowable> {
        let request = GetMaxBorrowableRequest {
            currency: currency.to_string(),
        };
        self.get_with_query(UNIFIED_BORROWABLE_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_borrowable_endpoint() {
        assert_eq!(UNIFIED_BORROWABLE_ENDPOINT, "/unified/borrowable");
    }

    #[test]
    fn test_get_max_borrowable_request_serialization() {
        let request = GetMaxBorrowableRequest {
            currency: "BTC".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
    }

    #[test]
    fn test_max_borrowable_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "amount": "1.5"
        }"#;

        let max_borrowable: MaxBorrowable = serde_json::from_str(json).unwrap();
        assert_eq!(max_borrowable.currency, "BTC");
        assert_eq!(max_borrowable.amount, "1.5");
    }

    #[test]
    fn test_get_max_borrowable_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "BNB", "SOL"];

        for currency in currencies {
            let request = GetMaxBorrowableRequest {
                currency: currency.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
        }
    }

    #[test]
    fn test_max_borrowable_zero_amount() {
        let json = r#"{
            "currency": "XRP",
            "amount": "0"
        }"#;

        let max_borrowable: MaxBorrowable = serde_json::from_str(json).unwrap();
        assert_eq!(max_borrowable.currency, "XRP");
        assert_eq!(max_borrowable.amount, "0");
    }
}
