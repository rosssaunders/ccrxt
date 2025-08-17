use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const MARK_PRICE_ENDPOINT_PREFIX: &str = "/api/v1/mark-price/";
const MARK_PRICE_ENDPOINT_SUFFIX: &str = "/current";

/// Request parameters for getting current mark price.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMarkPriceRequest {
    /// Symbol of the contract (e.g., "XBTUSDTM").
    pub symbol: String,
}

/// Mark price response data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    /// Symbol of the contract.
    pub symbol: String,

    /// Granularity (milliseconds).
    pub granularity: i64,

    /// Time point (milliseconds since epoch).
    pub time_point: i64,

    /// Current mark price.
    pub value: f64,

    /// Current index price.
    pub index_price: f64,
}

impl super::RestClient {
    /// Get current mark price for a symbol
    ///
    /// Returns the current mark price for the specified futures contract symbol.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-mark-price)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The mark price request parameters containing the symbol
    ///
    /// # Returns
    /// Mark price data wrapped in RestResponse with response headers
    pub async fn get_mark_price(
        &self,
        request: GetMarkPriceRequest,
    ) -> Result<(RestResponse<MarkPrice>, ResponseHeaders)> {
        let endpoint = format!(
            "{}{}{}",
            MARK_PRICE_ENDPOINT_PREFIX, request.symbol, MARK_PRICE_ENDPOINT_SUFFIX
        );
        self.send_request(&endpoint, None::<&GetMarkPriceRequest>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mark_price_request() {
        let request = GetMarkPriceRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_mark_price_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "granularity": 1000,
            "timePoint": 1634567890000,
            "value": 50000.5,
            "indexPrice": 49999.8
        }"#;

        let price: MarkPrice = serde_json::from_str(json).unwrap();
        assert_eq!(price.symbol, "XBTUSDTM");
        assert_eq!(price.granularity, 1000);
        assert_eq!(price.time_point, 1634567890000);
        assert_eq!(price.value, 50000.5);
        assert_eq!(price.index_price, 49999.8);
    }
}
