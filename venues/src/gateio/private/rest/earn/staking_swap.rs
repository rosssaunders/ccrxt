use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const STAKING_SWAP_ENDPOINT: &str = "/earn/staking/swap";

/// Request parameters for on-chain token swap for earned coins.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StakingSwapRequest {
    /// Currency
    pub coin: String,

    /// 0 - Stake, 1 - Redeem
    pub side: String,

    /// Amount
    pub amount: String,

    /// DeFi-type Mining Protocol Identifier. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
}

/// Response for staking swap.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StakingSwapResponse {
    /// Order ID
    pub id: i32,

    /// Product ID
    pub pid: i32,

    /// User ID
    pub uid: i32,

    /// Currency
    pub coin: String,

    /// Type 0-Staking 1-Redemption
    pub r#type: i32,

    /// 子Type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,

    /// Amount
    pub amount: String,

    /// Exchange ratio
    pub exchange_rate: String,

    /// Redemption Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_amount: Option<String>,

    /// 更新Timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_stamp: Option<i32>,

    /// Transaction timestamp
    #[serde(alias = "createStamp")]
    pub create_stamp: i32,

    /// status 1-success
    pub status: i32,

    /// DEFI Protocol Type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<i32>,

    /// Reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Order Origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl RestClient {
    /// On-chain token swap for earned coins endpoint
    ///
    /// Swaps earned coins on-chain.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#on-chain-token-swap-for-earned-coins)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The staking swap request parameters
    ///
    /// # Returns
    /// Staking swap response
    pub async fn staking_swap(
        &self,
        request: StakingSwapRequest,
    ) -> RestResult<StakingSwapResponse> {
        self.send_post_request::<StakingSwapResponse, _>(STAKING_SWAP_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staking_swap_request_serialization() {
        let req = StakingSwapRequest {
            coin: "GT".to_string(),
            side: "0".to_string(),
            amount: "1.5".to_string(),
            pid: Some(1),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("coin"));
        assert!(json.contains("side"));
        assert!(json.contains("amount"));
        assert!(json.contains("pid"));
    }

    #[test]
    fn test_staking_swap_response_deserialization() {
        let json = r#"{
            "id": 21000,
            "uid": 12345,
            "coin": "GT",
            "type": 0,
            "exchange_rate": "1.00000000",
            "amount": "2",
            "pid": 1,
            "status": 1,
            "createStamp": 1752200661
        }"#;
        let resp: StakingSwapResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, 21000);
        assert_eq!(resp.coin, "GT");
        assert_eq!(resp.status, 1);
    }
}
