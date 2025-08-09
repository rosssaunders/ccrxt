use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::coinbaseexchange::RestResult;

const PRODUCT_VOLUME_SUMMARY_ENDPOINT: &str = "products/volume-summary";

/// Request to get product volume summary
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductVolumeSummaryRequest {}

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

    /// Market types (array of strings like ["rfq", "spot", "conversions"])
    #[serde(default)]
    pub market_types: Vec<String>,

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
    /// [API Documentation](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getproductvolumesummary)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The product volume summary request parameters
    ///
    /// # Returns
    /// A result containing the list of product volume summaries or an error
    pub async fn get_product_volume_summary(
        &self,
        request: &GetProductVolumeSummaryRequest,
    ) -> RestResult<GetProductVolumeSummaryResponse> {
        self.send_get_request(PRODUCT_VOLUME_SUMMARY_ENDPOINT, Some(request))
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
            "market_types": ["rfq", "spot", "conversions"],
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
        assert_eq!(
            volume_summary.market_types,
            vec!["rfq", "spot", "conversions"]
        );
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
            "market_types": ["spot"],
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
