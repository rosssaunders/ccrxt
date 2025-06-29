//! Index prices endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

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

impl RestClient {
    /// Get all index prices
    ///
    /// Retrieves the index prices for all supported assets
    ///
    /// # Returns
    /// A `RestResult<Vec<IndexPrice>>` containing all index prices
    ///
    /// # Errors
    /// Returns an error if the request fails or the response cannot be parsed
    ///
    /// # Example
    /// ```no_run
    /// # use venues::bullish::public::rest::RestClient;
    /// # async fn example(client: RestClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let index_prices = client.get_index_prices().await?;
    /// for price in index_prices {
    ///     println!("{}: ${}", price.asset_symbol, price.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index_prices(&self) -> RestResult<Vec<IndexPrice>> {
        self.send_request::<Vec<IndexPrice>, ()>(
            "/trading-api/v1/index-prices",
            reqwest::Method::GET,
            None,
            EndpointType::PublicOther,
        )
        .await
    }

    /// Get index price for a specific asset symbol
    ///
    /// Retrieves the index price of a specified asset
    ///
    /// # Arguments
    /// * `asset_symbol` - The asset symbol to get the index price for
    ///
    /// # Returns
    /// A `RestResult<IndexPrice>` containing the index price for the asset
    ///
    /// # Errors
    /// Returns an error if the request fails, the asset is not found, or the response cannot be parsed
    ///
    /// # Example
    /// ```no_run
    /// # use venues::bullish::public::rest::RestClient;
    /// # async fn example(client: RestClient) -> Result<(), Box<dyn std::error::Error>> {
    /// let btc_price = client.get_index_price_by_symbol("BTC").await?;
    /// println!("BTC index price: ${}", btc_price.price);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_index_price_by_symbol(&self, asset_symbol: &str) -> RestResult<IndexPrice> {
        let endpoint = format!("/trading-api/v1/index-prices/{}", asset_symbol);
        self.send_request::<IndexPrice, ()>(
            &endpoint,
            reqwest::Method::GET,
            None,
            EndpointType::PublicOther,
        )
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
