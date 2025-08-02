use super::{
    RestClient,
    transferable::{UnifiedTransferableRequest, UnifiedTransferableResponse},
};

const UNIFIED_TRANSFERABLE_ENDPOINT: &str = "/unified/transferable";

impl RestClient {
    /// Get unified transferable amount
    ///
    /// This endpoint returns the amount that can be transferred between accounts.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-unified-transferable>
    pub async fn get_unified_transferable(
        &self,
        params: UnifiedTransferableRequest,
    ) -> crate::gateio::unified::Result<UnifiedTransferableResponse> {
        self.get_with_query(UNIFIED_TRANSFERABLE_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_transferable_request_serialization() {
        let request = UnifiedTransferableRequest {
            currency: "BTC".to_string(),
            from: "spot".to_string(),
            to: "futures".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["from"], "spot");
        assert_eq!(json["to"], "futures");
    }

    #[test]
    fn test_unified_transferable_response_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "transferable": "0.5"
        }"#;

        let response: UnifiedTransferableResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.currency, "BTC");
        assert_eq!(response.transferable, "0.5");
    }

    #[test]
    fn test_unified_transferable_endpoint() {
        assert_eq!(UNIFIED_TRANSFERABLE_ENDPOINT, "/unified/transferable");
    }

    #[test]
    fn test_unified_transferable_different_accounts() {
        let account_pairs = vec![
            ("spot", "futures"),
            ("spot", "margin"),
            ("futures", "spot"),
            ("margin", "spot"),
        ];

        for (from, to) in account_pairs {
            let request = UnifiedTransferableRequest {
                currency: "USDT".to_string(),
                from: from.to_string(),
                to: to.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["from"], from);
            assert_eq!(json["to"], to);
        }
    }
}
