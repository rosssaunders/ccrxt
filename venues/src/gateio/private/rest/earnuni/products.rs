use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const PRODUCTS_ENDPOINT: &str = "/earn/uni/products";

/// Query parameters for listing EarnUni products.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ProductsQuery {
    /// Currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Pagination offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Maximum number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Product information returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInfo {
    /// Product identifier.
    pub id: String,

    /// Underlying currency for the product.
    pub currency: String,

    /// Annual percentage yield as string.
    pub apy: String,

    /// Minimum investable amount.
    pub min_amount: String,
}

impl RestClient {
    /// List EarnUni products.
    pub async fn list_earnuni_products(
        &self,
        query: ProductsQuery,
    ) -> RestResult<Vec<ProductInfo>> {
        // verb-specific function (GET)
        self.send_get_request(PRODUCTS_ENDPOINT, Some(&query)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn products_query_serializes_to_query_string() {
        let q = ProductsQuery {
            currency: Some("BTC".to_string()),
            offset: Some(10),
            limit: Some(50),
        };

        let qs = serde_urlencoded::to_string(&q).expect("serialize query");
        // order of params is not guaranteed, just assert substrings
        assert!(qs.contains("currency=BTC"));
        assert!(qs.contains("offset=10"));
        assert!(qs.contains("limit=50"));
    }

    #[test]
    fn product_info_deserializes() {
        let json = r#"{"id":"p1","currency":"BTC","apy":"0.01","min_amount":"0.1"}"#;
        let p: ProductInfo = serde_json::from_str(json).expect("deserialize");
        assert_eq!(p.id, "p1");
        assert_eq!(p.currency, "BTC");
        assert_eq!(p.apy, "0.01");
        assert_eq!(p.min_amount, "0.1");
    }
}
