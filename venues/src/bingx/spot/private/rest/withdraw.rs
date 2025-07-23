use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult, WalletType};

const WITHDRAW_ENDPOINT: &str = "/openApi/wallets/v1/capital/withdraw/apply";

/// Request for withdrawing funds
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawRequest {
    /// Coin name
    pub coin: String,
    /// Network name (optional, uses default if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Withdrawal address
    pub address: String,
    /// Address tag or memo (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<String>,
    /// Withdrawal amount
    pub amount: f64,
    /// Account type
    pub wallet_type: WalletType,
    /// Customer-defined withdrawal ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,
    /// Payment platform information (only for Korean users)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vasp_entity_id: Option<String>,
    /// Recipient's surname in English (only for Korean users)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_last_name: Option<String>,
    /// Recipient's first name in English (only for Korean users)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_first_name: Option<String>,
    /// Recipient's date of birth (only for Korean users)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Execution window time, cannot be greater than 60000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// Response for withdrawal request
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawResponse {
    /// Platform unique withdrawal record ID
    pub id: String,
    /// Customer-defined withdrawal ID
    #[serde(rename = "withdrawOrderId")]
    pub withdraw_order_id: Option<String>,
}

impl RestClient {
    /// Withdraw funds
    ///
    /// Initiates a withdrawal from the user's account to an external address.
    ///
    /// # Arguments
    /// * `request` - The withdrawal request parameters
    ///
    /// # Returns
    /// A result containing the withdrawal response or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 2/s
    /// - IP rate limit group 2
    ///
    /// # API Permissions
    /// - Withdraw permission required
    ///
    /// # API Documentation
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/wallet-api.html#Withdraw
    pub async fn withdraw(&self, request: &WithdrawRequest) -> RestResult<WithdrawResponse> {
        self.send_request(
            WITHDRAW_ENDPOINT,
            reqwest::Method::POST,
            Some(request),
            EndpointType::AccountApiGroup2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_request_serialization() {
        let request = WithdrawRequest {
            coin: "BTC".to_string(),
            network: Some("BTC".to_string()),
            address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            address_tag: None,
            amount: 0.1,
            wallet_type: WalletType::Spot,
            withdraw_order_id: Some("custom123".to_string()),
            vasp_entity_id: None,
            recipient_last_name: None,
            recipient_first_name: None,
            date_of_birth: None,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("coin=BTC"));
        assert!(serialized.contains("address=1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"));
        assert!(serialized.contains("amount=0.1"));
        assert!(serialized.contains("wallet_type=15"));
    }

    #[test]
    fn test_withdraw_response_deserialization() {
        let json = r#"{
            "id": "12345",
            "withdrawOrderId": "custom123"
        }"#;

        let response: WithdrawResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "12345");
        assert_eq!(response.withdraw_order_id, Some("custom123".to_string()));
    }
}
