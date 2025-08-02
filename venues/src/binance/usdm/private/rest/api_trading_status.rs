use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const API_TRADING_STATUS_ENDPOINT: &str = "/fapi/v1/apiTradingStatus";

/// Request parameters for the API trading status endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetApiTradingStatusRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Optional. If omitted, returns account-level indicators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// The number of milliseconds after timestamp the request is valid for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch. Required.
    pub timestamp: u64,
}

/// Represents a single quantitative rule indicator for a symbol or account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantitativeRuleIndicator {
    /// Indicator type (e.g., "UFR", "IFER", "GCR", "DR", "TMV").
    pub indicator: String,

    /// Current value of the indicator.
    pub value: f64,

    /// Trigger value threshold for the indicator.
    pub trigger_value: f64,

    /// Planned recovery time in milliseconds (0 if no recovery is planned).
    pub planned_recover_time: u64,

    /// Whether the trading function is locked due to this indicator.
    pub is_locked: bool,
}

/// Response from the API trading status endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTradingStatusResponse {
    /// Quantitative rules indicators for each symbol or account.
    /// The key is the symbol (e.g., "BTCUSDT") or "ACCOUNT" for account-level violations.
    pub indicators: std::collections::HashMap<String, Vec<QuantitativeRuleIndicator>>,

    /// Update time of this status in milliseconds since epoch.
    pub update_time: u64,
}

impl UsdmClient {
    /// Futures Trading Quantitative Rules Indicators
    ///
    /// Retrieves the quantitative rules indicators for the account or a specific symbol.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Futures-Trading-Quantitative-Rules-Indicators
    ///
    /// Rate limit: 1 (single symbol), 10 (all symbols)
    ///
    /// # Arguments
    /// * `params` - The request parameters for API trading status
    ///
    /// # Returns
    /// Returns the quantitative rules indicators for the account or symbol.
    pub async fn get_api_trading_status(
        &self,
        params: GetApiTradingStatusRequest,
    ) -> RestResult<ApiTradingStatusResponse> {
        self.send_get_signed_request(API_TRADING_STATUS_ENDPOINT, params, 10, false)
            .await
    }
}
#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_api_trading_status_request_serialization() {
        let req = GetApiTradingStatusRequest {
            symbol: Some("BTCUSDT".to_string()),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&req).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_api_trading_status_response_deserialization() {
        let json = r#"{
            "indicators": {
                "BTCUSDT": [
                    {
                        "indicator": "UFR",
                        "value": 0.05,
                        "triggerValue": 0.995,
                        "plannedRecoverTime": 1545741270000,
                        "isLocked": true
                    },
                    {
                        "indicator": "IFER",
                        "value": 0.99,
                        "triggerValue": 0.99,
                        "plannedRecoverTime": 1545741270000,
                        "isLocked": true
                    }
                ],
                "ACCOUNT": [
                    {
                        "indicator": "TMV",
                        "value": 10.0,
                        "triggerValue": 1.0,
                        "plannedRecoverTime": 1644919865000,
                        "isLocked": true
                    }
                ]
            },
            "updateTime": 1644913304748
        }"#;
        let resp: ApiTradingStatusResponse = serde_json::from_str(json).unwrap();
        assert!(resp.indicators.contains_key("BTCUSDT"));
        assert!(resp.indicators.contains_key("ACCOUNT"));
        let btc_indicators = &resp.indicators["BTCUSDT"];
        assert_eq!(btc_indicators[0].indicator, "UFR");
        assert_eq!(btc_indicators[0].value, 0.05);
        assert_eq!(btc_indicators[0].trigger_value, 0.995);
        assert_eq!(btc_indicators[0].planned_recover_time, 1545741270000);
        assert!(btc_indicators[0].is_locked);
        let account_indicators = &resp.indicators["ACCOUNT"];
        assert_eq!(account_indicators[0].indicator, "TMV");
        assert_eq!(account_indicators[0].value, 10.0);
        assert_eq!(account_indicators[0].trigger_value, 1.0);
        assert_eq!(account_indicators[0].planned_recover_time, 1644919865000);
        assert!(account_indicators[0].is_locked);
        assert_eq!(resp.update_time, 1644913304748);
    }
}
