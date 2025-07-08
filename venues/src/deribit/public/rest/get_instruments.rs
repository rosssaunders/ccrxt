//! Implements the /public/get_instruments endpoint for Deribit.
//!
//! Retrieves a list of instruments for a given currency and kind.

use super::RestClient;
use crate::deribit::enums::{Currency, InstrumentKind};
use crate::deribit::{EndpointType, RestResult};

use reqwest::Method;
use serde::{Deserialize, Serialize};

const INSTRUMENTS_ENDPOINT: &str = "public/get_instruments";

/// Request parameters for the get_instruments endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct GetInstrumentsRequest {
    /// Currency for which to retrieve instruments.
    #[serde(rename = "currency")]
    pub currency: Currency,

    /// Kind of the instrument (future, option, etc.).
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,

    /// Whether to include expired instruments (default: false).
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
}

/// Instrument data returned by get_instruments.
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

/// The result object for get_instruments.
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentsResult {
    /// List of instrument data.
    #[serde(rename = "instruments")]
    pub instruments: Vec<InstrumentData>,
}

/// Response for the get_instruments endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentsResponse {
    /// The id that was sent in the request.
    #[serde(rename = "id")]
    pub id: u64,

    /// The JSON-RPC version (2.0).
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// The result object containing the list of instruments.
    #[serde(rename = "result")]
    pub result: GetInstrumentsResult,
}

impl RestClient {
    /// Calls the /public/get_instruments endpoint.
    ///
    /// Retrieves a list of instruments for a given currency and kind.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_instruments)
    pub async fn get_instruments(
        &self,
        params: GetInstrumentsRequest,
    ) -> RestResult<GetInstrumentsResponse> {
        self.send_request(
            INSTRUMENTS_ENDPOINT,
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
        let req = GetInstrumentsRequest {
            currency: Currency::BTC,
            kind: Some(InstrumentKind::Future),
            expired: Some(false),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("future"));
        assert!(json.contains("expired"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 15,
            "jsonrpc": "2.0",
            "result": {
                "instruments": [
                    {
                        "instrument_name": "BTC-PERPETUAL",
                        "currency": "BTC",
                        "kind": "future",
                        "tick_size": 0.5,
                        "contract_size": 10.0,
                        "min_trade_amount": 1.0
                    }
                ]
            }
        }"#;
        let resp: GetInstrumentsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 15);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.instruments.len(), 1);
        assert_eq!(resp.result.instruments[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(resp.result.instruments[0].currency, Currency::BTC);
        assert_eq!(resp.result.instruments[0].kind, InstrumentKind::Future);
        assert!((resp.result.instruments[0].tick_size - 0.5).abs() < 1e-8);
        assert!((resp.result.instruments[0].contract_size - 10.0).abs() < 1e-8);
        assert!((resp.result.instruments[0].min_trade_amount - 1.0).abs() < 1e-8);
    }
}
