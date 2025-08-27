use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const SOL_STAKING_APY_HISTORY_ENDPOINT: &str = "/api/v5/finance/staking-defi/sol/apy-history";

/// Request parameters for SOL staking APY history
#[derive(Debug, Clone, Serialize)]
pub struct GetSolStakingApyHistoryRequest {
    /// Get the days of APY(Annual percentage yield) history record in the past
    /// No more than 365 days
    pub days: String,
}

/// Response data for SOL staking APY history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolStakingApyHistoryData {
    /// APY(Annual percentage yield), e.g. 0.01 represents 1%
    pub rate: String,

    /// Data time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: String,
}

impl RestClient {
    /// Get SOL Staking APY history
    ///
    /// Public endpoint that retrieves the APY history for SOL staking.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-sol-staking-get-apy-history-public)
    ///
    /// Rate limit: 6 requests per second
    /// Rate limit rule: IP
    ///
    /// # Arguments
    /// * `request` - Request parameters including days to retrieve
    ///
    /// # Returns
    /// A vector of APY history data points wrapped in ApiResponse
    pub async fn get_sol_staking_apy_history(
        &self,
        request: GetSolStakingApyHistoryRequest,
    ) -> RestResult<SolStakingApyHistoryData> {
        self.send_get_request(
            SOL_STAKING_APY_HISTORY_ENDPOINT,
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
    fn test_get_sol_staking_apy_history_request_serialization() {
        let request = GetSolStakingApyHistoryRequest {
            days: "30".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("days=30"));
    }

    #[test]
    fn test_sol_staking_apy_history_data_serialization() {
        let data = SolStakingApyHistoryData {
            rate: "0.065".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: SolStakingApyHistoryData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_sol_staking_apy_history_data_deserialization_from_api() {
        let json_response = r#"[
            {
                "rate": "0.068",
                "ts": "1597026383085"
            },
            {
                "rate": "0.072",
                "ts": "1597112783085"
            }
        ]"#;

        let data: Vec<SolStakingApyHistoryData> = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].rate, "0.068");
        assert_eq!(data[0].ts, "1597026383085");
        assert_eq!(data[1].rate, "0.072");
        assert_eq!(data[1].ts, "1597112783085");
    }

    #[test]
    fn test_sol_staking_apy_history_data_empty_array() {
        let json_response = r#"[]"#;

        let data: Vec<SolStakingApyHistoryData> = serde_json::from_str(json_response).unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_get_sol_staking_apy_history_request_min_days() {
        let request = GetSolStakingApyHistoryRequest {
            days: "1".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("days=1"));
    }
}
