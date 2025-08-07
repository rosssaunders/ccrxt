use serde::{Deserialize, Serialize};

use super::RestClient;

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request to update dual mode leverage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDualModeLeverageRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Leverage value
    pub leverage: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeLeverageResponse {
    /// Leverage value
    pub leverage: String,
}

impl RestClient {
    /// Update dual mode position leverage
    ///
    /// This endpoint updates the leverage for a dual mode position.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#update-position-leverage-in-dual-mode>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The dual mode leverage update request parameters
    ///
    /// # Returns
    /// Updated leverage information
    pub async fn update_dual_mode_position_leverage(
        &self,
        request: UpdateDualModeLeverageRequest,
    ) -> crate::gateio::perpetual::RestResult<DualModeLeverageResponse> {
        let endpoint = format!(
            "{}/{}/dual_positions/{}/leverage",
            ENDPOINT_FUTURES_PREFIX,
            request.settle,
            request.contract
        );
        self.post(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_dual_mode_leverage_long() {
        let request = UpdateDualModeLeverageRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            leverage: "10".to_string(),
            side: "long".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["leverage"], "10");
        assert_eq!(json["side"], "long");
    }

    #[test]
    fn test_update_dual_mode_leverage_short() {
        let request = UpdateDualModeLeverageRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            leverage: "5".to_string(),
            side: "short".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["leverage"], "5");
        assert_eq!(json["side"], "short");
    }

    #[test]
    fn test_dual_mode_leverage_response() {
        let json = r#"{
            "leverage": "20"
        }"#;

        let response: DualModeLeverageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.leverage, "20");
    }

    #[test]
    fn test_leverage_values() {
        let leverage_values = vec![
            "1", "2", "3", "5", "10", "15", "20", "25", "50", "100", "125",
        ];

        for leverage in leverage_values {
            let request = UpdateDualModeLeverageRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                leverage: leverage.to_string(),
                side: "long".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["leverage"], leverage);
        }
    }

    #[test]
    fn test_position_sides() {
        let sides = vec![("long", "Long position"), ("short", "Short position")];

        for (side, _description) in sides {
            let request = UpdateDualModeLeverageRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                leverage: "10".to_string(),
                side: side.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["side"], side);
        }
    }

    #[test]
    fn test_leverage_scenarios() {
        let scenarios = vec![
            ("1", "long", "Conservative long leverage"),
            ("10", "long", "Moderate long leverage"),
            ("50", "long", "High long leverage"),
            ("1", "short", "Conservative short leverage"),
            ("10", "short", "Moderate short leverage"),
            ("50", "short", "High short leverage"),
        ];

        for (leverage, side, _description) in scenarios {
            let request = UpdateDualModeLeverageRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                leverage: leverage.to_string(),
                side: side.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["leverage"], leverage);
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
            let request = UpdateDualModeLeverageRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
                leverage: "10".to_string(),
                side: "long".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_serialization_round_trip() {
        let request = UpdateDualModeLeverageRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            leverage: "25".to_string(),
            side: "short".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateDualModeLeverageRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.settle, request.settle);
        assert_eq!(deserialized.contract, request.contract);
        assert_eq!(deserialized.leverage, request.leverage);
        assert_eq!(deserialized.side, request.side);
    }
}
