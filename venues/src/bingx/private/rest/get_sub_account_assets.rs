use serde::{Deserialize, Serialize};

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

impl GetSubAccountAssetsRequest {
    pub fn new(email: String, timestamp: i64) -> Self {
        Self {
            email,
            recv_window: None,
            timestamp,
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sub_account_assets_request_serialization() {
        let request = GetSubAccountAssetsRequest::new(
            "test@example.com".to_string(),
            1640995200000,
        )
        .recv_window(5000);

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
