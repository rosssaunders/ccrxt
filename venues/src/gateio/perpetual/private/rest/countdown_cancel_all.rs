use serde::{Deserialize, Serialize};

use super::RestClient;

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request for countdown cancel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownCancelRequest {
    /// Settlement currency
    pub settle: String,
    /// Timeout in seconds
    pub timeout: i32,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

impl RestClient {
    /// Countdown cancel all futures orders
    ///
    /// Sets a countdown timer to cancel all orders after a specified timeout.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#countdown-cancel-all-open-orders-of-specified-contract)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The countdown cancel request parameters
    ///
    /// # Returns
    /// Empty response indicating success
    pub async fn countdown_cancel_all_futures_orders(
        &self,
        request: CountdownCancelRequest,
    ) -> crate::gateio::perpetual::RestResult<()> {
        let endpoint = format!(
            "{}/{}/countdown_cancel_all",
            ENDPOINT_FUTURES_PREFIX, request.settle
        );
        self.post::<serde_json::Value>(&endpoint, &request).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown_cancel_request_minimal() {
        let request = CountdownCancelRequest {
            settle: "USDT".to_string(),
            timeout: 60,
            contract: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["timeout"], 60);
        assert!(!json.as_object().unwrap().contains_key("contract"));
    }

    #[test]
    fn test_countdown_cancel_request_with_contract() {
        let request = CountdownCancelRequest {
            settle: "USDT".to_string(),
            timeout: 30,
            contract: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["timeout"], 30);
        assert_eq!(json["contract"], "BTC_USDT");
    }

    #[test]
    fn test_timeout_values() {
        let timeout_scenarios = vec![
            (10, "10 seconds"),
            (30, "30 seconds"),
            (60, "1 minute"),
            (120, "2 minutes"),
            (300, "5 minutes"),
            (600, "10 minutes"),
        ];

        for (timeout, _description) in timeout_scenarios {
            let request = CountdownCancelRequest {
                settle: "USDT".to_string(),
                timeout,
                contract: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["timeout"], timeout);
        }
    }

    #[test]
    fn test_different_contracts() {
        let contracts = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT", "MATIC_USDT"];

        for contract in contracts {
            let request = CountdownCancelRequest {
                settle: "USDT".to_string(),
                timeout: 60,
                contract: Some(contract.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_settlement_currencies() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let request = CountdownCancelRequest {
                settle: settle.to_string(),
                timeout: 60,
                contract: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_serialization_omits_null() {
        let request = CountdownCancelRequest {
            settle: "USDT".to_string(),
            timeout: 60,
            contract: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        // Only settle and timeout should be present
        assert_eq!(obj.len(), 2);
        assert!(obj.contains_key("settle"));
        assert!(obj.contains_key("timeout"));
        assert!(!obj.contains_key("contract"));
    }

    #[test]
    fn test_countdown_scenarios() {
        // Scenario 1: Cancel all orders across all contracts
        let cancel_all = CountdownCancelRequest {
            settle: "USDT".to_string(),
            timeout: 60,
            contract: None,
        };

        let json = serde_json::to_value(&cancel_all).unwrap();
        assert!(!json.as_object().unwrap().contains_key("contract"));

        // Scenario 2: Cancel all orders for specific contract
        let cancel_specific = CountdownCancelRequest {
            settle: "USDT".to_string(),
            timeout: 30,
            contract: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&cancel_specific).unwrap();
        assert_eq!(json["contract"], "BTC_USDT");

        // Scenario 3: Emergency cancel with short timeout
        let emergency_cancel = CountdownCancelRequest {
            settle: "USDT".to_string(),
            timeout: 5,
            contract: None,
        };

        let json = serde_json::to_value(&emergency_cancel).unwrap();
        assert_eq!(json["timeout"], 5);
    }
}
