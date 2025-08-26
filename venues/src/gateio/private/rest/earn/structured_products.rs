use super::{RestClient, RestResult};
use serde::{Deserialize, Serialize};

const STRUCTURED_PRODUCTS_ENDPOINT: &str = "/earn/structured/products";

/// Request parameters for Structured Product List.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructuredProductsRequest {
    /// Product Type (Default empty to query all). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Status (Default empty to query all). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Page number. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records returned in a single list. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Represents a single Structured Product.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StructuredProduct {
    /// Product ID
    pub id: i32,

    /// Product Type
    pub r#type: String,

    /// Product Name
    pub name_en: String,

    /// Investment Period
    pub investment_period: String,

    /// Minimum Annual Rate
    pub min_annual_rate: String,

    /// Intermediate Annual Rate
    pub mid_annual_rate: String,

    /// Maximum Annual Rate
    pub max_annual_rate: String,

    /// Underlying Market
    pub watch_market: String,

    /// Investment Token
    pub investment_coin: String,

    /// Start Time
    pub start_time: i32,

    /// End time
    pub end_time: i32,

    /// Status: in_process, will_begin, wait_settlement, done
    pub status: String,
}

impl RestClient {
    /// Structured Product List endpoint
    ///
    /// Returns a list of structured products.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#structured-product-list)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The product list request parameters
    ///
    /// # Returns
    /// List of structured products
    pub async fn structured_products(
        &self,
        request: StructuredProductsRequest,
    ) -> RestResult<Vec<StructuredProduct>> {
        self.send_get_request::<Vec<StructuredProduct>, _>(
            STRUCTURED_PRODUCTS_ENDPOINT,
            Some(&request),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_products_request_serialization() {
        let req = StructuredProductsRequest {
            r#type: Some("BullishSharkFin".to_string()),
            status: Some("in_process".to_string()),
            page: Some(1),
            limit: Some(10),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("type"));
        assert!(json.contains("status"));
        assert!(json.contains("page"));
        assert!(json.contains("limit"));
    }

    #[test]
    fn test_structured_product_deserialization() {
        let json = r#"{
            "id": 3700,
            "type": "BullishSharkFin",
            "name_en": "Bullish Sharkfin_USDT",
            "investment_period": "7",
            "min_annual_rate": "0.50",
            "mid_annual_rate": "7.50",
            "max_annual_rate": "13.00",
            "watch_market": "BTC_USDT",
            "investment_coin": "USDT",
            "start_time": 1698224400,
            "end_time": 1700902800,
            "status": "in_process"
        }"#;
        let product: StructuredProduct = serde_json::from_str(json).unwrap();
        assert_eq!(product.id, 3700);
        assert_eq!(product.r#type, "BullishSharkFin");
        assert_eq!(product.status, "in_process");
    }
}
