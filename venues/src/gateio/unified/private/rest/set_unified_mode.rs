use super::{
    RestClient,
    unified_mode::{UnifiedModeRequest, UnifiedModeResponse},
};

const UNIFIED_MODE_ENDPOINT: &str = "/unified/unified_mode";

impl RestClient {
    /// Set unified mode
    ///
    /// This endpoint enables or disables unified account mode.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#set-unified-mode>
    pub async fn set_unified_mode(
        &self,
        request: UnifiedModeRequest,
    ) -> crate::gateio::unified::Result<UnifiedModeResponse> {
        self.put(UNIFIED_MODE_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_mode_request_enable() {
        let request = UnifiedModeRequest { unified: true };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["unified"], true);
    }

    #[test]
    fn test_unified_mode_request_disable() {
        let request = UnifiedModeRequest { unified: false };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["unified"], false);
    }

    #[test]
    fn test_unified_mode_request_default() {
        let request = UnifiedModeRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["unified"], false); // Default is false
    }

    #[test]
    fn test_unified_mode_endpoint() {
        assert_eq!(UNIFIED_MODE_ENDPOINT, "/unified/unified_mode");
    }
}
