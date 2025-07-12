//! Implements the /public/get_trade_volumes endpoint for Deribit.
//!
//! Retrieves the trade volumes for all supported currencies.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const TRADE_VOLUMES_ENDPOINT: &str = "public/get_trade_volumes";

/// Request parameters for the get_trade_volumes endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetTradeVolumesRequest {}

/// Represents a single trade volume entry.
#[derive(Debug, Clone, Deserialize)]
pub struct TradeVolumeEntry {
    /// Currency for which the volume is reported.
    #[serde(rename = "currency")]
    pub currency: String,

    /// Currency pair for which the volume is reported.
    #[serde(rename = "currency_pair")]
    pub currency_pair: String,

    /// Volume for futures in the last 24 hours.
    #[serde(rename = "futures_volume")]
    pub futures_volume: f64,

    /// Volume for calls in the last 24 hours.
    #[serde(rename = "calls_volume")]
    pub calls_volume: f64,

    /// Volume for puts in the last 24 hours.
    #[serde(rename = "puts_volume")]
    pub puts_volume: f64,

    /// Volume for spot in the last 24 hours.
    #[serde(rename = "spot_volume")]
    pub spot_volume: f64,
}

/// Response for the get_trade_volumes endpoint.
pub type GetTradeVolumesResponse = JsonRpcResult<Vec<TradeVolumeEntry>>;

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
            "result": [
                {
                    "calls_volume": 22403.1,
                    "currency": "BTC",
                    "currency_pair": "btc_usd",
                    "futures_volume": 9208.77695393,
                    "puts_volume": 12744.4,
                    "spot_volume": 73.55579536
                },
                {
                    "calls_volume": 0.0,
                    "currency": "ETH",
                    "currency_pair": "eth_usd",
                    "futures_volume": 135725.742808,
                    "puts_volume": 96772.0,
                    "spot_volume": 4240.871183
                }
            ]
        }"#;
        let resp: GetTradeVolumesResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 27);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 2);

        let entry1 = &resp.result[0];
        assert_eq!(entry1.currency, "BTC");
        assert_eq!(entry1.currency_pair, "btc_usd");
        assert!((entry1.futures_volume - 9208.77695393).abs() < 1e-8);
        assert!((entry1.calls_volume - 22403.1).abs() < 1e-8);
        assert!((entry1.puts_volume - 12744.4).abs() < 1e-8);
        assert!((entry1.spot_volume - 73.55579536).abs() < 1e-8);

        let entry2 = &resp.result[1];
        assert_eq!(entry2.currency, "ETH");
        assert_eq!(entry2.currency_pair, "eth_usd");
        assert!((entry2.futures_volume - 135725.742808).abs() < 1e-8);
    }
}
