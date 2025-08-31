use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const MAIN_ACCOUNT_INTERNAL_TRANSFER_ENDPOINT: &str =
    "/openApi/wallets/v1/capital/innerTransfer/apply";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MainAccountInternalTransferRequest {
    /// Name of the transferred currency
    pub coin: String,

    /// User account type 1=UID 2=phone number 3=email
    pub user_account_type: i32,

    /// User account: UID, phone number, email
    pub user_account: String,

    /// Transfer amount
    pub amount: f64,

    /// Area code for telephone, required when userAccountType=2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_code: Option<String>,

    /// Custom ID for internal transfer by the client, combination of numbers and letters, length less than 100 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_client_id: Option<String>,

    /// Account type, 1 Fund Account; 2 Standard Futures Account; 3 Perpetual Futures Account
    pub wallet_type: i32,

    /// The timestamp of the request, in milliseconds
    pub timestamp: i64,

    /// Request validity time window, unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response for main account internal transfer
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainAccountInternalTransferResponse {
    /// The platform returns the unique ID of the internal transfer record
    pub id: String,

    /// Custom ID for internal transfer by the client, combination of numbers and letters, length less than 100 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_client_id: Option<String>,
}

impl RestClient {
    /// Main Account internal transfer
    ///
    /// Users can transfer money to each other within the bingx platform. Transfers are
    /// only allowed between main accounts and from main accounts to sub-accounts.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Main%20Accoun%20internal%20transfer)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The main account internal transfer request parameters
    ///
    /// # Returns
    /// A result containing the transfer response with unique ID or an error
    pub async fn main_account_internal_transfer(
        &self,
        request: &MainAccountInternalTransferRequest,
    ) -> RestResult<MainAccountInternalTransferResponse> {
        self.send_post_signed_request(
            MAIN_ACCOUNT_INTERNAL_TRANSFER_ENDPOINT,
            request,
            EndpointType::AccountApiGroup2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_account_internal_transfer_request_serialization() {
        let request = MainAccountInternalTransferRequest {
            coin: "USDT".to_string(),
            user_account_type: 1,
            user_account: "123456789".to_string(),
            amount: 100.0,
            calling_code: None,
            transfer_client_id: Some("test_transfer_123".to_string()),
            wallet_type: 1,
            timestamp: 1640995200000,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"userAccountType\":1"));
        assert!(json.contains("\"userAccount\":\"123456789\""));
        assert!(json.contains("\"amount\":100.0"));
        assert!(json.contains("\"transferClientId\":\"test_transfer_123\""));
        assert!(json.contains("\"walletType\":1"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(json.contains("\"recvWindow\":5000"));
    }

    #[test]
    fn test_main_account_internal_transfer_request_serialization_minimal() {
        let request = MainAccountInternalTransferRequest {
            coin: "BTC".to_string(),
            user_account_type: 3,
            user_account: "test@example.com".to_string(),
            amount: 0.001,
            calling_code: None,
            transfer_client_id: None,
            wallet_type: 3,
            timestamp: 1640995200000,
            recv_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"BTC\""));
        assert!(json.contains("\"userAccountType\":3"));
        assert!(json.contains("\"userAccount\":\"test@example.com\""));
        assert!(json.contains("\"amount\":0.001"));
        assert!(json.contains("\"walletType\":3"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(!json.contains("\"transferClientId\""));
        assert!(!json.contains("\"recvWindow\""));
        assert!(!json.contains("\"callingCode\""));
    }

    #[test]
    fn test_main_account_internal_transfer_request_with_phone() {
        let request = MainAccountInternalTransferRequest {
            coin: "ETH".to_string(),
            user_account_type: 2,
            user_account: "1234567890".to_string(),
            amount: 1.5,
            calling_code: Some("+1".to_string()),
            transfer_client_id: None,
            wallet_type: 2,
            timestamp: 1640995200000,
            recv_window: Some(10000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"ETH\""));
        assert!(json.contains("\"userAccountType\":2"));
        assert!(json.contains("\"userAccount\":\"1234567890\""));
        assert!(json.contains("\"amount\":1.5"));
        assert!(json.contains("\"callingCode\":\"+1\""));
        assert!(json.contains("\"walletType\":2"));
        assert!(json.contains("\"recvWindow\":10000"));
    }

    #[test]
    fn test_main_account_internal_transfer_response_deserialization() {
        let json = r#"{"id":"12345678901234567890","transferClientId":"test_transfer_123"}"#;
        let response: MainAccountInternalTransferResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.id, "12345678901234567890");
        assert_eq!(
            response.transfer_client_id,
            Some("test_transfer_123".to_string())
        );
    }

    #[test]
    fn test_main_account_internal_transfer_response_deserialization_minimal() {
        let json = r#"{"id":"12345678901234567890"}"#;
        let response: MainAccountInternalTransferResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.id, "12345678901234567890");
        assert_eq!(response.transfer_client_id, None);
    }
}
