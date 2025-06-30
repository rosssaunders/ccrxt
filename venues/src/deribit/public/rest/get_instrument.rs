//! Implements the /public/get_instrument endpoint for Deribit.
//!
//! Retrieves instrument data for a given instrument name.

use super::RestClient;
use crate::deribit::enums::{Currency, InstrumentKind};
use crate::deribit::{EndpointType, RestResult};

use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Request parameters for the get_instrument endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetInstrumentRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
}

/// Instrument data returned by get_instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct InstrumentData {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Currency of the instrument.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Kind of the instrument (future, option, etc.).
    #[serde(rename = "kind")]
    pub kind: InstrumentKind,

    /// Tick size for the instrument.
    #[serde(rename = "tick_size")]
    pub tick_size: f64,

    /// Contract size for the instrument.
    #[serde(rename = "contract_size")]
    pub contract_size: f64,

    /// Minimum trade amount.
    #[serde(rename = "min_trade_amount")]
    pub min_trade_amount: f64,

    /// Optionally, the strike price (for options).
    #[serde(rename = "strike", skip_serializing_if = "Option::is_none")]
    pub strike: Option<f64>,

    /// Optionally, the expiration timestamp (for expiring instruments).
    #[serde(
        rename = "expiration_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_timestamp: Option<u64>,

    /// Optionally, the creation timestamp.
    #[serde(rename = "creation_timestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<u64>,

    /// Optionally, the settlement period (for expiring instruments).
    #[serde(rename = "settlement_period", skip_serializing_if = "Option::is_none")]
    pub settlement_period: Option<String>,

    /// Optionally, the base currency (for spot pairs).
    #[serde(rename = "base_currency", skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<Currency>,

    /// Optionally, the quote currency (for spot pairs).
    #[serde(rename = "quote_currency", skip_serializing_if = "Option::is_none")]
    pub quote_currency: Option<Currency>,
}

/// The result object for get_instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentResult {
    /// Instrument data.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentData,
}

/// Response for the get_instrument endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing instrument data.
    #[serde(rename = "result")]
    pub result: GetInstrumentResult,
}

impl RestClient {
    /// Calls the /public/get_instrument endpoint.
    ///
    /// Retrieves instrument data for a given instrument name.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_instrument)
    pub async fn get_instrument(
        &self,
        params: GetInstrumentRequest,
    ) -> RestResult<GetInstrumentResponse> {
        self.send_request(
            "public/get_instrument",
            Method::POST,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::enums::Currency;
    use crate::deribit::enums::InstrumentKind;
    use serde_json;

    #[test]
    fn test_serialize_request() {
        let req = GetInstrumentRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 14,
            "jsonrpc": "2.0",
            "result": {
                "instrument": {
                    "instrument_name": "BTC-PERPETUAL",
                    "currency": "BTC",
                    "kind": "future",
                    "tick_size": 0.5,
                    "contract_size": 10.0,
                    "min_trade_amount": 1.0
                }
            }
        }"#;
        let resp: GetInstrumentResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 14);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.instrument.instrument_name, "BTC-PERPETUAL");
        assert_eq!(resp.result.instrument.currency, Currency::BTC);
        assert_eq!(resp.result.instrument.kind, InstrumentKind::Future);
        assert!((resp.result.instrument.tick_size - 0.5).abs() < 1e-8);
        assert!((resp.result.instrument.contract_size - 10.0).abs() < 1e-8);
        assert!((resp.result.instrument.min_trade_amount - 1.0).abs() < 1e-8);
    }
}
