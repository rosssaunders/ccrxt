use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Endpoint path for Open Interest
const OPEN_INTEREST_ENDPOINT: &str = "/dapi/v1/openInterest";

/// Request parameters for the Open Interest endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenInterestRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Required.
    /// Must match a valid COIN-M futures symbol.
    pub symbol: String,
}

/// Response data for the Open Interest endpoint.
///
/// Contains open interest and related metadata for a COIN-M futures symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    /// Trading symbol.
    pub symbol: String,

    /// Trading pair (e.g., "BTCUSD").
    pub pair: String,

    /// Open interest value.
    pub open_interest: Decimal,

    /// Contract type (e.g., "CURRENT_QUARTER").
    pub contract_type: String,

    /// Timestamp (milliseconds since epoch).
    pub time: i64,
}

impl RestClient {
    /// Open Interest
    ///
    /// Retrieves the present open interest of a specific symbol.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Open-Interest
    ///
    /// Rate limit: 1 request weight per call
    ///
    /// # Arguments
    /// * `params` - The request parameters for Open Interest
    ///
    /// # Returns
    /// Open interest data for the specified symbol
    pub async fn get_open_interest(&self, params: OpenInterestRequest) -> RestResult<OpenInterest> {
        self.send_get_request(OPEN_INTEREST_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_open_interest_request_serialization() {
        let request = OpenInterestRequest {
            symbol: "BTCUSD_PERP".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD_PERP");
    }

    #[test]
    fn test_open_interest_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSD_200626",
            "pair": "BTCUSD",
            "openInterest": "1234567.8901",
            "contractType": "CURRENT_QUARTER",
            "time": 1699948800000
        }"#;

        let response: OpenInterest = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSD_200626");
        assert_eq!(response.pair, "BTCUSD");
        assert_eq!(response.open_interest, Decimal::new(12345678901, 4));
        assert_eq!(response.contract_type, "CURRENT_QUARTER");
        assert_eq!(response.time, 1699948800000);
    }

    #[test]
    fn test_open_interest_response_with_zero_interest() {
        let json = r#"{
            "symbol": "ETHUSD_240329",
            "pair": "ETHUSD",
            "openInterest": "0.0000",
            "contractType": "CURRENT_QUARTER",
            "time": 1625097600000
        }"#;

        let response: OpenInterest = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSD_240329");
        assert_eq!(response.pair, "ETHUSD");
        assert_eq!(response.open_interest, Decimal::ZERO);
        assert_eq!(response.contract_type, "CURRENT_QUARTER");
        assert_eq!(response.time, 1625097600000);
    }
}
