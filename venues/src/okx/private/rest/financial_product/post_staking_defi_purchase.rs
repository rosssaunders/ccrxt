use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const STAKING_DEFI_PURCHASE_ENDPOINT: &str = "/api/v5/finance/staking-defi/purchase";

/// Investment data for purchase request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseInvestData {
    /// Investment currency, e.g. BTC
    pub ccy: String,

    /// Investment amount
    pub amt: String,
}

/// Request parameters for On-chain Earn purchase
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostStakingDefiPurchaseRequest {
    /// Product ID
    pub product_id: String,

    /// Investment data
    pub invest_data: Vec<PurchaseInvestData>,

    /// Investment term
    /// Investment term must be specified for fixed-term product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,

    /// Order tag
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Response data for On-chain Earn purchase
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakingDefiPurchaseResponse {
    /// Order ID
    #[serde(rename = "ordId")]
    pub ord_id: String,

    /// Order tag
    pub tag: String,
}

impl RestClient {
    /// Purchase On-chain Earn product
    ///
    /// Creates a new purchase order for On-chain Earn products.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-on-chain-earn-post-purchase)
    ///
    /// Rate limit: 2 requests per second
    /// Rate limit rule: User ID
    /// Permission: Trade
    ///
    /// # Arguments
    /// * `request` - Purchase request parameters
    ///
    /// # Returns
    /// Purchase response with order ID and tag
    pub async fn post_staking_defi_purchase(
        &self,
        request: PostStakingDefiPurchaseRequest,
    ) -> RestResult<StakingDefiPurchaseResponse> {
        self.send_post_request(
            STAKING_DEFI_PURCHASE_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_purchase_invest_data_serialization() {
        let data = PurchaseInvestData {
            ccy: "BTC".to_string(),
            amt: "1.0".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: PurchaseInvestData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_post_staking_defi_purchase_request_serialization() {
        let request = PostStakingDefiPurchaseRequest {
            product_id: "PROD123".to_string(),
            invest_data: vec![PurchaseInvestData {
                ccy: "BTC".to_string(),
                amt: "2.0".to_string(),
            }],
            term: Some("30".to_string()),
            tag: Some("myorder".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("PROD123"));
        assert!(serialized.contains("BTC"));
        assert!(serialized.contains("2.0"));
        assert!(serialized.contains("30"));
        assert!(serialized.contains("myorder"));
    }

    #[test]
    fn test_staking_defi_purchase_response_deserialization() {
        let json_response = r#"{
            "ordId": "12345",
            "tag": "myorder"
        }"#;

        let response: StakingDefiPurchaseResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.ord_id, "12345");
        assert_eq!(response.tag, "myorder");
    }
}
