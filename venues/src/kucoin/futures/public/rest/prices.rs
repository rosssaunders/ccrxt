use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

// API endpoints
const MARK_PRICE_ENDPOINT_PREFIX: &str = "/api/v1/mark-price/";
const MARK_PRICE_ENDPOINT_SUFFIX: &str = "/current";
const SPOT_INDEX_PRICE_ENDPOINT: &str = "/api/v1/index/query";

/// Get mark price request
#[derive(Debug, Clone, Serialize)]
pub struct GetMarkPriceRequest {
    pub symbol: String,
}

/// Mark price response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    /// Symbol of the contract
    pub symbol: String,
    /// Granularity (milliseconds)
    pub granularity: i64,
    /// Time point (milliseconds)
    pub time_point: i64,
    /// Mark price
    pub value: f64,
    /// Index price
    pub index_price: f64,
}

/// Get spot index price request
#[derive(Debug, Clone, Serialize)]
pub struct GetSpotIndexPriceRequest {
    /// Index symbols (e.g., .KXBTUSDT, .BXBT)
    pub symbol: String,
}

/// Spot index price item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotIndexPriceItem {
    /// Index symbol
    pub symbol: String,
    /// Granularity (milliseconds)
    pub granularity: i64,
    /// Time point (milliseconds)
    pub time_point: i64,
    /// Index price value
    pub value: f64,
    /// Decomposition list (exchange prices that make up the index)
    #[serde(default)]
    pub decomposition_list: Vec<DecompositionItem>,
}

/// Decomposition item for index price
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecompositionItem {
    /// Exchange name
    pub exchange: String,
    /// Price on this exchange
    pub price: f64,
    /// Weight in the index
    pub weight: f64,
}

/// Response for getting spot index prices
pub type GetSpotIndexPriceResponse = Vec<SpotIndexPriceItem>;

impl super::RestClient {
    /// Get current mark price for a symbol
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-mark-price>
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

    /// Get spot index price for one or more index symbols
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-spot-index-price>
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

    #[test]
    fn test_get_spot_index_price_request() {
        let request = GetSpotIndexPriceRequest {
            symbol: ".KXBTUSDT,.BXBT".to_string(),
        };
        assert_eq!(request.symbol, ".KXBTUSDT,.BXBT");
    }
}
