use serde::{Deserialize, Serialize};

use super::RestClient;

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request to update dual mode risk limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDualModeRiskLimitRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Risk limit value
    pub risk_limit: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeRiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

impl RestClient {
    /// Update dual mode position risk limit
    ///
    /// This endpoint updates the risk limit for a dual mode position.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#update-position-risk-limit-in-dual-mode>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The dual mode risk limit update request parameters
    ///
    /// # Returns
    /// Updated risk limit information
    pub async fn update_dual_mode_position_risk_limit(
        &self,
        request: UpdateDualModeRiskLimitRequest,
    ) -> crate::gateio::perpetual::RestResult<DualModeRiskLimitResponse> {
        let endpoint = format!(
            "{}/{}/dual_positions/{}/risk_limit",
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
    fn test_update_dual_mode_risk_limit_long() {
        let request = UpdateDualModeRiskLimitRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            risk_limit: "1000000".to_string(),
            side: "long".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["risk_limit"], "1000000");
        assert_eq!(json["side"], "long");
    }

    #[test]
    fn test_update_dual_mode_risk_limit_short() {
        let request = UpdateDualModeRiskLimitRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            risk_limit: "2000000".to_string(),
            side: "short".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["risk_limit"], "2000000");
        assert_eq!(json["side"], "short");
    }

    #[test]
    fn test_dual_mode_risk_limit_response() {
        let json = r#"{
            "risk_limit": "5000000"
        }"#;

        let response: DualModeRiskLimitResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.risk_limit, "5000000");
    }

    #[test]
    fn test_risk_limit_values() {
        let risk_limits = vec![
            "100000", "500000", "1000000", "2000000", "5000000", "10000000", "20000000", "50000000",
        ];

        for limit in risk_limits {
            let request = UpdateDualModeRiskLimitRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                risk_limit: limit.to_string(),
                side: "long".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["risk_limit"], limit);
        }
    }

    #[test]
    fn test_position_sides() {
        let sides = vec![("long", "Long position"), ("short", "Short position")];

        for (side, _description) in sides {
            let request = UpdateDualModeRiskLimitRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                risk_limit: "1000000".to_string(),
                side: side.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["side"], side);
        }
    }

    #[test]
    fn test_risk_limit_scenarios() {
        let scenarios = vec![
            ("100000", "long", "Low risk limit for long"),
            ("1000000", "long", "Medium risk limit for long"),
            ("10000000", "long", "High risk limit for long"),
            ("100000", "short", "Low risk limit for short"),
            ("1000000", "short", "Medium risk limit for short"),
            ("10000000", "short", "High risk limit for short"),
        ];

        for (risk_limit, side, _description) in scenarios {
            let request = UpdateDualModeRiskLimitRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                risk_limit: risk_limit.to_string(),
                side: side.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["risk_limit"], risk_limit);
            assert_eq!(json["side"], side);
        }
    }

    #[test]
    fn test_different_contracts() {
        let contracts = vec![
            ("USDT", "BTC_USDT"),
            ("USDT", "ETH_USDT"),
            ("USDT", "SOL_USDT"),
            ("BTC", "BTC_USD"),
            ("ETH", "ETH_USD"),
        ];

        for (settle, contract) in contracts {
            let request = UpdateDualModeRiskLimitRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
                risk_limit: "1000000".to_string(),
                side: "long".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_serialization_round_trip() {
        let request = UpdateDualModeRiskLimitRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            risk_limit: "5000000".to_string(),
            side: "short".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateDualModeRiskLimitRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.settle, request.settle);
        assert_eq!(deserialized.contract, request.contract);
        assert_eq!(deserialized.risk_limit, request.risk_limit);
        assert_eq!(deserialized.side, request.side);
    }

    #[test]
    fn test_endpoint_formatting() {
        let request = UpdateDualModeRiskLimitRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            risk_limit: "1000000".to_string(),
            side: "long".to_string(),
        };

        let endpoint = format!(
            "/futures/{}/dual_positions/{}/risk_limit",
            request.settle, request.contract
        );
        assert_eq!(endpoint, "/futures/USDT/dual_positions/BTC_USDT/risk_limit");
    }
}
