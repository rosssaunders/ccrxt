use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult, WalletType};

const WITHDRAW_ENDPOINT: &str = "/openApi/wallets/v1/capital/withdraw/apply";

/// Request for withdrawing funds
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRequest {
    /// Coin name (required)
    pub coin: String,

    /// Network name, use default network if not transmitted (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    /// Withdrawal address (required)
    pub address: String,

    /// Tag or memo, some currencies support tag or memo (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<String>,

    /// Withdrawal amount (required)
    pub amount: f64,

    /// Account type: 1 fund account, 2 standard account, 3 perpetual account, 15 spot account (required)
    pub wallet_type: WalletType,

    /// Customer-defined withdrawal ID, a combination of numbers and letters, with a length of less than 100 characters (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,

    /// Payment platform information, only KYC=KOR (Korean individual users) must pass this field (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vasp_entity_id: Option<String>,

    /// The recipient's surname is in English, and only KYC=KOR (Korean individual users) must pass this field (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_last_name: Option<String>,

    /// The recipient's name in English, only KYC=KOR (Korean individual users) must pass this field (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_first_name: Option<String>,

    /// The payee's date of birth (example 1999-09-09) must be passed as this field only for KYC=KOR (Korean individual users) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,

    /// Timestamp of initiating the request, Unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Request valid time window value, Unit: milliseconds (required)
    pub timestamp: i64,
}

/// Response for withdrawal request
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawResponse {
    /// The platform returns the unique ID of the internal transfer record
    pub id: String,

    /// Customer-defined withdrawal ID, a combination of numbers and letters, with a length of less than 100 characters
    #[serde(rename = "withdrawOrderId")]
    pub withdraw_order_id: Option<String>,
}

impl RestClient {
    /// Withdraw
    ///
    /// Specify user account to initiate coin withdrawal.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/wallet-api.html#Withdraw)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The withdrawal request parameters
    ///
    /// # Returns
    /// A result containing the withdrawal response or an error
    pub async fn withdraw(&self, request: WithdrawRequest) -> RestResult<WithdrawResponse> {
        self.send_post_signed_request(WITHDRAW_ENDPOINT, &request, EndpointType::AccountApiGroup2)
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
        assert!(serialized.contains("walletType=15"));
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
