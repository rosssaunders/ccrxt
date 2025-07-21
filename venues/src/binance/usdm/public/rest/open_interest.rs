use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

const OPEN_INTEREST_ENDPOINT: &str = "/fapi/v1/openInterest";

/// Request parameters for the open interest endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenInterestRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Required parameter to specify which symbol's open interest to query.
    pub symbol: Cow<'static, str>,
}

/// Response containing the current open interest data for a specific symbol.
#[derive(Debug, Clone, Deserialize)]
pub struct OpenInterest {
    /// The open interest value as a string with decimal precision (e.g., "10659.509").
    #[serde(rename = "openInterest")]
    pub open_interest: String,

    /// The trading pair symbol that this open interest data corresponds to.
    pub symbol: Cow<'static, str>,

    /// Transaction timestamp in milliseconds since Unix epoch when this data was recorded.
    pub time: u64,
}

impl RestClient {
    /// Open Interest
    ///
    /// Get present open interest of a specific symbol.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The open interest request parameters containing the symbol to query
    ///
    /// # Returns
    /// Open interest data including the current value, symbol, and timestamp
    pub async fn get_open_interest(&self, params: OpenInterestRequest) -> RestResult<OpenInterest> {
        let query = format!("symbol={}", params.symbol);
        self.send_public_request(
            OPEN_INTEREST_ENDPOINT,
            reqwest::Method::GET,
            Some(&query),
            1,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_interest_request_default() {
        let request = OpenInterestRequest::default();
        assert_eq!(request.symbol, "");
    }

    #[test]
    fn test_open_interest_request_serialization() {
        let request = OpenInterestRequest {
            symbol: "BTCUSDT".into(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_open_interest_response_deserialization() {
        let json = r#"{
            "openInterest": "10000.00000000",
            "symbol": "BTCUSDT",
            "time": 1625097600000
        }"#;

        let open_interest: OpenInterest = serde_json::from_str(json).unwrap();
        assert_eq!(open_interest.open_interest, "10000.00000000");
        assert_eq!(open_interest.symbol, "BTCUSDT");
        assert_eq!(open_interest.time, 1625097600000);
    }

    #[test]
    fn test_open_interest_large_values() {
        let json = r#"{
            "openInterest": "999999999.99999999",
            "symbol": "ETHUSDT",
            "time": 1625184000000
        }"#;

        let open_interest: OpenInterest = serde_json::from_str(json).unwrap();
        assert_eq!(open_interest.open_interest, "999999999.99999999");
        assert_eq!(open_interest.symbol, "ETHUSDT");
        assert_eq!(open_interest.time, 1625184000000);
    }

    #[test]
    fn test_open_interest_zero_value() {
        let json = r#"{
            "openInterest": "0.00000000",
            "symbol": "DOGEUSDT",
            "time": 1625097600000
        }"#;

        let open_interest: OpenInterest = serde_json::from_str(json).unwrap();
        assert_eq!(open_interest.open_interest, "0.00000000");
        assert_eq!(open_interest.symbol, "DOGEUSDT");
    }

    #[test]
    fn test_open_interest_cow_str() {
        // Test with static string
        let request = OpenInterestRequest {
            symbol: "BTCUSDT".into(),
        };
        assert_eq!(request.symbol, "BTCUSDT");

        // Test with owned string
        let owned_symbol = String::from("ETHUSDT");
        let request2 = OpenInterestRequest {
            symbol: owned_symbol.into(),
        };
        assert_eq!(request2.symbol, "ETHUSDT");
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(OPEN_INTEREST_ENDPOINT, "/fapi/v1/openInterest");
    }
}
