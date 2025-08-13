use super::{
    RestClient,
    borrowable::{BatchBorrowableRequest, BatchBorrowableResponse},
};

const BATCH_BORROWABLE_ENDPOINT: &str = "/unified/batch_borrowable";

impl RestClient {
    /// Get batch borrowable amounts
    ///
    /// This endpoint returns borrowable amounts for multiple currencies.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-batch-borrowable>
    pub async fn get_batch_borrowable(
        &self,
        request: BatchBorrowableRequest,
    ) -> crate::gateio::unified::RestResult<Vec<BatchBorrowableResponse>> {
        self.post(BATCH_BORROWABLE_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_borrowable_request_serialization() {
        let request = BatchBorrowableRequest {
            currencies: vec!["BTC".to_string(), "ETH".to_string(), "USDT".to_string()],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currencies"][0], "BTC");
        assert_eq!(json["currencies"][1], "ETH");
        assert_eq!(json["currencies"][2], "USDT");
    }

    #[test]
    fn test_batch_borrowable_response_deserialization() {
        let json = r#"[
            {
                "currency": "BTC",
                "borrowable": "0.5"
            },
            {
                "currency": "ETH",
                "borrowable": "10.0"
            }
        ]"#;

        let responses: Vec<BatchBorrowableResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 2);
        assert_eq!(responses[0].currency, "BTC");
        assert_eq!(responses[0].borrowable, "0.5");
        assert_eq!(responses[1].currency, "ETH");
        assert_eq!(responses[1].borrowable, "10.0");
    }

    #[test]
    fn test_batch_borrowable_endpoint() {
        assert_eq!(BATCH_BORROWABLE_ENDPOINT, "/unified/batch_borrowable");
    }

    #[test]
    fn test_batch_borrowable_empty_currencies() {
        let request = BatchBorrowableRequest { currencies: vec![] };

        let json = serde_json::to_value(&request).unwrap();
        let currencies = json["currencies"].as_array().unwrap();
        assert_eq!(currencies.len(), 0);
    }

    #[test]
    fn test_batch_borrowable_single_currency() {
        let request = BatchBorrowableRequest {
            currencies: vec!["USDT".to_string()],
        };

        let json = serde_json::to_value(&request).unwrap();
        let currencies = json["currencies"].as_array().unwrap();
        assert_eq!(currencies.len(), 1);
        assert_eq!(currencies[0], "USDT");
    }

    #[test]
    fn test_batch_borrowable_many_currencies() {
        let currencies = [
            "BTC", "ETH", "USDT", "BNB", "SOL", "ADA", "XRP", "DOT", "MATIC", "LINK",
        ];

        let request = BatchBorrowableRequest {
            currencies: currencies.iter().map(|&c| c.to_string()).collect(),
        };

        let json = serde_json::to_value(&request).unwrap();
        let json_currencies = json["currencies"].as_array().unwrap();
        assert_eq!(json_currencies.len(), currencies.len());

        for (i, currency) in currencies.iter().enumerate() {
            assert_eq!(json_currencies[i], *currency);
        }
    }
}
