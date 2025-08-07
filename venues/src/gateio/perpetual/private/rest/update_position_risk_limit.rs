use serde::{Deserialize, Serialize};

use super::RestClient;

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request to update risk limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRiskLimitRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Risk limit value
    pub risk_limit: String,
}

/// Risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

impl RestClient {
    /// Update position risk limit
    ///
    /// This endpoint updates the risk limit for a futures position.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#update-position-risk-limit>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The risk limit update request parameters
    ///
    /// # Returns
    /// Updated risk limit information
    pub async fn update_position_risk_limit(
        &self,
        request: UpdateRiskLimitRequest,
    ) -> crate::gateio::perpetual::RestResult<RiskLimitResponse> {
        let endpoint = format!(
            "{}/{}/positions/{}/risk_limit",
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
    fn test_update_risk_limit_request() {
        let request = UpdateRiskLimitRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            risk_limit: "1000000".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["risk_limit"], "1000000");
    }

    #[test]
    fn test_risk_limit_values() {
        let risk_limits = vec![
            "100000", "500000", "1000000", "2000000", "5000000", "10000000", "20000000", "50000000",
        ];

        for limit in risk_limits {
            let request = UpdateRiskLimitRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                risk_limit: limit.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["risk_limit"], limit);
        }
    }

    #[test]
    fn test_risk_limit_response_deserialization() {
        let json = r#"{
            "risk_limit": "2000000"
        }"#;

        let response: RiskLimitResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.risk_limit, "2000000");
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
            let request = UpdateRiskLimitRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
                risk_limit: "1000000".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_risk_limit_tiers() {
        // Common risk limit tiers for different contracts
        let btc_tiers = vec!["1000000", "2000000", "5000000", "10000000", "20000000"];

        for tier in btc_tiers {
            let request = UpdateRiskLimitRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                risk_limit: tier.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["risk_limit"], tier);

            // Verify it's a valid number
            let limit_val = json["risk_limit"].as_str().unwrap();
            assert!(limit_val.parse::<u64>().is_ok());
        }
    }

    #[test]
    fn test_serialization_round_trip() {
        let request = UpdateRiskLimitRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            risk_limit: "5000000".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateRiskLimitRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.settle, request.settle);
        assert_eq!(deserialized.contract, request.contract);
        assert_eq!(deserialized.risk_limit, request.risk_limit);
    }

    #[test]
    fn test_endpoint_formatting() {
        let request = UpdateRiskLimitRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            risk_limit: "1000000".to_string(),
        };

        let endpoint = format!(
            "/futures/{}/positions/{}/risk_limit",
            request.settle, request.contract
        );
        assert_eq!(endpoint, "/futures/USDT/positions/BTC_USDT/risk_limit");
    }
}
