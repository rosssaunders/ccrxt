use super::RestClient;
use super::transferable::UnifiedTransferableResponse;

const UNIFIED_TRANSFERABLES_ENDPOINT: &str = "/unified/transferables";

impl RestClient {
    /// Get transferables for all currencies
    ///
    /// This endpoint returns transferable amounts for all currencies.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-unified-transferables>
    pub async fn get_unified_transferables(
        &self,
    ) -> crate::gateio::unified::Result<Vec<UnifiedTransferableResponse>> {
        self.get(UNIFIED_TRANSFERABLES_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_transferables_response_deserialization() {
        let json = r#"[
            {
                "currency": "BTC",
                "transferable": "0.5"
            },
            {
                "currency": "ETH",
                "transferable": "10.0"
            },
            {
                "currency": "USDT",
                "transferable": "1000.0"
            }
        ]"#;

        let responses: Vec<UnifiedTransferableResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 3);
        assert_eq!(responses[0].currency, "BTC");
        assert_eq!(responses[0].transferable, "0.5");
        assert_eq!(responses[1].currency, "ETH");
        assert_eq!(responses[1].transferable, "10.0");
        assert_eq!(responses[2].currency, "USDT");
        assert_eq!(responses[2].transferable, "1000.0");
    }

    #[test]
    fn test_unified_transferables_endpoint() {
        assert_eq!(UNIFIED_TRANSFERABLES_ENDPOINT, "/unified/transferables");
    }

    #[test]
    fn test_unified_transferables_empty_response() {
        let json = r#"[]"#;
        let responses: Vec<UnifiedTransferableResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 0);
    }

    #[test]
    fn test_unified_transferables_zero_amounts() {
        let json = r#"[
            {
                "currency": "XRP",
                "transferable": "0"
            },
            {
                "currency": "ADA",
                "transferable": "0.0"
            }
        ]"#;

        let responses: Vec<UnifiedTransferableResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 2);
        assert_eq!(responses[0].transferable, "0");
        assert_eq!(responses[1].transferable, "0.0");
    }
}