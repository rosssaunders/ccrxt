use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to update position margin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePositionMarginRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Change amount (positive to add, negative to remove)
    pub change: String,
}

/// Position margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionMarginResponse {
    /// New margin amount
    pub margin: String,
}

impl RestClient {
    /// Update position margin
    ///
    /// This endpoint updates the margin for a futures position.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#update-position-margin>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The margin update request parameters
    ///
    /// # Returns
    /// Updated margin information
    pub async fn update_position_margin(
        &self,
        request: UpdatePositionMarginRequest,
    ) -> crate::gateio::perpetual::RestResult<PositionMarginResponse> {
        let endpoint = format!(
            "/futures/{}/positions/{}/margin",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_margin_request_add() {
        let request = UpdatePositionMarginRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            change: "1000.0".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["change"], "1000.0");
    }

    #[test]
    fn test_update_margin_request_remove() {
        let request = UpdatePositionMarginRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            change: "-500.0".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");
        assert_eq!(json["change"], "-500.0");
    }

    #[test]
    fn test_margin_change_amounts() {
        let change_amounts = vec![
            "100.0", "500.0", "1000.0", "5000.0", "-100.0", "-500.0", "-1000.0", "-5000.0",
        ];

        for change in change_amounts {
            let request = UpdatePositionMarginRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                change: change.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["change"], change);
        }
    }

    #[test]
    fn test_position_margin_response_deserialization() {
        let json = r#"{
            "margin": "5000.0"
        }"#;

        let response: PositionMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.margin, "5000.0");
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
            let request = UpdatePositionMarginRequest {
                settle: settle.to_string(),
                contract: contract.to_string(),
                change: "1000.0".to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_margin_scenarios() {
        // Add margin scenarios
        let add_scenarios = vec![
            ("100.0", "Small margin addition"),
            ("1000.0", "Medium margin addition"),
            ("10000.0", "Large margin addition"),
        ];

        for (amount, _description) in add_scenarios {
            let request = UpdatePositionMarginRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                change: amount.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            let change_val = json["change"].as_str().unwrap();
            assert!(change_val.parse::<f64>().unwrap() > 0.0);
        }

        // Remove margin scenarios
        let remove_scenarios = vec![
            ("-100.0", "Small margin removal"),
            ("-1000.0", "Medium margin removal"),
            ("-10000.0", "Large margin removal"),
        ];

        for (amount, _description) in remove_scenarios {
            let request = UpdatePositionMarginRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                change: amount.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            let change_val = json["change"].as_str().unwrap();
            assert!(change_val.parse::<f64>().unwrap() < 0.0);
        }
    }

    #[test]
    fn test_decimal_precision() {
        let precise_amounts = vec![
            "100.1", "100.01", "100.001", "100.0001", "-50.5", "-50.05", "-50.005",
        ];

        for amount in precise_amounts {
            let request = UpdatePositionMarginRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                change: amount.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["change"], amount);
        }
    }
}
