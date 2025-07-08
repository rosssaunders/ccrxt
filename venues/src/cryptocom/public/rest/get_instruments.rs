//! Request and response structs for public/get-instruments endpoint
//!
//! Provides information on all supported instruments (e.g. BTCUSD-PERP).

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::EndpointType;
use crate::cryptocom::InstrumentType;
use crate::cryptocom::RestResult;

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
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentsResponse {
    /// Result data for instruments.
    #[serde(rename = "result")]
    pub result: InstrumentsResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

/// Result data for instruments.
#[derive(Debug, Clone, Deserialize)]
pub struct InstrumentsResult {
    /// List of instrument objects.
    #[serde(rename = "data")]
    pub data: Vec<Instrument>,
}

/// Instrument object.
#[derive(Debug, Clone, Deserialize)]
pub struct Instrument {
    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Instrument type.
    #[serde(rename = "instrument_type")]
    pub instrument_type: InstrumentType,

    /// Quote currency.
    #[serde(rename = "quote_currency")]
    pub quote_currency: Cow<'static, str>,

    /// Base currency.
    #[serde(rename = "base_currency")]
    pub base_currency: Cow<'static, str>,

    /// Price decimal places.
    #[serde(rename = "price_decimals")]
    pub price_decimals: u32,

    /// Quantity decimal places.
    #[serde(rename = "quantity_decimals")]
    pub quantity_decimals: u32,

    /// Maximum leverage allowed.
    #[serde(rename = "max_leverage")]
    pub max_leverage: f64,
}

impl RestClient {
    /// Calls the public/get-instruments endpoint.
    ///
    /// Provides information on all supported instruments.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html)
    pub async fn get_instruments(
        &self,
        params: GetInstrumentsRequest,
    ) -> RestResult<GetInstrumentsResponse> {
        self.send_request(
            GET_INSTRUMENTS_ENDPOINT,
            reqwest::Method::GET,
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
