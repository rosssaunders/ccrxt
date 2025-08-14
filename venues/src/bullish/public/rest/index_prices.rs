//! Index prices endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for index prices
const INDEX_PRICES_ENDPOINT: &str = "/trading-api/v1/index-prices";

/// Endpoint URL path for single index price (with parameter)
const SINGLE_INDEX_PRICE_ENDPOINT: &str = "/trading-api/v1/index-prices/{}";

/// Index price information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPrice {
    /// Asset symbol
    pub asset_symbol: String,

    /// Asset price in USD
    pub price: String,

    /// Date and time when the index price was updated
    pub updated_at_datetime: String,

    /// Timestamp when the index price was updated
    pub updated_at_timestamp: String,
}

/// Request parameters for getting a specific index price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIndexPriceRequest {
    /// The asset symbol to retrieve
    pub asset_symbol: String,
}

impl RestClient {
    /// Get all index prices
    ///
    /// Retrieves the index prices for all supported assets
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/index-prices)
    ///
    /// # Returns
    /// A `RestResult<Vec<IndexPrice>>` containing all index prices
    ///
    /// # Errors
    /// Returns an error if the request fails or the response cannot be parsed
    pub async fn get_index_prices(&self) -> RestResult<Vec<IndexPrice>> {
        self.send_get_request(INDEX_PRICES_ENDPOINT, EndpointType::PublicOther)
            .await
    }

    /// Get index price for a specific asset symbol
    ///
    /// Retrieves the index price of a specified asset
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/index-prices)
    ///
    /// # Arguments
    /// * `request` - Request parameters containing the asset symbol
    ///
    /// # Returns
    /// A `RestResult<IndexPrice>` containing the index price for the asset
    ///
    /// # Errors
    /// Returns an error if the request fails, the asset is not found, or the response cannot be parsed
    pub async fn get_index_price_by_symbol(
        &self,
        request: &GetIndexPriceRequest,
    ) -> RestResult<IndexPrice> {
        let endpoint = SINGLE_INDEX_PRICE_ENDPOINT.replace("{}", &request.asset_symbol);
        self.send_get_request(&endpoint, EndpointType::PublicOther)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_price_deserialization() {
        let json = r#"
        {
            "assetSymbol": "BTC",
            "price": "66100.0000",
            "updatedAtDatetime": "2025-05-20T01:01:01.000Z",
            "updatedAtTimestamp": "1621490985000"
        }
        "#;

        let index_price: IndexPrice = serde_json::from_str(json).unwrap();
        assert_eq!(index_price.asset_symbol, "BTC");
        assert_eq!(index_price.price, "66100.0000");
        assert_eq!(index_price.updated_at_datetime, "2025-05-20T01:01:01.000Z");
        assert_eq!(index_price.updated_at_timestamp, "1621490985000");
    }

    #[test]
    fn test_index_prices_array_deserialization() {
        let json = r#"
        [
            {
                "assetSymbol": "BTC",
                "price": "66100.0000",
                "updatedAtDatetime": "2025-05-20T01:01:01.000Z",
                "updatedAtTimestamp": "1621490985000"
            },
            {
                "assetSymbol": "ETH",
                "price": "3500.0000",
                "updatedAtDatetime": "2025-05-20T01:01:01.000Z",
                "updatedAtTimestamp": "1621490985000"
            }
        ]
        "#;

        let index_prices: Vec<IndexPrice> = serde_json::from_str(json).unwrap();
        assert_eq!(index_prices.len(), 2);
        assert_eq!(index_prices[0].asset_symbol, "BTC");
        assert_eq!(index_prices[1].asset_symbol, "ETH");
    }
}
