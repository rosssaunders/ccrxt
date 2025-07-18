use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

const API_TRADING_STATUS_ENDPOINT: &str = "/dapi/v1/apiTradingStatus";

/// Request parameters for API trading status.
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradingStatusRequest {
    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Indicator information for trading status.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingIndicator {
    /// Indicator name
    #[serde(rename = "i")]
    pub indicator: String,

    /// Count
    #[serde(rename = "c")]
    pub count: u32,

    /// Current value
    #[serde(rename = "v")]
    pub current_value: f64,

    /// Trigger value
    #[serde(rename = "t")]
    pub trigger_value: f64,
}

/// Response for API trading status.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingStatusResponse {
    /// Is locked
    pub is_locked: bool,

    /// Planned recover time
    pub planned_recover_time: u64,

    /// Trigger condition
    pub trigger_condition: Option<TradingIndicator>,

    /// Indicators
    pub indicators: Option<Vec<TradingIndicator>>,

    /// Update time
    pub update_time: u64,
}

impl RestClient {
    /// Get API trading status on Binance Coin-M Futures.
    ///
    /// See: <https://binance-docs.github.io/apidocs/delivery/en/>
    /// GET /dapi/v1/apiTradingStatus
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`TradingStatusRequest`])
    ///
    /// # Returns
    /// A [`TradingStatusResponse`] object with trading status details.
    pub async fn get_trading_status(
        &self,
        params: TradingStatusRequest,
    ) -> RestResult<TradingStatusResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            API_TRADING_STATUS_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_status_request_serialization() {
        let request = TradingStatusRequest {
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_trading_status_request_with_recv_window() {
        let request = TradingStatusRequest {
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "recvWindow=5000");
    }

    #[test]
    fn test_trading_indicator_deserialization() {
        let json = r#"{
            "i": "GCR",
            "c": 150,
            "v": 0.05,
            "t": 0.15
        }"#;

        let indicator: TradingIndicator = serde_json::from_str(json).unwrap();
        assert_eq!(indicator.indicator, "GCR");
        assert_eq!(indicator.count, 150);
        assert_eq!(indicator.current_value, 0.05);
        assert_eq!(indicator.trigger_value, 0.15);
    }

    #[test]
    fn test_trading_status_response_not_locked() {
        let json = r#"{
            "isLocked": false,
            "plannedRecoverTime": 0,
            "triggerCondition": null,
            "indicators": null,
            "updateTime": 1625097600000
        }"#;

        let response: TradingStatusResponse = serde_json::from_str(json).unwrap();
        assert!(!response.is_locked);
        assert_eq!(response.planned_recover_time, 0);
        assert!(response.trigger_condition.is_none());
        assert!(response.indicators.is_none());
        assert_eq!(response.update_time, 1625097600000);
    }

    #[test]
    fn test_trading_status_response_locked() {
        let json = r#"{
            "isLocked": true,
            "plannedRecoverTime": 1625097700000,
            "triggerCondition": {
                "i": "GCR",
                "c": 200,
                "v": 0.20,
                "t": 0.15
            },
            "indicators": [
                {
                    "i": "UFR",
                    "c": 50,
                    "v": 0.95,
                    "t": 0.99
                },
                {
                    "i": "IFER",
                    "c": 100,
                    "v": 0.10,
                    "t": 0.05
                }
            ],
            "updateTime": 1625097600000
        }"#;

        let response: TradingStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.is_locked);
        assert_eq!(response.planned_recover_time, 1625097700000);
        
        let trigger = response.trigger_condition.unwrap();
        assert_eq!(trigger.indicator, "GCR");
        assert_eq!(trigger.count, 200);
        assert_eq!(trigger.current_value, 0.20);
        assert_eq!(trigger.trigger_value, 0.15);
        
        let indicators = response.indicators.unwrap();
        assert_eq!(indicators.len(), 2);
        
        assert_eq!(indicators[0].indicator, "UFR");
        assert_eq!(indicators[0].count, 50);
        assert_eq!(indicators[0].current_value, 0.95);
        assert_eq!(indicators[0].trigger_value, 0.99);
        
        assert_eq!(indicators[1].indicator, "IFER");
        assert_eq!(indicators[1].count, 100);
        assert_eq!(indicators[1].current_value, 0.10);
        assert_eq!(indicators[1].trigger_value, 0.05);
        
        assert_eq!(response.update_time, 1625097600000);
    }
}
