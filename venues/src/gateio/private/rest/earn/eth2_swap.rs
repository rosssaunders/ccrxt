use super::{RestClient, RestResult};
use serde::Serialize;

const ETH2_SWAP_ENDPOINT: &str = "/earn/staking/eth2/swap";

/// Request parameters for ETH2 swap.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Eth2SwapRequest {
    /// 1-Forward Swap (ETH -> ETH2), 2-Reverse Swap (ETH2 -> ETH)
    pub side: String,

    /// Swap Amount
    pub amount: String,
}

impl RestClient {
    /// ETH2 swap endpoint
    ///
    /// Swaps ETH to ETH2 or vice versa.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#eth2-swap)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The swap request parameters
    ///
    /// # Returns
    /// Empty response - success indicated by HTTP status
    pub async fn eth2_swap(&self, request: Eth2SwapRequest) -> RestResult<()> {
        self.send_post_request::<(), _>(ETH2_SWAP_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eth2_swap_request_serialization() {
        let req = Eth2SwapRequest {
            side: "1".to_string(),
            amount: "1.5".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("side"));
        assert!(json.contains("amount"));
    }
}
