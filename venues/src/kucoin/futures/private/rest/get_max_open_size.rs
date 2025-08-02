use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Get max open size request
#[derive(Debug, Clone, Serialize)]
pub struct GetMaxOpenSizeRequest {
    /// Contract symbol
    pub symbol: String,
    /// Order price
    pub price: String,
    /// Leverage value
    pub leverage: i32,
}

/// Max open size response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxOpenSizeResponse {
    /// Contract symbol
    pub symbol: String,
    /// Maximum buy open size
    pub max_buy_open_size: i64,
    /// Maximum sell open size
    pub max_sell_open_size: i64,
}

impl super::RestClient {
    /// Get maximum open position size
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-max-open-size>
    pub async fn get_max_open_size(
        &self,
        request: GetMaxOpenSizeRequest,
    ) -> Result<(RestResponse<GetMaxOpenSizeResponse>, ResponseHeaders)> {
        const GET_MAX_OPEN_SIZE_ENDPOINT: &str = "/api/v2/getMaxOpenSize";
        self.get(GET_MAX_OPEN_SIZE_ENDPOINT, Some(&request)).await
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
}
