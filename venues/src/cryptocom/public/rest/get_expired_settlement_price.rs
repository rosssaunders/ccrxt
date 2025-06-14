//! Request and response structs for public/get-expired-settlement-price endpoint
//!
//! Fetches settlement price of expired instruments.

use super::client::RestClient;
use crate::cryptocom::EndpointType;
use crate::cryptocom::InstrumentType;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request parameters for the public/get-expired-settlement-price endpoint.
///
/// Fetches settlement price of expired instruments.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetExpiredSettlementPriceRequest {
    /// Instrument type (e.g., "PERPETUAL", "FUTURE"). Optional.
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<InstrumentType>,

    /// Instrument name. Optional.
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<Cow<'static, str>>,
}

/// Response for public/get-expired-settlement-price endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetExpiredSettlementPriceResponse {
    /// Result data for expired settlement prices.
    #[serde(rename = "result")]
    pub result: ExpiredSettlementPriceResult,

    /// Success status.
    #[serde(rename = "success")]
    pub success: bool,

    /// Response ID.
    #[serde(rename = "id")]
    pub id: u64,
}

/// Result data for expired settlement prices.
#[derive(Debug, Clone, Deserialize)]
pub struct ExpiredSettlementPriceResult {
    /// List of settlement price data.
    #[serde(rename = "data")]
    pub data: Vec<SettlementPrice>,
}

/// Settlement price data for an expired instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct SettlementPrice {
    /// Instrument name.
    #[serde(rename = "instrument_name")]
    pub instrument_name: Cow<'static, str>,

    /// Settlement price.
    #[serde(rename = "settlement_price")]
    pub settlement_price: f64,

    /// Settlement time (milliseconds since epoch).
    #[serde(rename = "settlement_time")]
    pub settlement_time: u64,
}

impl RestClient {
    /// Calls the public/get-expired-settlement-price endpoint.
    ///
    /// Fetches settlement price of expired instruments.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/spot/index.html#public-get-expired-settlement-price)
    pub async fn get_expired_settlement_price(
        &self,
        params: GetExpiredSettlementPriceRequest,
    ) -> RestResult<GetExpiredSettlementPriceResponse> {
        self.send_request(
            "public/get-expired-settlement-price",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetExpiredSettlementPrice,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_expired_settlement_price_endpoint_type() {
        let expired_settlement_endpoint = EndpointType::PublicGetExpiredSettlementPrice;
        assert!(expired_settlement_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_expired_settlement_price_parameter_building() {
        let params = json!({
            "instrument_type": "FUTURE",
            "page": 1
        });
        assert_eq!(params.get("instrument_type").unwrap(), "FUTURE");
        assert_eq!(params.get("page").unwrap(), 1);
    }
}
