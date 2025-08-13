use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const SPOT_INDEX_PRICE_ENDPOINT: &str = "/api/v1/index/query";

/// Request parameters for getting spot index price.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSpotIndexPriceRequest {
    /// Index symbols (e.g., ".KXBTUSDT", ".BXBT"). Multiple symbols can be comma-separated.
    pub symbol: String,
}

/// Spot index price item data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotIndexPriceItem {
    /// Index symbol.
    pub symbol: String,

    /// Granularity (milliseconds).
    pub granularity: i64,

    /// Time point (milliseconds since epoch).
    pub time_point: i64,

    /// Index price value.
    pub value: f64,

    /// Decomposition list (exchange prices that make up the index).
    #[serde(default)]
    pub decomposition_list: Vec<DecompositionItem>,
}

/// Decomposition item showing how index price is calculated from individual exchanges.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecompositionItem {
    /// Exchange name.
    pub exchange: String,

    /// Price on this exchange.
    pub price: f64,

    /// Weight of this exchange in the index calculation.
    pub weight: f64,
}

/// Response for getting spot index prices.
pub type GetSpotIndexPriceResponse = Vec<SpotIndexPriceItem>;

impl super::RestClient {
    /// Get spot index price for one or more index symbols
    ///
    /// Returns the spot index price data for the specified index symbols, including
    /// the decomposition showing how the price is calculated from multiple exchanges.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-spot-index-price)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The spot index price request parameters containing the symbol(s)
    ///
    /// # Returns
    /// List of spot index price items wrapped in RestResponse with response headers
    pub async fn get_spot_index_price(
        &self,
        request: GetSpotIndexPriceRequest,
    ) -> Result<(RestResponse<GetSpotIndexPriceResponse>, ResponseHeaders)> {
        self.send_request(SPOT_INDEX_PRICE_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_spot_index_price_request() {
        let request = GetSpotIndexPriceRequest {
            symbol: ".KXBTUSDT,.BXBT".to_string(),
        };
        assert_eq!(request.symbol, ".KXBTUSDT,.BXBT");
    }

    #[test]
    fn test_spot_index_price_deserialization() {
        let json = r#"{
            "symbol": ".KXBTUSDT",
            "granularity": 1000,
            "timePoint": 1634567890000,
            "value": 49999.8,
            "decompositionList": [
                {
                    "exchange": "binance",
                    "price": 50000.0,
                    "weight": 0.5
                },
                {
                    "exchange": "okx",
                    "price": 49999.6,
                    "weight": 0.5
                }
            ]
        }"#;

        let price: SpotIndexPriceItem = serde_json::from_str(json).unwrap();
        assert_eq!(price.symbol, ".KXBTUSDT");
        assert_eq!(price.value, 49999.8);
        assert_eq!(price.decomposition_list.len(), 2);
        assert_eq!(price.decomposition_list[0].exchange, "binance");
        assert_eq!(price.decomposition_list[0].price, 50000.0);
        assert_eq!(price.decomposition_list[0].weight, 0.5);
    }

    #[test]
    fn test_spot_index_price_response_deserialization() {
        let json = r#"[
            {
                "symbol": ".KXBTUSDT",
                "granularity": 1000,
                "timePoint": 1634567890000,
                "value": 49999.8,
                "decompositionList": []
            },
            {
                "symbol": ".BXBT",
                "granularity": 1000,
                "timePoint": 1634567890000,
                "value": 50100.2,
                "decompositionList": []
            }
        ]"#;

        let prices: GetSpotIndexPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(prices.len(), 2);
        assert_eq!(prices[0].symbol, ".KXBTUSDT");
        assert_eq!(prices[1].symbol, ".BXBT");
    }
}
