//! Implements the /public/get_last_settlements_by_currency endpoint for Deribit.
//!
//! Retrieves the most recent settlements for a given currency and instrument kind.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    EndpointType, RestResult,
    enums::{Currency, InstrumentKind},
};

const LAST_SETTLEMENTS_BY_CURRENCY_ENDPOINT: &str = "public/get_last_settlements_by_currency";

/// Request parameters for the get_last_settlements_by_currency endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetLastSettlementsByCurrencyRequest {
    /// Currency for which to retrieve settlements.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Kind of the instrument (future, option, etc.).
    #[serde(rename = "kind")]
    pub kind: InstrumentKind,

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
    #[serde(rename = "settlement_price")]
    pub settlement_price: f64,

    /// Timestamp in milliseconds since epoch.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// The result object for get_last_settlements_by_currency.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastSettlementsByCurrencyResult {
    /// List of settlement entries.
    #[serde(rename = "settlements")]
    pub settlements: Vec<SettlementEntry>,
}

/// Response for the get_last_settlements_by_currency endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetLastSettlementsByCurrencyResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the settlements.
    #[serde(rename = "result")]
    pub result: GetLastSettlementsByCurrencyResult,
}

impl RestClient {
    /// Calls the /public/get_last_settlements_by_currency endpoint.
    ///
    /// Retrieves the most recent settlements for a given currency and instrument kind.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_last_settlements_by_currency)
    pub async fn get_last_settlements_by_currency(
        &self,
        params: GetLastSettlementsByCurrencyRequest,
    ) -> RestResult<GetLastSettlementsByCurrencyResponse> {
        self.send_request(
            LAST_SETTLEMENTS_BY_CURRENCY_ENDPOINT,
            Method::POST,
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
    use crate::deribit::enums::{Currency, InstrumentKind};

    #[test]
    fn test_serialize_request() {
        let req = GetLastSettlementsByCurrencyRequest {
            currency: Currency::BTC,
            kind: InstrumentKind::Future,
            count: Some(5),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("future"));
        assert!(json.contains("count"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 16,
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
        let resp: GetLastSettlementsByCurrencyResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 16);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.settlements.len(), 1);
        assert_eq!(resp.result.settlements[0].instrument_name, "BTC-PERPETUAL");
        assert!((resp.result.settlements[0].settlement_price - 65000.0).abs() < 1e-8);
        assert_eq!(resp.result.settlements[0].timestamp, 1680310800000);
    }
}
