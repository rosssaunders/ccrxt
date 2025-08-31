use serde::{Deserialize, Serialize};

use crate::coinbaseexchange::{PublicRestClient, RestResult};

const PRODUCTS_ENDPOINT: &str = "products";

/// Request to get all products
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductsRequest {
    /// Type filter for products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Product information
#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    /// Product ID (e.g., "BTC-USD")
    pub id: String,

    /// Base currency symbol
    pub base_currency: String,

    /// Quote currency symbol
    pub quote_currency: String,

    /// Min order price (a.k.a. price increment)
    pub quote_increment: String,

    /// Base currency increment
    pub base_increment: String,

    /// Display name for the product
    pub display_name: String,

    /// Minimum market funds required
    pub min_market_funds: String,

    /// Whether margin trading is enabled
    #[serde(default)]
    pub margin_enabled: bool,

    /// Whether only post-only orders are allowed
    #[serde(default)]
    pub post_only: bool,

    /// Whether only limit orders are allowed
    #[serde(default)]
    pub limit_only: bool,

    /// Whether only cancel orders are allowed
    #[serde(default)]
    pub cancel_only: bool,

    /// Product status
    pub status: String,

    /// Additional status message
    #[serde(default)]
    pub status_message: String,

    /// Whether trading is currently disabled
    #[serde(default)]
    pub trading_disabled: bool,

    /// Whether this is an FX stablecoin pair
    #[serde(default)]
    pub fx_stablecoin: bool,

    /// Maximum slippage percentage
    #[serde(default)]
    pub max_slippage_percentage: String,

    /// Whether the book is in auction mode
    #[serde(default)]
    pub auction_mode: bool,

    /// Percentage to calculate highest price for limit buy order (Stable coin trading pair only)
    #[serde(default)]
    pub high_bid_limit_percentage: String,
}

/// Response from getting all products
pub type GetProductsResponse = Vec<Product>;

impl PublicRestClient {
    /// Get all known trading pairs
    ///
    /// Gets a list of available currency pairs for trading.
    ///
    /// [docs](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getproducts)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The products request parameters
    ///
    /// # Returns
    /// A result containing the list of products or an error
    pub async fn get_products(
        &self,
        request: &GetProductsRequest,
    ) -> RestResult<GetProductsResponse> {
        self.send_get_request(PRODUCTS_ENDPOINT, Some(request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_products_request_serialization() {
        let request = GetProductsRequest {
            r#type: Some("spot".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=spot"));
    }

    #[test]
    fn test_get_products_request_default() {
        let request = GetProductsRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.is_empty());
    }

    #[test]
    fn test_product_deserialization() {
        let json = r#"{
            "id": "BTC-USD",
            "base_currency": "BTC",
            "quote_currency": "USD",
            "quote_increment": "0.01",
            "base_increment": "0.00000001",
            "display_name": "BTC/USD",
            "min_market_funds": "10.00",
            "margin_enabled": false,
            "post_only": false,
            "limit_only": false,
            "cancel_only": false,
            "status": "online",
            "status_message": "",
            "trading_disabled": false,
            "fx_stablecoin": false,
            "max_slippage_percentage": "0.02",
            "auction_mode": false,
            "high_bid_limit_percentage": "0.1"
        }"#;

        let product: Product = serde_json::from_str(json).unwrap();
        assert_eq!(product.id, "BTC-USD");
        assert_eq!(product.base_currency, "BTC");
        assert_eq!(product.quote_currency, "USD");
        assert_eq!(product.status, "online");
        assert!(!product.trading_disabled);
    }

    #[test]
    fn test_get_products_response_deserialization() {
        let json = r#"[{
            "id": "BTC-USD",
            "base_currency": "BTC",
            "quote_currency": "USD",
            "quote_increment": "0.01",
            "base_increment": "0.00000001",
            "display_name": "BTC/USD",
            "min_market_funds": "10.00",
            "margin_enabled": false,
            "post_only": false,
            "limit_only": false,
            "cancel_only": false,
            "status": "online",
            "status_message": "",
            "trading_disabled": false,
            "fx_stablecoin": false,
            "max_slippage_percentage": "0.02",
            "auction_mode": false,
            "high_bid_limit_percentage": "0.1"
        }]"#;

        let products: GetProductsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id, "BTC-USD");
    }
}
