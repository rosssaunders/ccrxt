use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{ApiResult, EndpointType, InstrumentType, RestResult};

/// Endpoint for getting instruments
const GET_INSTRUMENTS_ENDPOINT: &str = "public/get-instruments";

/// Request parameters for the public/get-instruments endpoint.
///
/// Provides information on all supported instruments.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetInstrumentsRequest {
    /// Instrument type (e.g., "PERPETUAL", "FUTURE"). Optional.
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<InstrumentType>,

    /// Instrument name. Optional.
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<Cow<'static, str>>,
}

/// Response for public/get-instruments endpoint.
pub type GetInstrumentsResponse = ApiResult<InstrumentsResult>;

/// Result data for instruments.
#[derive(Debug, Clone, Deserialize)]
pub struct InstrumentsResult {
    /// List of instrument objects.
    pub data: Vec<Instrument>,
}

/// Instrument object.
#[derive(Debug, Clone, Deserialize)]
pub struct Instrument {
    /// Symbol (instrument name).
    pub symbol: String,

    /// Instrument type.
    pub inst_type: String,

    /// Display name.
    pub display_name: String,

    /// Base currency.
    pub base_ccy: String,

    /// Quote currency.
    pub quote_ccy: String,

    /// Quote decimal places.
    pub quote_decimals: u32,

    /// Quantity decimal places.
    pub quantity_decimals: u32,

    /// Price tick size.
    pub price_tick_size: String,

    /// Quantity tick size.
    pub qty_tick_size: String,

    /// Maximum leverage allowed (returned as string by API).
    pub max_leverage: String,

    /// Whether the instrument is tradable.
    pub tradable: bool,

    /// Expiry timestamp in milliseconds.
    pub expiry_timestamp_ms: u64,

    /// Whether this is a beta product.
    pub beta_product: bool,

    /// Underlying symbol (for derivatives).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_symbol: Option<String>,

    /// Contract size (for derivatives).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_size: Option<String>,

    /// Whether margin buying is enabled.
    pub margin_buy_enabled: bool,

    /// Whether margin selling is enabled.
    pub margin_sell_enabled: bool,
}

impl RestClient {
    /// Calls the public/get-instruments endpoint.
    ///
    /// Provides information on all supported instruments.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-instruments)
    pub async fn get_instruments(
        &self,
        params: GetInstrumentsRequest,
    ) -> RestResult<GetInstrumentsResponse> {
        self.send_get_request(
            GET_INSTRUMENTS_ENDPOINT,
            Some(&params),
            EndpointType::PublicGetInstruments,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruments_endpoint_type() {
        let instruments_endpoint = EndpointType::PublicGetInstruments;
        assert!(instruments_endpoint.rate_limit().max_requests > 0);
    }
}
