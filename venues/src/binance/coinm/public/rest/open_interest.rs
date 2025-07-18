use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Parameters for Open Interest
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenInterestRequest {
    /// Symbol name
    pub symbol: String,
}

/// Open interest data
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    /// Open interest
    pub open_interest: Decimal,
    /// Symbol name
    pub symbol: String,
    /// Timestamp
    pub time: i64,
}

impl RestClient {
    /// Get Open Interest
    ///
    /// Weight: 1
    pub async fn get_open_interest(&self, params: OpenInterestRequest) -> RestResult<OpenInterest> {
        self.send_request(
            "/dapi/v1/openInterest",
            reqwest::Method::GET,
            Some(params),
            1,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

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
            "openInterest": "1234567.8901",
            "symbol": "BTCUSD_PERP",
            "time": 1699948800000
        }"#;

        let response: OpenInterest = serde_json::from_str(json).unwrap();
        assert_eq!(response.open_interest, Decimal::new(12345678901, 4));
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.time, 1699948800000);
    }

    #[test]
    fn test_open_interest_response_with_zero_interest() {
        let json = r#"{
            "openInterest": "0.0000",
            "symbol": "ETHUSD_240329",
            "time": 1625097600000
        }"#;

        let response: OpenInterest = serde_json::from_str(json).unwrap();
        assert_eq!(response.open_interest, Decimal::ZERO);
        assert_eq!(response.symbol, "ETHUSD_240329");
        assert_eq!(response.time, 1625097600000);
    }
}
