use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to enable/disable dual mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeRequest {
    /// Settlement currency
    pub settle: String,
    /// Enable dual mode
    pub dual_mode: bool,
}

/// Dual mode response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeResponse {
    /// Whether dual mode is enabled
    pub dual_mode: bool,
}

impl RestClient {
    /// Set dual mode
    ///
    /// This endpoint enables or disables dual mode for futures positions.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#enable-or-disable-dual-mode>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The dual mode configuration request
    ///
    /// # Returns
    /// Dual mode status
    pub async fn set_dual_mode(
        &self,
        request: DualModeRequest,
    ) -> crate::gateio::perpetual::Result<DualModeResponse> {
        let endpoint = format!("/futures/{}/dual_mode", request.settle);
        self.post(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_mode_request_enable() {
        let request = DualModeRequest {
            settle: "USDT".to_string(),
            dual_mode: true,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["dual_mode"], true);
    }

    #[test]
    fn test_dual_mode_request_disable() {
        let request = DualModeRequest {
            settle: "USDT".to_string(),
            dual_mode: false,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["dual_mode"], false);
    }

    #[test]
    fn test_dual_mode_response_deserialization() {
        let json = r#"{
            "dual_mode": true
        }"#;

        let response: DualModeResponse = serde_json::from_str(json).unwrap();
        assert!(response.dual_mode);

        let json = r#"{
            "dual_mode": false
        }"#;

        let response: DualModeResponse = serde_json::from_str(json).unwrap();
        assert!(!response.dual_mode);
    }

    #[test]
    fn test_different_settlements() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let request = DualModeRequest {
                settle: settle.to_string(),
                dual_mode: true,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_endpoint_formatting() {
        let request = DualModeRequest {
            settle: "USDT".to_string(),
            dual_mode: true,
        };

        let endpoint = format!("/futures/{}/dual_mode", request.settle);
        assert_eq!(endpoint, "/futures/USDT/dual_mode");
    }

    #[test]
    fn test_serialization_round_trip() {
        let request = DualModeRequest {
            settle: "USDT".to_string(),
            dual_mode: true,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: DualModeRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.settle, request.settle);
        assert_eq!(deserialized.dual_mode, request.dual_mode);
    }

    #[test]
    fn test_dual_mode_scenarios() {
        // Enable dual mode for different settlements
        let enable_scenarios = vec![
            ("USDT", true, "Enable dual mode for USDT"),
            ("BTC", true, "Enable dual mode for BTC"),
            ("ETH", true, "Enable dual mode for ETH"),
        ];

        for (settle, dual_mode, _description) in enable_scenarios {
            let request = DualModeRequest {
                settle: settle.to_string(),
                dual_mode,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["dual_mode"], dual_mode);
        }

        // Disable dual mode for different settlements
        let disable_scenarios = vec![
            ("USDT", false, "Disable dual mode for USDT"),
            ("BTC", false, "Disable dual mode for BTC"),
            ("ETH", false, "Disable dual mode for ETH"),
        ];

        for (settle, dual_mode, _description) in disable_scenarios {
            let request = DualModeRequest {
                settle: settle.to_string(),
                dual_mode,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["dual_mode"], dual_mode);
        }
    }
}