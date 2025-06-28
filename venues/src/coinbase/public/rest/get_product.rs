//! Get single product endpoint for Coinbase Exchange REST API
//!
//! Get information on a single product.

use serde::{Deserialize, Serialize};

use crate::coinbase::RestResult;

use super::{Product, RestClient};

/// Request to get a single product
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductRequest {}

/// Response from getting a single product
pub type GetProductResponse = Product;

impl RestClient {
    /// Get single product
    ///
    /// Get information on a single product.
    ///
    /// # Arguments
    /// * `product_id` - The product ID (e.g., "BTC-USD")
    /// * `request` - The product request parameters
    ///
    /// # Returns
    /// A result containing the product information or an error
    ///
    /// # Example
    /// ```no_run
    /// use ccrxt::venues::coinbase::public::rest::{GetProductRequest, RestClient};
    /// use reqwest::Client;
    /// use ccrxt::venues::coinbase::RateLimiter;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RestClient::new(
    ///     "https://api.exchange.coinbase.com",
    ///     Client::new(),
    ///     RateLimiter::new(),
    /// );
    ///
    /// let request = GetProductRequest::default();
    /// let product = client.get_product("BTC-USD", &request).await?;
    /// println!("Product: {} - {}", product.id, product.display_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_product(
        &self,
        product_id: &str,
        request: &GetProductRequest,
    ) -> RestResult<GetProductResponse> {
        let endpoint = format!("products/{}", product_id);
        self.send_request(&endpoint, reqwest::Method::GET, Some(request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_request_serialization() {
        let request = GetProductRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.is_empty());
    }
}
