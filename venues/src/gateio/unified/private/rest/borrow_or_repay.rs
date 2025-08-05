use super::{
    RestClient,
    loan::{BorrowOrRepayRequest, BorrowOrRepayResponse},
};

const UNIFIED_BORROW_OR_REPAY_ENDPOINT: &str = "/unified/borrow_or_repay";

impl RestClient {
    /// Borrow or repay
    ///
    /// This endpoint allows borrowing or repaying funds.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#borrow-or-repay>
    pub async fn borrow_or_repay(
        &self,
        request: BorrowOrRepayRequest,
    ) -> crate::gateio::unified::RestResult<BorrowOrRepayResponse> {
        self.post(UNIFIED_BORROW_OR_REPAY_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_request_serialization() {
        let request = BorrowOrRepayRequest {
            currency: "BTC".to_string(),
            amount: "0.1".to_string(),
            type_: "borrow".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["amount"], "0.1");
        assert_eq!(json["type"], "borrow");
    }

    #[test]
    fn test_repay_request_serialization() {
        let request = BorrowOrRepayRequest {
            currency: "USDT".to_string(),
            amount: "1000".to_string(),
            type_: "repay".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["amount"], "1000");
        assert_eq!(json["type"], "repay");
    }

    #[test]
    fn test_borrow_repay_response_deserialization() {
        let json = r#"{"succeed": true}"#;
        let response: BorrowOrRepayResponse = serde_json::from_str(json).unwrap();
        assert!(response.succeed);

        let json = r#"{"succeed": false}"#;
        let response: BorrowOrRepayResponse = serde_json::from_str(json).unwrap();
        assert!(!response.succeed);
    }
}
