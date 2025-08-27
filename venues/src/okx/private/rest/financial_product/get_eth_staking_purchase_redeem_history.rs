use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const ETH_STAKING_PURCHASE_REDEEM_HISTORY_ENDPOINT: &str =
    "/api/v5/finance/staking-defi/eth/purchase-redeem-history";

/// Request parameters for ETH staking purchase & redeem history
#[derive(Debug, Clone, Serialize)]
pub struct GetEthStakingPurchaseRedeemHistoryRequest {
    /// Type
    /// purchase, redeem
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Status
    /// pending, success, failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Pagination of data to return records earlier than the requestTime.
    /// The value passed is the corresponding timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requestTime.
    /// The value passed is the corresponding timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The default is 100. The maximum is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response data for ETH staking purchase & redeem history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthStakingPurchaseRedeemHistoryData {
    /// Type
    /// purchase, redeem
    pub r#type: String,

    /// Purchase/Redeem amount
    pub amt: String,

    /// Redeeming amount
    #[serde(rename = "redeemingAmt")]
    pub redeeming_amt: String,

    /// Status
    /// pending, success, failed
    pub status: String,

    /// Request time of make purchase/redeem, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "requestTime")]
    pub request_time: String,

    /// Completed time of redeem settlement, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "completedTime")]
    pub completed_time: String,

    /// Estimated completed time of redeem settlement, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "estCompletedTime")]
    pub est_completed_time: String,
}

impl RestClient {
    /// Get ETH Staking purchase & redeem history
    ///
    /// Retrieves the history of purchases and redemptions for ETH staking.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-eth-staking-get-purchase-amp-redeem-history)
    ///
    /// Rate limit: 6 requests per second
    /// Rate limit rule: User ID
    /// Permission: Read
    ///
    /// # Arguments
    /// * `request` - Request parameters for filtering purchase & redeem history
    ///
    /// # Returns
    /// A vector of purchase & redeem history data
    pub async fn get_eth_staking_purchase_redeem_history(
        &self,
        request: GetEthStakingPurchaseRedeemHistoryRequest,
    ) -> RestResult<Vec<EthStakingPurchaseRedeemHistoryData>> {
        self.send_get_request(
            ETH_STAKING_PURCHASE_REDEEM_HISTORY_ENDPOINT,
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
    fn test_get_eth_staking_purchase_redeem_history_request_serialization() {
        let request = GetEthStakingPurchaseRedeemHistoryRequest {
            r#type: Some("purchase".to_string()),
            status: Some("success".to_string()),
            after: Some("1597026383085".to_string()),
            before: Some("1597112783085".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=purchase"));
        assert!(serialized.contains("status=success"));
        assert!(serialized.contains("after=1597026383085"));
        assert!(serialized.contains("before=1597112783085"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_eth_staking_purchase_redeem_history_data_serialization() {
        let data = EthStakingPurchaseRedeemHistoryData {
            r#type: "redeem".to_string(),
            amt: "5.0".to_string(),
            redeeming_amt: "4.95".to_string(),
            status: "pending".to_string(),
            request_time: "1597026383085".to_string(),
            completed_time: "1597112783085".to_string(),
            est_completed_time: "1597112783085".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: EthStakingPurchaseRedeemHistoryData =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_eth_staking_purchase_redeem_history_data_deserialization_from_api() {
        let json_response = r#"[
            {
                "type": "purchase",
                "amt": "10.0",
                "redeemingAmt": "0",
                "status": "success",
                "requestTime": "1597026383085",
                "completedTime": "1597026483085",
                "estCompletedTime": "1597026483085"
            }
        ]"#;

        let data: Vec<EthStakingPurchaseRedeemHistoryData> =
            serde_json::from_str(json_response).unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].r#type, "purchase");
        assert_eq!(data[0].amt, "10.0");
        assert_eq!(data[0].status, "success");
    }
}
