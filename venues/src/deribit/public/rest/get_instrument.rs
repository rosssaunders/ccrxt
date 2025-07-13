//! Implements the /public/get_instrument endpoint for Deribit.
//!
//! Retrieves instrument data for a given instrument name.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::InstrumentKind};

const INSTRUMENT_ENDPOINT: &str = "public/get_instrument";

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

    /// The price index for the instrument.
    #[serde(rename = "price_index")]
    pub price_index: String,

    /// Whether RFQ is enabled for the instrument.
    #[serde(rename = "rfq")]
    pub rfq: bool,

    /// Maker commission.
    #[serde(rename = "maker_commission")]
    pub maker_commission: f64,

    /// Taker commission.
    #[serde(rename = "taker_commission")]
    pub taker_commission: f64,

    /// Type of the instrument.
    #[serde(rename = "instrument_type")]
    pub instrument_type: String,

    /// Timestamp when the instrument expires.
    #[serde(rename = "expiration_timestamp")]
    pub expiration_timestamp: u64,

    /// Timestamp when the instrument was created.
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: u64,

    /// Whether the instrument is active.
    #[serde(rename = "is_active")]
    pub is_active: bool,

    /// ID of the instrument.
    #[serde(rename = "instrument_id")]
    pub instrument_id: u64,

    /// Settlement period.
    #[serde(rename = "settlement_period")]
    pub settlement_period: String,

    /// Type of the future (if applicable).
    #[serde(rename = "future_type", skip_serializing_if = "Option::is_none")]
    pub future_type: Option<String>,

    /// Maximum leverage allowed.
    #[serde(rename = "max_leverage", skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<f64>,

    /// Maximum liquidation commission.
    #[serde(
        rename = "max_liquidation_commission",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_liquidation_commission: Option<f64>,

    /// Block trade commission.
    #[serde(
        rename = "block_trade_commission",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_commission: Option<f64>,

    /// Minimum block trade amount.
    #[serde(
        rename = "block_trade_min_trade_amount",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_min_trade_amount: Option<f64>,

    /// Block trade tick size.
    #[serde(
        rename = "block_trade_tick_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_tick_size: Option<f64>,

    /// Settlement currency.
    #[serde(
        rename = "settlement_currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub settlement_currency: Option<String>,

    /// Base currency of the instrument.
    #[serde(rename = "base_currency", skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<String>,

    /// Counter currency.
    #[serde(rename = "counter_currency", skip_serializing_if = "Option::is_none")]
    pub counter_currency: Option<String>,

    /// Quote currency for the instrument.
    #[serde(rename = "quote_currency", skip_serializing_if = "Option::is_none")]
    pub quote_currency: Option<String>,

    /// Steps for tick sizes (array).
    #[serde(rename = "tick_size_steps", default)]
    pub tick_size_steps: Vec<f64>,

    /// Strike price for options (optional).
    #[serde(rename = "strike", default, skip_serializing_if = "Option::is_none")]
    pub strike: Option<f64>,

    /// Option type (optional).
    #[serde(
        rename = "option_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_type: Option<String>,
}

/// The result object for get_instrument is the instrument data directly.
pub type GetInstrumentResult = InstrumentData;

/// Response for the get_instrument endpoint.
pub type GetInstrumentResponse = JsonRpcResult<GetInstrumentResult>;

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
            INSTRUMENT_ENDPOINT,
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
    use crate::deribit::enums::InstrumentKind;

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
                "instrument_name": "BTC-PERPETUAL",
                "price_index": "btc_usd",
                "rfq": false,
                "kind": "future",
                "tick_size": 0.5,
                "contract_size": 10.0,
                "min_trade_amount": 1.0,
                "instrument_type": "reversed",
                "expiration_timestamp": 32503708800000,
                "creation_timestamp": 1534242287000,
                "is_active": true,
                "instrument_id": 210838,
                "settlement_period": "perpetual",
                "maker_commission": 0.0,
                "taker_commission": 0.0005,
                "base_currency": "BTC",
                "counter_currency": "USD",
                "quote_currency": "USD",
                "tick_size_steps": []
            }
        }"#;
        let resp: GetInstrumentResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 14);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.instrument_name, "BTC-PERPETUAL");
        assert_eq!(resp.result.base_currency.as_deref().unwrap_or(""), "BTC");
        assert_eq!(resp.result.kind, InstrumentKind::Future);
        assert!((resp.result.tick_size - 0.5).abs() < 1e-8);
        assert!((resp.result.contract_size - 10.0).abs() < 1e-8);
        assert!((resp.result.min_trade_amount - 1.0).abs() < 1e-8);
    }
}
