use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const ETH_STAKING_APY_HISTORY_ENDPOINT: &str = "/api/v5/finance/staking-defi/eth/apy-history";

/// Request parameters for ETH staking APY history
#[derive(Debug, Clone, Serialize)]
pub struct GetEthStakingApyHistoryRequest {
    /// Get the days of APY(Annual percentage yield) history record in the past
    /// No more than 365 days
    pub days: String,
}

/// Response data for ETH staking APY history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthStakingApyHistoryData {
    /// APY(Annual percentage yield), e.g. 0.01 represents 1%
    pub rate: String,

    /// Data time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: String,
}

impl RestClient {
    /// Get ETH Staking APY history
    ///
    /// Public endpoint that retrieves the APY history for ETH staking.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-eth-staking-get-apy-history-public)
    ///
    /// Rate limit: 6 requests per second
    /// Rate limit rule: IP
    ///
    /// # Arguments
    /// * `request` - Request parameters including days to retrieve
    ///
    /// # Returns
    /// A vector of APY history data points wrapped in ApiResponse
    pub async fn get_eth_staking_apy_history(
        &self,
        request: GetEthStakingApyHistoryRequest,
    ) -> RestResult<EthStakingApyHistoryData> {
        self.send_get_request(
            ETH_STAKING_APY_HISTORY_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_eth_staking_apy_history_request_serialization() {
        let request = GetEthStakingApyHistoryRequest {
            days: "7".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("days=7"));
    }

    #[test]
    fn test_eth_staking_apy_history_data_serialization() {
        let data = EthStakingApyHistoryData {
            rate: "0.05".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: EthStakingApyHistoryData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_eth_staking_apy_history_data_deserialization_from_api() {
        let json_response = r#"[
            {
                "rate": "0.045",
                "ts": "1597026383085"
            },
            {
                "rate": "0.047",
                "ts": "1597112783085"
            }
        ]"#;

        let data: Vec<EthStakingApyHistoryData> = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].rate, "0.045");
        assert_eq!(data[0].ts, "1597026383085");
        assert_eq!(data[1].rate, "0.047");
        assert_eq!(data[1].ts, "1597112783085");
    }

    #[test]
    fn test_eth_staking_apy_history_data_empty_array() {
        let json_response = r#"[]"#;

        let data: Vec<EthStakingApyHistoryData> = serde_json::from_str(json_response).unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_get_eth_staking_apy_history_request_max_days() {
        let request = GetEthStakingApyHistoryRequest {
            days: "365".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("days=365"));
    }
}
