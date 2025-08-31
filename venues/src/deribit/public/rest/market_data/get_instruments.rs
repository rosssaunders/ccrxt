//! Implements the /public/get_instruments endpoint for Deribit.
//!
//! Retrieves a list of instruments for a given currency and kind.

use serde::{Deserialize, Serialize};

use crate::deribit::{
    EndpointType, JsonRpcResult, PublicRestClient, RestResult,
    enums::{Currency, FutureType, InstrumentKind, InstrumentType, OptionType},
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

/// Tick size step configuration for instruments.
#[derive(Debug, Clone, Deserialize)]
pub struct TickSizeStep {
    /// The price from which the increased tick size applies.
    #[serde(rename = "above_price")]
    pub above_price: f64,

    /// Tick size to be used above the price. It must be multiple of the minimum tick size.
    #[serde(rename = "tick_size")]
    pub tick_size: f64,
}

/// Instrument data returned by get_instruments.
#[derive(Debug, Clone, Deserialize)]
pub struct InstrumentData {
    /// The underlying currency being traded.
    #[serde(rename = "base_currency")]
    pub base_currency: String,

    /// Block Trade commission for instrument.
    #[serde(
        rename = "block_trade_commission",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_commission: Option<f64>,

    /// Minimum amount for block trading.
    #[serde(
        rename = "block_trade_min_trade_amount",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_min_trade_amount: Option<f64>,

    /// Specifies minimal price change for block trading.
    #[serde(
        rename = "block_trade_tick_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_tick_size: Option<f64>,

    /// Contract size for instrument.
    #[serde(rename = "contract_size")]
    pub contract_size: f64,

    /// Counter currency for the instrument.
    #[serde(rename = "counter_currency")]
    pub counter_currency: String,

    /// The time when the instrument was first created (milliseconds since the UNIX epoch).
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: u64,

    /// The time when the instrument will expire (milliseconds since the UNIX epoch).
    #[serde(rename = "expiration_timestamp")]
    pub expiration_timestamp: u64,

    /// Future type (only for futures)(field is deprecated and will be removed in the future, instrument_type should be used instead).
    #[serde(rename = "future_type", skip_serializing_if = "Option::is_none")]
    pub future_type: Option<FutureType>,

    /// Instrument ID.
    #[serde(rename = "instrument_id")]
    pub instrument_id: u64,

    /// Unique instrument identifier.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Type of the instrument. linear or reversed.
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<InstrumentType>,

    /// Indicates if the instrument can currently be traded.
    #[serde(rename = "is_active")]
    pub is_active: bool,

    /// Instrument kind: "future", "option", "spot", "future_combo", "option_combo".
    #[serde(rename = "kind")]
    pub kind: InstrumentKind,

    /// Maker commission for instrument.
    #[serde(rename = "maker_commission")]
    pub maker_commission: f64,

    /// Maximal leverage for instrument (only for futures).
    #[serde(rename = "max_leverage", skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<u64>,

    /// Maximal liquidation trade commission for instrument (only for futures).
    #[serde(
        rename = "max_liquidation_commission",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_liquidation_commission: Option<f64>,

    /// Minimum amount for trading. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
    #[serde(rename = "min_trade_amount")]
    pub min_trade_amount: f64,

    /// The option type (only for options).
    #[serde(rename = "option_type", skip_serializing_if = "Option::is_none")]
    pub option_type: Option<OptionType>,

    /// Name of price index that is used for this instrument.
    #[serde(rename = "price_index")]
    pub price_index: String,

    /// The currency in which the instrument prices are quoted.
    #[serde(rename = "quote_currency")]
    pub quote_currency: String,

    /// Whether or not RFQ is active on the instrument.
    #[serde(rename = "rfq")]
    pub rfq: bool,

    /// Optional (not added for spot). Settlement currency for the instrument.
    #[serde(
        rename = "settlement_currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub settlement_currency: Option<String>,

    /// Optional (not added for spot). The settlement period.
    #[serde(rename = "settlement_period", skip_serializing_if = "Option::is_none")]
    pub settlement_period: Option<String>,

    /// The strike value (only for options).
    #[serde(rename = "strike", skip_serializing_if = "Option::is_none")]
    pub strike: Option<f64>,

    /// Taker commission for instrument.
    #[serde(rename = "taker_commission")]
    pub taker_commission: f64,

    /// Specifies minimal price change and, as follows, the number of decimal places for instrument prices.
    #[serde(rename = "tick_size")]
    pub tick_size: f64,

    /// Tick size steps configuration.
    #[serde(rename = "tick_size_steps", skip_serializing_if = "Option::is_none")]
    pub tick_size_steps: Option<Vec<TickSizeStep>>,
}

/// Response for public/get_instruments endpoint following Deribit JSON-RPC 2.0 format.
pub type GetInstrumentsResponse = JsonRpcResult<Vec<InstrumentData>>;

impl PublicRestClient {
    /// Calls the /public/get_instruments endpoint.
    ///
    /// Retrieves a list of instruments for a given currency and kind.
    ///
    /// [docs](https://docs.deribit.com/#public-get_instruments)
    pub async fn get_instruments(
        &self,
        params: GetInstrumentsRequest,
    ) -> RestResult<GetInstrumentsResponse> {
        self.send_post_request(
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
                    "base_currency": "BTC",
                    "block_trade_commission": 0.0003,
                    "block_trade_min_trade_amount": 25000,
                    "block_trade_tick_size": 0.5,
                    "contract_size": 10,
                    "counter_currency": "USD",
                    "creation_timestamp": 1569888000000,
                    "expiration_timestamp": 32503680000000,
                    "instrument_id": 1234,
                    "instrument_name": "BTC-PERPETUAL",
                    "instrument_type": "reversed",
                    "is_active": true,
                    "kind": "future",
                    "maker_commission": 0.0001,
                    "min_trade_amount": 1.0,
                    "price_index": "btc_usd",
                    "quote_currency": "USD",
                    "rfq": false,
                    "taker_commission": 0.0005,
                    "tick_size": 0.5
                }
            ]
        }"#;
        let resp: GetInstrumentsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 1);
        assert_eq!(resp.result[0].instrument_name, "BTC-PERPETUAL");
        assert_eq!(resp.result[0].base_currency, "BTC");
        assert_eq!(resp.result[0].kind, InstrumentKind::Future);
        assert!((resp.result[0].tick_size - 0.5).abs() < 1e-8);
        assert_eq!(resp.result[0].contract_size, 10.0);
        assert!((resp.result[0].min_trade_amount - 1.0).abs() < 1e-8);
        assert!(resp.result[0].is_active);
        assert!(!resp.result[0].rfq);
    }
}
