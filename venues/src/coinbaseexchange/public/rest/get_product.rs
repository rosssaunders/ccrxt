use serde::Serialize;

use super::{Product, RestClient};
use crate::coinbaseexchange::RestResult;

/// Endpoint URL path for getting a single product
const ENDPOINT_PATH: &str = "products/{}";

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
    /// [docs](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getproduct)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `product_id` - The product ID (e.g., "BTC-USD")
    /// * `request` - The product request parameters
    ///
    /// # Returns
    /// A result containing the product information or an error
    pub async fn get_product(
        &self,
        product_id: &str,
        request: &GetProductRequest,
    ) -> RestResult<GetProductResponse> {
        let endpoint = ENDPOINT_PATH.replace("{}", product_id);
        self.send_get_request(&endpoint, Some(request)).await
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
