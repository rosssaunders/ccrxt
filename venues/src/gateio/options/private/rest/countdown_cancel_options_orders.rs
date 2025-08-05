use serde::Serialize;

use super::RestClient;

const COUNTDOWN_CANCEL_OPTIONS_ORDERS_ENDPOINT: &str = "/options/countdown_cancel_all";

/// Request for countdown cancel orders
#[derive(Debug, Clone, Serialize)]
pub struct CountdownCancelOptionsOrdersRequest {
    /// Countdown time in seconds
    pub timeout: i32,

    /// Optional underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
}

impl RestClient {
    /// Countdown cancel orders
    ///
    /// Sets a countdown timer to cancel all open options orders after specified time.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#options-countdown-cancel>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `timeout` - Countdown time in seconds
    /// * `underlying` - Optional underlying asset filter
    ///
    /// # Returns
    /// Countdown cancel response
    pub async fn countdown_cancel_options_orders(
        &self,
        timeout: i32,
        underlying: Option<&str>,
    ) -> crate::gateio::options::RestResult<serde_json::Value> {
        let request = CountdownCancelOptionsOrdersRequest {
            timeout,
            underlying: underlying.map(|s| s.to_string()),
        };

        self.post(COUNTDOWN_CANCEL_OPTIONS_ORDERS_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown_cancel_options_orders_request_minimal() {
        let request = CountdownCancelOptionsOrdersRequest {
            timeout: 10,
            underlying: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 10);
        assert!(!json.as_object().unwrap().contains_key("underlying"));
    }

    #[test]
    fn test_countdown_cancel_options_orders_request_with_underlying() {
        let request = CountdownCancelOptionsOrdersRequest {
            timeout: 30,
            underlying: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 30);
        assert_eq!(json["underlying"], "BTC_USDT");
    }

    #[test]
    fn test_countdown_cancel_options_orders_different_timeouts() {
        let timeouts = vec![1, 5, 10, 30, 60, 120, 300, 600];

        for timeout in timeouts {
            let request = CountdownCancelOptionsOrdersRequest {
                timeout,
                underlying: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["timeout"], timeout);
        }
    }

    #[test]
    fn test_countdown_cancel_options_orders_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT", "BNB_USDT"];

        for underlying in underlyings {
            let request = CountdownCancelOptionsOrdersRequest {
                timeout: 60,
                underlying: Some(underlying.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["timeout"], 60);
            assert_eq!(json["underlying"], underlying);
        }
    }

    #[test]
    fn test_countdown_cancel_options_orders_max_timeout() {
        let request = CountdownCancelOptionsOrdersRequest {
            timeout: i32::MAX,
            underlying: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], i32::MAX);
    }

    #[test]
    fn test_countdown_cancel_options_orders_zero_timeout() {
        let request = CountdownCancelOptionsOrdersRequest {
            timeout: 0,
            underlying: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 0);
    }

    #[test]
    fn test_countdown_cancel_options_orders_negative_timeout() {
        let request = CountdownCancelOptionsOrdersRequest {
            timeout: -10,
            underlying: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], -10);
    }

    #[test]
    fn test_countdown_cancel_options_orders_serialization() {
        let request = CountdownCancelOptionsOrdersRequest {
            timeout: 45,
            underlying: Some("ETH_USDT".to_string()),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        assert!(json_str.contains("\"timeout\":45"));
        assert!(json_str.contains("\"underlying\":\"ETH_USDT\""));
    }

    #[test]
    fn test_countdown_cancel_options_orders_empty_underlying() {
        let request = CountdownCancelOptionsOrdersRequest {
            timeout: 15,
            underlying: Some("".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 15);
        assert_eq!(json["underlying"], "");
    }
}
