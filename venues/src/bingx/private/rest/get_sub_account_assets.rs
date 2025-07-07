use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

const SUB_ACCOUNT_ASSETS_ENDPOINT: &str = "/openApi/subAccount/v1/subAccountAsset";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountAssetsRequest {
    /// Sub-account email
    pub email: String,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountAssetsResponse {
    /// List of balances
    pub balances: Vec<SubAccountAsset>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountAsset {
    /// Asset symbol
    pub asset: String,
    /// Free balance
    pub free: String,
    /// Locked balance
    pub locked: String,
}

impl RestClient {
    /// Get sub-account assets
    ///
    /// Query the assets of a sub-account.
    ///
    /// # Arguments
    /// * `request` - The sub-account assets request
    ///
    /// # Returns
    /// A result containing the sub-account assets response or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 10/s
    ///
    /// # API Permissions
    /// - SubAccount-Read permission required
    pub async fn get_sub_account_assets(
        &self,
        request: &GetSubAccountAssetsRequest,
    ) -> RestResult<GetSubAccountAssetsResponse> {
        self.send_request(
            SUB_ACCOUNT_ASSETS_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sub_account_assets_request_serialization() {
        let request = GetSubAccountAssetsRequest {
            email: "test@example.com".to_string(),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"email\":\"test@example.com\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_get_sub_account_assets_response_deserialization() {
        let json = r#"
        {
            "balances": [
                {
                    "asset": "BTC",
                    "free": "0.00123456",
                    "locked": "0.0"
                },
                {
                    "asset": "USDT",
                    "free": "100.50",
                    "locked": "25.75"
                }
            ]
        }
        "#;

        let response: GetSubAccountAssetsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.balances.len(), 2);
        assert_eq!(response.balances[0].asset, "BTC");
        assert_eq!(response.balances[0].free, "0.00123456");
        assert_eq!(response.balances[0].locked, "0.0");
        assert_eq!(response.balances[1].asset, "USDT");
        assert_eq!(response.balances[1].free, "100.50");
        assert_eq!(response.balances[1].locked, "25.75");
    }
}
