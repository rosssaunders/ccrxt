use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::coinbaseexchange::RestResult;

/// Endpoint URL path for getting product stats
const ENDPOINT_PATH: &str = "products/{}/stats";

/// Request to get product stats
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductStatsRequest {}

/// Product statistics
#[derive(Debug, Clone, Deserialize)]
pub struct ProductStats {
    /// Opening price
    pub open: String,

    /// Highest price in 24h
    pub high: String,

    /// Lowest price in 24h
    pub low: String,

    /// Last trade price
    pub last: String,

    /// Volume in base currency
    pub volume: String,

    /// 30-day volume
    pub volume_30day: String,

    /// 24-hour RFQ volume
    #[serde(default)]
    pub rfq_volume_24hour: String,

    /// 30-day RFQ volume
    #[serde(default)]
    pub rfq_volume_30day: String,

    /// 24-hour conversions volume
    #[serde(default)]
    pub conversions_volume_24hour: String,

    /// 30-day conversions volume
    #[serde(default)]
    pub conversions_volume_30day: String,
}

/// Response from getting product stats
pub type GetProductStatsResponse = ProductStats;

impl RestClient {
    /// Get product stats
    ///
    /// Gets 30day and 24hour stats for a product.
    /// The volume property is in base currency units. Properties open, high, low are in quote currency units.
    ///
    /// [docs](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getproductstats)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `product_id` - The product ID (e.g., "BTC-USD")
    /// * `request` - The product stats request parameters
    ///
    /// # Returns
    /// A result containing the product stats or an error
    pub async fn get_product_stats(
        &self,
        product_id: &str,
        request: &GetProductStatsRequest,
    ) -> RestResult<GetProductStatsResponse> {
        let endpoint = ENDPOINT_PATH.replace("{}", product_id);
        self.send_get_request(&endpoint, Some(request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_stats_request_serialization() {
        let request = GetProductStatsRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.is_empty());
    }

    #[test]
    fn test_product_stats_deserialization() {
        let json = r#"{
            "open": "28000.00",
            "high": "29500.00",
            "low": "27500.00",
            "last": "28800.00",
            "volume": "1500.25",
            "volume_30day": "45000.75",
            "rfq_volume_24hour": "100.00",
            "rfq_volume_30day": "3000.00",
            "conversions_volume_24hour": "50.00",
            "conversions_volume_30day": "1500.00"
        }"#;

        let stats: ProductStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.open, "28000.00");
        assert_eq!(stats.high, "29500.00");
        assert_eq!(stats.low, "27500.00");
        assert_eq!(stats.last, "28800.00");
        assert_eq!(stats.volume, "1500.25");
        assert_eq!(stats.volume_30day, "45000.75");
    }
}
