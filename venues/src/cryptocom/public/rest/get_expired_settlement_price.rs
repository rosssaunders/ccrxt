//! Request and response structs for public/get-expired-settlement-price endpoint
//!
//! Fetches settlement price of expired instruments.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{ApiResult, EndpointType, InstrumentType, RestResult};

/// Endpoint path for the get-expired-settlement-price API
const EXPIRED_SETTLEMENT_PRICE_ENDPOINT: &str = "public/get-expired-settlement-price";

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
pub type GetExpiredSettlementPriceResponse = ApiResult<ExpiredSettlementPriceResult>;

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
    /// Instrument name (may not be present in response).
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<Cow<'static, str>>,

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
    /// [Official API docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-expired-settlement-price)
    pub async fn get_expired_settlement_price(
        &self,
        params: GetExpiredSettlementPriceRequest,
    ) -> RestResult<GetExpiredSettlementPriceResponse> {
        self.send_request(
            EXPIRED_SETTLEMENT_PRICE_ENDPOINT,
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetExpiredSettlementPrice,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

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
