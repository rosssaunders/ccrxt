use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const SOL_STAKING_PRODUCT_INFO_ENDPOINT: &str = "/api/v5/finance/staking-defi/sol/product-info";

/// Response data for SOL staking product info
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolStakingProductInfoData {
    /// Fast redemption daily limit
    /// The master account and sub-accounts share the same limit
    #[serde(rename = "fastRedemptionDailyLimit")]
    pub fast_redemption_daily_limit: String,

    /// Currently fast redemption max available amount
    #[serde(rename = "fastRedemptionAvail")]
    pub fast_redemption_avail: String,
}

impl RestClient {
    /// Get SOL Staking product info
    ///
    /// Retrieves product information for SOL staking including daily limits.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-sol-staking-get-product-info)
    ///
    /// Rate limit: 3 requests per second
    /// Rate limit rule: User ID
    /// Permission: Read
    ///
    /// # Returns
    /// Product information for SOL staking
    pub async fn get_sol_staking_product_info(&self) -> RestResult<SolStakingProductInfoData> {
        self.send_get_request(
            SOL_STAKING_PRODUCT_INFO_ENDPOINT,
            None::<&()>,
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
    fn test_sol_staking_product_info_data_serialization() {
        let data = SolStakingProductInfoData {
            fast_redemption_daily_limit: "2000.0".to_string(),
            fast_redemption_avail: "1500.0".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: SolStakingProductInfoData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_sol_staking_product_info_data_deserialization_from_api() {
        let json_response = r#"{
            "fastRedemptionDailyLimit": "750.0",
            "fastRedemptionAvail": "250.0"
        }"#;

        let data: SolStakingProductInfoData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.fast_redemption_daily_limit, "750.0");
        assert_eq!(data.fast_redemption_avail, "250.0");
    }
}
