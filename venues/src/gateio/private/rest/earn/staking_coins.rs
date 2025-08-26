use serde::Serialize;

use super::{RestClient, RestResult};

const STAKING_COINS_ENDPOINT: &str = "/earn/staking/coins";

/// Request parameters for staking coins.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StakingCoinsRequest {
    /// Currency. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// Token Type: swap-Voucher, lock-Locked. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cointype: Option<String>,
}

impl RestClient {
    /// Staking coins endpoint
    ///
    /// Returns a list of staking coins.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#staking-coins)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The staking coins request parameters
    ///
    /// # Returns
    /// List of staking coins
    pub async fn staking_coins(&self, request: StakingCoinsRequest) -> RestResult<Vec<String>> {
        self.send_post_request::<Vec<String>, _>(STAKING_COINS_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staking_coins_request_serialization() {
        let req = StakingCoinsRequest {
            coin: Some("GT".to_string()),
            cointype: Some("swap".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("coin"));
        assert!(json.contains("cointype"));
    }
}
