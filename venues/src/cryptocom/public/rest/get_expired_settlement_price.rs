//! Request and response structs for public/get-expired-settlement-price endpoint
//!
//! Fetches settlement price of expired instruments.

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{ApiResult, EndpointType, InstrumentType, RestResult};

/// Endpoint path for the get-expired-settlement-price API
const EXPIRED_SETTLEMENT_PRICE_ENDPOINT: &str = "public/get-expired-settlement-price";

/// Request parameters for the public/get-expired-settlement-price endpoint.
///
/// Fetches settlement price of expired instruments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetExpiredSettlementPriceRequest {
    /// Instrument type. Required. Only FUTURE is currently supported by the API.
    pub instrument_type: InstrumentType,

    /// Page number for pagination. Optional. Defaults to 1 when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

/// Response for public/get-expired-settlement-price endpoint.
pub type GetExpiredSettlementPriceResponse = ApiResult<ExpiredSettlementPriceResult>;

/// Result data for expired settlement prices.
#[derive(Debug, Clone, Deserialize)]
pub struct ExpiredSettlementPriceResult {
    /// List of settlement price data.
    pub data: Vec<SettlementPrice>,
}

/// Settlement price data for an expired instrument.
#[derive(Debug, Clone, Deserialize)]
pub struct SettlementPrice {
    /// Instrument name.
    #[serde(rename = "i")]
    pub instrument_name: String,

    /// Expiry timestamp (milliseconds since epoch).
    #[serde(rename = "x")]
    pub expiry_timestamp_ms: u64,

    /// Settlement price as a string value.
    #[serde(rename = "v")]
    pub settlement_value: String,

    /// Settlement timestamp (milliseconds since epoch).
    #[serde(rename = "t")]
    pub timestamp_ms: u64,
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
        self.send_get_request(
            EXPIRED_SETTLEMENT_PRICE_ENDPOINT,
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
    fn test_expired_settlement_price_request_serialization() {
        let request = GetExpiredSettlementPriceRequest {
            instrument_type: InstrumentType::Future,
            page: Some(1),
        };

        let value = serde_json::to_value(&request).unwrap();
        assert_eq!(value.get("instrument_type").unwrap(), "FUTURE");
        assert_eq!(value.get("page").unwrap(), 1);
    }

    #[test]
    fn test_expired_settlement_price_response_deserialization() {
        let response_json = json!({
            "id": -1,
            "method": "public/get-expired-settlement-price",
            "code": 0,
            "result": {
                "data": [
                    { "i": "BTCUSD-210528m2", "x": 1622145600000u64, "v": "50776.73000", "t": 1622145540000u64 },
                    { "i": "BTCUSD-210528m3", "x": 1622160000000u64, "v": "38545.570000", "t": 1622159940000u64 }
                ]
            }
        });

        let parsed: GetExpiredSettlementPriceResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(parsed.code, 0);
        assert_eq!(parsed.result.data.len(), 2);
        let first = &parsed.result.data[0];
        assert_eq!(first.instrument_name, "BTCUSD-210528m2");
        assert_eq!(first.expiry_timestamp_ms, 1622145600000);
        assert_eq!(first.settlement_value, "50776.73000");
        assert_eq!(first.timestamp_ms, 1622145540000);
    }
}
