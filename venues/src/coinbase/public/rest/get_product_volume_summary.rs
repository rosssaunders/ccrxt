//! Get product volume summary endpoint for Coinbase Exchange REST API
//!
//! Gets 30day and 24hour volume for all products and market types.

use serde::{Deserialize, Serialize};

use crate::coinbase::RestResult;

use super::RestClient;

/// Request to get product volume summary
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductVolumeSummaryRequest {}

/// Market type information
#[derive(Debug, Clone, Deserialize)]
pub struct MarketType {
    /// Market type name
    #[serde(default)]
    pub name: String,
}

/// Product volume summary information
#[derive(Debug, Clone, Deserialize)]
pub struct ProductVolumeSummary {
    /// Product ID
    pub id: String,

    /// Base currency symbol
    pub base_currency: String,

    /// Quote currency symbol
    pub quote_currency: String,

    /// Display name for the product
    pub display_name: String,

    /// Market types
    #[serde(default)]
    pub market_types: Vec<MarketType>,

    /// 24-hour spot volume
    #[serde(default)]
    pub spot_volume_24hour: String,

    /// 30-day spot volume
    #[serde(default)]
    pub spot_volume_30day: String,

    /// 24-hour RFQ volume
    #[serde(default)]
    pub rfq_volume_24hour: String,

    /// 30-day RFQ volume
    #[serde(default)]
    pub rfq_volume_30day: String,

    /// 24-hour conversion volume
    #[serde(default)]
    pub conversion_volume_24hour: String,

    /// 30-day conversion volume
    #[serde(default)]
    pub conversion_volume_30day: String,
}

/// Response from getting product volume summary
pub type GetProductVolumeSummaryResponse = Vec<ProductVolumeSummary>;

impl RestClient {
    /// Get all product volume summary
    ///
    /// Gets 30day and 24hour volume for all products and market types.
    ///
    /// # Arguments
    /// * `request` - The product volume summary request parameters
    ///
    /// # Returns
    /// A result containing the list of product volume summaries or an error
    ///
    /// # Example
    /// ```no_run
    /// use ccrxt::venues::coinbase::public::rest::{GetProductVolumeSummaryRequest, RestClient};
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
    /// let request = GetProductVolumeSummaryRequest::default();
    /// let volume_summaries = client.get_product_volume_summary(&request).await?;
    /// println!("Found {} product volume summaries", volume_summaries.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_product_volume_summary(
        &self,
        request: &GetProductVolumeSummaryRequest,
    ) -> RestResult<GetProductVolumeSummaryResponse> {
        self.send_request(
            "products/volume-summary",
            reqwest::Method::GET,
            Some(request),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_volume_summary_request_serialization() {
        let request = GetProductVolumeSummaryRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.is_empty());
    }

    #[test]
    fn test_product_volume_summary_deserialization() {
        let json = r#"{
            "id": "BTC-USD",
            "base_currency": "BTC",
            "quote_currency": "USD",
            "display_name": "BTC/USD",
            "market_types": [],
            "spot_volume_24hour": "1000000.00",
            "spot_volume_30day": "30000000.00",
            "rfq_volume_24hour": "50000.00",
            "rfq_volume_30day": "1500000.00",
            "conversion_volume_24hour": "10000.00",
            "conversion_volume_30day": "300000.00"
        }"#;

        let volume_summary: ProductVolumeSummary = serde_json::from_str(json).unwrap();
        assert_eq!(volume_summary.id, "BTC-USD");
        assert_eq!(volume_summary.base_currency, "BTC");
        assert_eq!(volume_summary.quote_currency, "USD");
        assert_eq!(volume_summary.spot_volume_24hour, "1000000.00");
        assert_eq!(volume_summary.spot_volume_30day, "30000000.00");
    }

    #[test]
    fn test_get_product_volume_summary_response_deserialization() {
        let json = r#"[{
            "id": "BTC-USD",
            "base_currency": "BTC",
            "quote_currency": "USD",
            "display_name": "BTC/USD",
            "market_types": [],
            "spot_volume_24hour": "1000000.00",
            "spot_volume_30day": "30000000.00",
            "rfq_volume_24hour": "50000.00",
            "rfq_volume_30day": "1500000.00",
            "conversion_volume_24hour": "10000.00",
            "conversion_volume_30day": "300000.00"
        }]"#;

        let volume_summaries: GetProductVolumeSummaryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(volume_summaries.len(), 1);
        assert_eq!(volume_summaries[0].id, "BTC-USD");
    }
}
