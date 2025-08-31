use serde::{Deserialize, Serialize};

use crate::kucoin::futures::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

/// Endpoint URL for getting maximum open size
const GET_MAX_OPEN_SIZE_ENDPOINT: &str = "/api/v2/getMaxOpenSize";

/// Request parameters for getting maximum open position size.
#[derive(Debug, Clone, Serialize)]
pub struct GetMaxOpenSizeRequest {
    /// Contract symbol (e.g., "XBTUSDTM"). Required parameter.
    pub symbol: String,

    /// Order price (e.g., "50000"). Required parameter.
    pub price: String,

    /// Leverage value for the position (e.g., 10). Required parameter.
    pub leverage: i32,
}

/// Response containing maximum open size information.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxOpenSizeResponse {
    /// Contract symbol (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Maximum buy open size allowed for the position.
    pub max_buy_open_size: i64,

    /// Maximum sell open size allowed for the position.
    pub max_sell_open_size: i64,
}

impl RestClient {
    /// Get Maximum Open Size
    ///
    /// Retrieve the maximum open position size for a given symbol at a specified price and leverage.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-max-open-size)
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The maximum open size request parameters
    ///
    /// # Returns
    /// Maximum open size information for both buy and sell positions
    pub async fn get_max_open_size(
        &self,
        request: GetMaxOpenSizeRequest,
    ) -> Result<(RestResponse<GetMaxOpenSizeResponse>, ResponseHeaders)> {
        self.get_with_request(GET_MAX_OPEN_SIZE_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_open_size_request_creation() {
        let request = GetMaxOpenSizeRequest {
            symbol: "XBTUSDTM".to_string(),
            price: "50000".to_string(),
            leverage: 10,
        };
        assert_eq!(request.symbol, "XBTUSDTM");
        assert_eq!(request.price, "50000");
        assert_eq!(request.leverage, 10);
    }

    #[test]
    fn test_get_max_open_size_response_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "maxBuyOpenSize": 100,
            "maxSellOpenSize": 150
        }"#;

        let response: GetMaxOpenSizeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "XBTUSDTM");
        assert_eq!(response.max_buy_open_size, 100);
        assert_eq!(response.max_sell_open_size, 150);
    }

    #[test]
    fn test_request_serialization() {
        let request = GetMaxOpenSizeRequest {
            symbol: "ETHUSDTM".to_string(),
            price: "3500".to_string(),
            leverage: 5,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDTM");
        assert_eq!(json["price"], "3500");
        assert_eq!(json["leverage"], 5);
    }

    #[test]
    fn test_various_symbols() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let request = GetMaxOpenSizeRequest {
                symbol: symbol.to_string(),
                price: "10000".to_string(),
                leverage: 1,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
        }
    }

    #[test]
    fn test_leverage_variations() {
        let leverages = [1, 3, 5, 10, 20, 50, 100];

        for leverage in leverages.iter() {
            let request = GetMaxOpenSizeRequest {
                symbol: "XBTUSDTM".to_string(),
                price: "50000".to_string(),
                leverage: *leverage,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["leverage"], *leverage);
        }
    }

    #[test]
    fn test_price_variations() {
        let prices = ["100", "1000", "10000", "50000", "100000"];

        for price in prices.iter() {
            let request = GetMaxOpenSizeRequest {
                symbol: "XBTUSDTM".to_string(),
                price: price.to_string(),
                leverage: 10,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["price"], *price);
        }
    }

    #[test]
    fn test_response_field_types() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "maxBuyOpenSize": 500,
            "maxSellOpenSize": 300
        }"#;

        let response: GetMaxOpenSizeResponse = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        assert!(serialized["symbol"].is_string());
        assert!(serialized["maxBuyOpenSize"].is_number());
        assert!(serialized["maxSellOpenSize"].is_number());
    }

    #[test]
    fn test_large_open_size_values() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "maxBuyOpenSize": 999999999,
            "maxSellOpenSize": 888888888
        }"#;

        let response: GetMaxOpenSizeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.max_buy_open_size, 999999999);
        assert_eq!(response.max_sell_open_size, 888888888);
    }

    #[test]
    fn test_zero_open_size_values() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "maxBuyOpenSize": 0,
            "maxSellOpenSize": 0
        }"#;

        let response: GetMaxOpenSizeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.max_buy_open_size, 0);
        assert_eq!(response.max_sell_open_size, 0);
    }

    #[test]
    fn test_camel_case_conversion() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "maxBuyOpenSize": 100,
            "maxSellOpenSize": 150
        }"#;

        let response: GetMaxOpenSizeResponse = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&response).unwrap();

        // Verify camelCase fields exist
        assert!(serialized.get("maxBuyOpenSize").is_some());
        assert!(serialized.get("maxSellOpenSize").is_some());
        // Verify snake_case fields do not exist
        assert!(serialized.get("max_buy_open_size").is_none());
        assert!(serialized.get("max_sell_open_size").is_none());
    }

    #[test]
    fn test_request_all_field_types() {
        let request = GetMaxOpenSizeRequest {
            symbol: "XBTUSDTM".to_string(),
            price: "50000".to_string(),
            leverage: 10,
        };

        let json = serde_json::to_value(&request).unwrap();

        assert!(json["symbol"].is_string());
        assert!(json["price"].is_string());
        assert!(json["leverage"].is_number());
    }

    #[test]
    fn test_asymmetric_open_sizes() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "maxBuyOpenSize": 1000,
            "maxSellOpenSize": 500
        }"#;

        let response: GetMaxOpenSizeResponse = serde_json::from_str(json).unwrap();
        assert_ne!(response.max_buy_open_size, response.max_sell_open_size);
        assert!(response.max_buy_open_size > response.max_sell_open_size);
    }

    #[test]
    fn test_decimal_price_handling() {
        let request = GetMaxOpenSizeRequest {
            symbol: "XBTUSDTM".to_string(),
            price: "50000.5".to_string(),
            leverage: 10,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["price"], "50000.5");
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(GET_MAX_OPEN_SIZE_ENDPOINT, "/api/v2/getMaxOpenSize");
    }
}
