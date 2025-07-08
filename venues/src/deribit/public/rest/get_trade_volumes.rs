//! Implements the /public/get_trade_volumes endpoint for Deribit.
//!
//! Retrieves the trade volumes for all supported currencies.


use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::Currency};

const TRADE_VOLUMES_ENDPOINT: &str = "public/get_trade_volumes";

/// Request parameters for the get_trade_volumes endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetTradeVolumesRequest {}

/// Represents a single trade volume entry.
#[derive(Debug, Clone, Deserialize)]
pub struct TradeVolumeEntry {
    /// Currency for which the volume is reported.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Volume traded in the last 24 hours.
    #[serde(rename = "volume_24h")]
    pub volume_24h: f64,

    /// Volume traded in the last 30 days.
    #[serde(rename = "volume_30d")]
    pub volume_30d: f64,
}

/// The result object for get_trade_volumes.
#[derive(Debug, Clone, Deserialize)]
pub struct GetTradeVolumesResult {
    /// List of trade volume entries.
    #[serde(rename = "volumes")]
    pub volumes: Vec<TradeVolumeEntry>,
}

/// Response for the get_trade_volumes endpoint.
pub type GetTradeVolumesResponse = JsonRpcResult<GetTradeVolumesResult>;

impl RestClient {
    /// Calls the /public/get_trade_volumes endpoint.
    ///
    /// Retrieves the trade volumes for all supported currencies.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_trade_volumes)
    pub async fn get_trade_volumes(
        &self,
        params: GetTradeVolumesRequest,
    ) -> RestResult<GetTradeVolumesResponse> {
        self.send_request(
            TRADE_VOLUMES_ENDPOINT,

            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;
    use crate::deribit::enums::Currency;

    #[test]
    fn test_serialize_request() {
        let req = GetTradeVolumesRequest {};
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("{}"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 27,
            "jsonrpc": "2.0",
            "result": {
                "volumes": [
                    {
                        "currency": "BTC",
                        "volume_24h": 12345.6,
                        "volume_30d": 789012.3
                    }
                ]
            }
        }"#;
        let resp: GetTradeVolumesResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 27);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.volumes.len(), 1);
        let entry = &resp.result.volumes[0];
        assert_eq!(entry.currency, Currency::BTC);
        assert!((entry.volume_24h - 12345.6).abs() < 1e-8);
        assert!((entry.volume_30d - 789012.3).abs() < 1e-8);
    }
}
