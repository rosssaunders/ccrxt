use serde::{Deserialize, Serialize};

use super::RestClient;

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request to update dual mode margin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDualModeMarginRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Change amount
    pub change: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeMarginResponse {
    /// New margin amount
    pub margin: String,
}

impl RestClient {
    /// Update dual mode position margin
    ///
    /// This endpoint updates the margin for a dual mode position.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#update-position-margin-in-dual-mode>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The dual mode margin update request parameters
    ///
    /// # Returns
    /// Updated margin information
    pub async fn update_dual_mode_position_margin(
        &self,
        request: UpdateDualModeMarginRequest,
    ) -> crate::gateio::perpetual::RestResult<DualModeMarginResponse> {
        let endpoint = format!(
            "{}/{}/dual_positions/{}/margin",
            ENDPOINT_FUTURES_PREFIX, request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_dual_mode_margin_long() {
        let request = UpdateDualModeMarginRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            change: "1000.0".to_string(),
            side: "long".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["change"], "1000.0");
        assert_eq!(json["side"], "long");
    }

    #[test]
    fn test_update_dual_mode_margin_short() {
        let request = UpdateDualModeMarginRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            change: "-500.0".to_string(),
            side: "short".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["change"], "-500.0");
        assert_eq!(json["side"], "short");
    }

    #[test]
    fn test_dual_mode_margin_response() {
        let json = r#"{
            "margin": "5000.0"
        }"#;

        let response: DualModeMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.margin, "5000.0");
    }

    #[test]
    fn test_position_sides() {
        let sides = vec!["long", "short"];

        for side in sides {
            let request = UpdateDualModeMarginRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                change: "1000.0".to_string(),
                side: side.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["side"], side);
        }
    }

    #[test]
    fn test_margin_change_scenarios() {
        let scenarios = vec![
            ("1000.0", "long", "Add margin to long position"),
            ("-500.0", "long", "Remove margin from long position"),
            ("2000.0", "short", "Add margin to short position"),
            ("-1000.0", "short", "Remove margin from short position"),
        ];

        for (change, side, _description) in scenarios {
            let request = UpdateDualModeMarginRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                change: change.to_string(),
                side: side.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["change"], change);
            assert_eq!(json["side"], side);
        }
    }

    #[test]
    fn test_different_contracts() {
        let contracts = vec![
            ("USDT", "BTC_USDT"),
            ("USDT", "ETH_USDT"),
            ("BTC", "BTC_USD"),
            ("ETH", "ETH_USD"),
        ];

        for (settle, contract) in contracts {
            let request = UpdateDualModeMarginRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
                change: "1000.0".to_string(),
                side: "long".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_serialization_round_trip() {
        let request = UpdateDualModeMarginRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            change: "1500.0".to_string(),
            side: "short".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateDualModeMarginRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.settle, request.settle);
        assert_eq!(deserialized.contract, request.contract);
        assert_eq!(deserialized.change, request.change);
        assert_eq!(deserialized.side, request.side);
    }
}
