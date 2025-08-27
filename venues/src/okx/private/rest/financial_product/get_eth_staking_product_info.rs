use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const ETH_STAKING_PRODUCT_INFO_ENDPOINT: &str = "/api/v5/finance/staking-defi/eth/product-info";

/// Response data for ETH staking product info
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthStakingProductInfoData {
    /// Fast redemption daily limit
    /// The master account and sub-accounts share the same limit
    #[serde(rename = "fastRedemptionDailyLimit")]
    pub fast_redemption_daily_limit: String,
}

impl RestClient {
    /// Get ETH Staking product info
    ///
    /// Retrieves product information for ETH staking including daily limits.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-eth-staking-get-product-info)
    ///
    /// Rate limit: 3 requests per second
    /// Rate limit rule: User ID
    /// Permission: Read
    ///
    /// # Returns
    /// Product information for ETH staking
    pub async fn get_eth_staking_product_info(&self) -> RestResult<EthStakingProductInfoData> {
        self.send_get_request(
            ETH_STAKING_PRODUCT_INFO_ENDPOINT,
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
    fn test_eth_staking_product_info_data_serialization() {
        let data = EthStakingProductInfoData {
            fast_redemption_daily_limit: "1000.0".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: EthStakingProductInfoData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_eth_staking_product_info_data_deserialization_from_api() {
        let json_response = r#"{
            "fastRedemptionDailyLimit": "500.0"
        }"#;

        let data: EthStakingProductInfoData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.fast_redemption_daily_limit, "500.0");
    }

    #[test]
    fn test_eth_staking_product_info_data_zero_limit() {
        let json_response = r#"{
            "fastRedemptionDailyLimit": "0"
        }"#;

        let data: EthStakingProductInfoData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.fast_redemption_daily_limit, "0");
    }

    #[test]
    fn test_eth_staking_product_info_data_large_limit() {
        let json_response = r#"{
            "fastRedemptionDailyLimit": "10000.5"
        }"#;

        let data: EthStakingProductInfoData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.fast_redemption_daily_limit, "10000.5");
    }
}
