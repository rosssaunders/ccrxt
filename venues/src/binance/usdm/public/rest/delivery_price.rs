use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint constant for Binance Delivery Price.
const DELIVERY_PRICE_ENDPOINT: &str = "/futures/data/delivery-price";

/// Request parameters for the Quarterly Contract Settlement Price endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceRequest<'a> {
    /// Contract pair, e.g., "BTCUSDT".
    pub pair: Cow<'a, str>,
}

/// Response structure for the Quarterly Contract Settlement Price.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceResponse {
    /// Delivery time (milliseconds since epoch).
    pub delivery_time: u64,

    /// Settlement price at the delivery time.
    pub delivery_price: f64,
}

impl RestClient {
    /// Quarterly Contract Settlement Price (GET /futures/data/delivery-price)
    ///
    /// Latest settlement price for the specified quarterly contract pair.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Delivery-Price
    ///
    /// Rate limit: 0
    ///
    /// # Arguments
    ///
    /// * `params` - Request parameters including the contract pair.
    ///
    /// # Returns
    ///
    /// A vector of `DeliveryPriceResponse` entries with delivery times and prices.
    pub async fn delivery_price(
        &self,
        params: DeliveryPriceRequest<'_>,
    ) -> RestResult<Vec<DeliveryPriceResponse>> {
        self.send_get_request(DELIVERY_PRICE_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_price_request_serialization() {
        let request = DeliveryPriceRequest {
            pair: "BTCUSDT".into(),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, r#"{"pair":"BTCUSDT"}"#);
    }

    #[test]
    fn test_delivery_price_response_deserialization() {
        let json = r#"[
            {
                "deliveryTime": 1625097600000,
                "deliveryPrice": 45000.50
            },
            {
                "deliveryTime": 1625184000000,
                "deliveryPrice": 46250.75
            }
        ]"#;

        let response: Vec<DeliveryPriceResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        assert_eq!(response[0].delivery_time, 1625097600000);
        assert_eq!(response[0].delivery_price, 45000.50);

        assert_eq!(response[1].delivery_time, 1625184000000);
        assert_eq!(response[1].delivery_price, 46250.75);
    }

    #[test]
    fn test_delivery_price_high_price() {
        let json = r#"[
            {
                "deliveryTime": 1625097600000,
                "deliveryPrice": 999999.99
            }
        ]"#;

        let response: Vec<DeliveryPriceResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].delivery_price, 999999.99);
    }

    #[test]
    fn test_delivery_price_low_price() {
        let json = r#"[
            {
                "deliveryTime": 1625097600000,
                "deliveryPrice": 0.01
            }
        ]"#;

        let response: Vec<DeliveryPriceResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].delivery_price, 0.01);
    }

    #[test]
    fn test_delivery_price_empty_response() {
        let json = r#"[]"#;
        let response: Vec<DeliveryPriceResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }

    #[test]
    fn test_delivery_price_fractional_price() {
        let json = r#"[
            {
                "deliveryTime": 1625097600000,
                "deliveryPrice": 12345.6789
            }
        ]"#;

        let response: Vec<DeliveryPriceResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].delivery_price, 12345.6789);
    }

    #[test]
    fn test_delivery_price_multiple_deliveries() {
        let json = r#"[
            {
                "deliveryTime": 1617235200000,
                "deliveryPrice": 58000.00
            },
            {
                "deliveryTime": 1619827200000,
                "deliveryPrice": 55000.00
            },
            {
                "deliveryTime": 1622505600000,
                "deliveryPrice": 35000.00
            },
            {
                "deliveryTime": 1625097600000,
                "deliveryPrice": 40000.00
            }
        ]"#;

        let response: Vec<DeliveryPriceResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 4);
        // Verify chronological order
        assert!(response[0].delivery_time < response[1].delivery_time);
        assert!(response[1].delivery_time < response[2].delivery_time);
        assert!(response[2].delivery_time < response[3].delivery_time);
    }
}
