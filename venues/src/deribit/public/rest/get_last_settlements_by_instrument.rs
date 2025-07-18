//! Implements the /public/get_last_settlements_by_instrument endpoint for Deribit.
//!
//! Retrieves the most recent settlements for a given instrument.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

const LAST_SETTLEMENTS_BY_INSTRUMENT_ENDPOINT: &str = "public/get_last_settlements_by_instrument";

/// Request parameters for the get_last_settlements_by_instrument endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLastSettlementsByInstrumentRequest {
    /// Instrument name for which to retrieve settlements.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Number of results to return (default: 10, max: 1000).
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Represents a single settlement entry.
#[derive(Debug, Clone, Deserialize)]
pub struct SettlementEntry {
    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Settlement price.
    #[serde(rename = "settlement_price", skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<f64>,

    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// The result object for get_last_settlements_by_instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastSettlementsByInstrumentResult {
    /// List of settlement entries.
    #[serde(rename = "settlements")]
    pub settlements: Vec<SettlementEntry>,
}

/// Response for the get_last_settlements_by_instrument endpoint.
pub type GetLastSettlementsByInstrumentResponse =
    JsonRpcResult<GetLastSettlementsByInstrumentResult>;

impl RestClient {
    /// Calls the /public/get_last_settlements_by_instrument endpoint.
    ///
    /// Retrieves the most recent settlements for a given instrument.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_last_settlements_by_instrument)
    pub async fn get_last_settlements_by_instrument(
        &self,
        params: GetLastSettlementsByInstrumentRequest,
    ) -> RestResult<GetLastSettlementsByInstrumentResponse> {
        self.send_request(
            LAST_SETTLEMENTS_BY_INSTRUMENT_ENDPOINT,
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
        let req = GetLastSettlementsByInstrumentRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            count: Some(3),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("count"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 17,
            "jsonrpc": "2.0",
            "result": {
                "settlements": [
                    {
                        "instrument_name": "BTC-PERPETUAL",
                        "settlement_price": 65000.0,
                        "timestamp": 1680310800000
                    }
                ]
            }
        }"#;
        let resp: GetLastSettlementsByInstrumentResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 17);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.settlements.len(), 1);
        assert_eq!(resp.result.settlements[0].instrument_name, "BTC-PERPETUAL");
        assert!((resp.result.settlements[0].settlement_price.unwrap() - 65000.0).abs() < 1e-8);
        assert_eq!(resp.result.settlements[0].timestamp, 1680310800000);
    }
}
