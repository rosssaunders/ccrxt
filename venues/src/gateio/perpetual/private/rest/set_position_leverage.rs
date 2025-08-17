use serde::{Deserialize, Serialize};

use super::RestClient;

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request to set leverage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLeverageRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageResponse {
    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

impl RestClient {
    /// Set position leverage
    ///
    /// This endpoint sets the leverage for a futures position.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#update-position-leverage)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The leverage update request parameters
    ///
    /// # Returns
    /// Updated leverage information
    pub async fn set_position_leverage(
        &self,
        request: SetLeverageRequest,
    ) -> crate::gateio::perpetual::RestResult<LeverageResponse> {
        let endpoint = format!(
            "{}/{}/positions/{}/leverage",
            ENDPOINT_FUTURES_PREFIX, request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_leverage_request_minimal() {
        let request = SetLeverageRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            leverage: "10".to_string(),
            cross_leverage_limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["leverage"], "10");
        assert!(
            !json
                .as_object()
                .unwrap()
                .contains_key("cross_leverage_limit")
        );
    }

    #[test]
    fn test_set_leverage_request_with_cross_limit() {
        let request = SetLeverageRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            leverage: "10".to_string(),
            cross_leverage_limit: Some("25".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["leverage"], "10");
        assert_eq!(json["cross_leverage_limit"], "25");
    }

    #[test]
    fn test_leverage_values() {
        let leverage_values = vec![
            "1", "2", "3", "5", "10", "15", "20", "25", "50", "100", "125",
        ];

        for leverage in leverage_values {
            let request = SetLeverageRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                leverage: leverage.to_string(),
                cross_leverage_limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["leverage"], leverage);
        }
    }

    #[test]
    fn test_leverage_response_deserialization() {
        let json = r#"{
            "leverage": "10",
            "cross_leverage_limit": "25"
        }"#;

        let response: LeverageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.leverage, "10");
        assert_eq!(response.cross_leverage_limit, Some("25".to_string()));
    }

    #[test]
    fn test_leverage_response_minimal() {
        let json = r#"{
            "leverage": "5"
        }"#;

        let response: LeverageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.leverage, "5");
        assert!(response.cross_leverage_limit.is_none());
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
            let request = SetLeverageRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
                leverage: "10".to_string(),
                cross_leverage_limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_cross_leverage_limit_scenarios() {
        let cross_limits = vec![
            None,
            Some("10".to_string()),
            Some("25".to_string()),
            Some("50".to_string()),
            Some("100".to_string()),
        ];

        for cross_limit in cross_limits {
            let request = SetLeverageRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                leverage: "5".to_string(),
                cross_leverage_limit: cross_limit.clone(),
            };

            let json = serde_json::to_value(&request).unwrap();

            if let Some(limit) = cross_limit {
                assert_eq!(json["cross_leverage_limit"], limit);
            } else {
                assert!(
                    !json
                        .as_object()
                        .unwrap()
                        .contains_key("cross_leverage_limit")
                );
            }
        }
    }

    #[test]
    fn test_serialization_round_trip() {
        let request = SetLeverageRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            leverage: "20".to_string(),
            cross_leverage_limit: Some("50".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: SetLeverageRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.settle, request.settle);
        assert_eq!(deserialized.contract, request.contract);
        assert_eq!(deserialized.leverage, request.leverage);
        assert_eq!(
            deserialized.cross_leverage_limit,
            request.cross_leverage_limit
        );
    }
}
