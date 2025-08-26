use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::PublicRestClient as RestClient;
use crate::binance::spot::RestResult;

const AVG_PRICE_ENDPOINT: &str = "/api/v3/avgPrice";

/// Request parameters for average price
#[derive(Debug, Clone, Serialize)]
pub struct AvgPriceRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
}

/// Average price response
#[derive(Debug, Clone, Deserialize)]
pub struct AvgPriceResponse {
    /// Minutes for which the average price is calculated
    #[serde(rename = "mins")]
    pub mins: u32,

    /// Average price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Close time of the interval
    #[serde(rename = "closeTime")]
    pub close_time: u64,
}

impl RestClient {
    /// Get current average price
    ///
    /// Current average price for a symbol.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#current-average-price)
    ///
    /// Method: GET /api/v3/avgPrice
    /// Weight: 2
    /// Security: None
    pub async fn get_avg_price(&self, params: AvgPriceRequest) -> RestResult<AvgPriceResponse> {
        self.send_get_request(AVG_PRICE_ENDPOINT, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::FromPrimitive;

    use super::*;

    #[test]
    fn test_avg_price_request_serialization() {
        let request = AvgPriceRequest {
            symbol: "BTCUSDT".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_avg_price_response_deserialization() {
        let json = r#"{
            "mins": 5,
            "price": "50000.50",
            "closeTime": 1625097600000
        }"#;

        let response: AvgPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.mins, 5);
        assert_eq!(response.price, Decimal::from_f64(50000.50).unwrap());
        assert_eq!(response.close_time, 1625097600000);
    }

    #[test]
    fn test_avg_price_response_deserialization_different_duration() {
        let json = r#"{
            "mins": 15,
            "price": "3200.75",
            "closeTime": 1625184000000
        }"#;

        let response: AvgPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.mins, 15);
        assert_eq!(response.price, Decimal::from_f64(3200.75).unwrap());
        assert_eq!(response.close_time, 1625184000000);
    }
}
