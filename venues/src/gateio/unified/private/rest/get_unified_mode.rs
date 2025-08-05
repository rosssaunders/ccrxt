use super::{RestClient, unified_mode::UnifiedModeResponse};

const UNIFIED_MODE_ENDPOINT: &str = "/unified/unified_mode";

impl RestClient {
    /// Get unified mode status
    ///
    /// This endpoint returns the current unified mode status.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-unified-mode>
    pub async fn get_unified_mode(
        &self,
    ) -> crate::gateio::unified::RestResult<UnifiedModeResponse> {
        self.get(UNIFIED_MODE_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_mode_response_deserialization() {
        let json = r#"{
            "user_id": 123456,
            "unified": true
        }"#;

        let response: UnifiedModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.user_id, 123456);
        assert!(response.unified);
    }

    #[test]
    fn test_unified_mode_response_classic_mode() {
        let json = r#"{
            "user_id": 789012,
            "unified": false
        }"#;

        let response: UnifiedModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.user_id, 789012);
        assert!(!response.unified);
    }

    #[test]
    fn test_unified_mode_endpoint() {
        assert_eq!(UNIFIED_MODE_ENDPOINT, "/unified/unified_mode");
    }
}
