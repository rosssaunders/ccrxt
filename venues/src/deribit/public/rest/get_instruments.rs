//! Implements the /public/get_instruments endpoint for Deribit.
//!
//! Retrieves a list of instruments for a given currency and kind.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    EndpointType, JsonRpcResult, RestResult,
    enums::{Currency, InstrumentKind},
};

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

    /// Currency of the instrument (optional for some instruments).
    #[serde(rename = "currency", default)]
    pub currency: Option<Currency>,

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

/// Response for public/get_instruments endpoint following Deribit JSON-RPC 2.0 format.
pub type GetInstrumentsResponse = JsonRpcResult<Vec<InstrumentData>>;

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
            "result": [
                {
                    "instrument_name": "BTC-PERPETUAL",
                    "currency": "BTC",
                    "kind": "future",
                    "tick_size": 0.5,
                    "contract_size": 10.0,
                    "min_trade_amount": 1.0
                }
            ]
        }"#;
        let resp: GetInstrumentsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 1);
        assert_eq!(resp.result[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(resp.result[0].currency, Some(Currency::BTC));
        assert_eq!(resp.result[0].kind, InstrumentKind::Future);
        assert!((resp.result[0].tick_size - 0.5).abs() < 1e-8);
        assert!((resp.result[0].contract_size - 10.0).abs() < 1e-8);
        assert!((resp.result[0].min_trade_amount - 1.0).abs() < 1e-8);
    }
}
