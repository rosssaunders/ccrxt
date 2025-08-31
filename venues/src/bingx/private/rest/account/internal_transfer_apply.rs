use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const INTERNAL_TRANSFER_APPLY_ENDPOINT: &str = "/openApi/wallets/v1/capital/innerTransfer/apply";

/// Request to create a main account internal transfer
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferApplyRequest {
    /// Name of the transferred currency (required)
    pub coin: String,

    /// User account type (required): 1=UID, 2=phone number, 3=email
    pub user_account_type: i32,

    /// User account: UID, phone number, email (required)
    pub user_account: String,

    /// Transfer amount (required)
    pub amount: Decimal,

    /// Area code for telephone (optional) - required when userAccountType=2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_code: Option<String>,

    /// Account type (required): 1=Fund Account, 2=Standard Futures Account, 3=Perpetual Futures Account
    pub wallet_type: i32,

    /// Custom ID for internal transfer by the client (optional) - combination of numbers and letters, length less than 100 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_client_id: Option<String>,

    /// The timestamp of the request, in milliseconds (required)
    pub timestamp: i64,

    /// Request validity time window, unit: milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response from the internal transfer apply endpoint
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferApplyResponse {
    /// The platform returns the unique ID of the internal transfer record
    pub id: String,

    /// Custom ID for internal transfer by the client, combination of numbers and letters, length less than 100 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_client_id: Option<String>,
}

impl RestClient {
    /// Main Accoun internal transfer
    ///
    /// Users can transfer money to each other within the bingx platform. Transfers are
    /// only allowed between main accounts and from main accounts to sub-accounts.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/account-api.html#Main%20Accoun%20internal%20transfer)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The internal transfer apply request
    ///
    /// # Returns
    /// A result containing the transfer response or an error
    pub async fn internal_transfer_apply(
        &self,
        request: &InternalTransferApplyRequest,
    ) -> RestResult<InternalTransferApplyResponse> {
        self.send_post_signed_request(
            INTERNAL_TRANSFER_APPLY_ENDPOINT,
            request,
            EndpointType::Trading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_internal_transfer_apply_request_serialization() {
        let request = InternalTransferApplyRequest {
            coin: "USDT".to_string(),
            user_account_type: 1,
            user_account: "123456789".to_string(),
            amount: dec!(100.5),
            calling_code: None,
            wallet_type: 1,
            transfer_client_id: Some("CLIENT123".to_string()),
            timestamp: 1640995200000,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"userAccountType\":1"));
        assert!(json.contains("\"userAccount\":\"123456789\""));
        assert!(json.contains("\"amount\":\"100.5\""));
        assert!(json.contains("\"walletType\":1"));
        assert!(json.contains("\"transferClientId\":\"CLIENT123\""));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(json.contains("\"recvWindow\":5000"));
    }

    #[test]
    fn test_internal_transfer_apply_response_deserialization() {
        let json = r#"
        {
            "id": "TRANSFER123456",
            "transferClientId": "CLIENT123"
        }
        "#;

        let response: InternalTransferApplyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "TRANSFER123456");
        assert_eq!(response.transfer_client_id, Some("CLIENT123".to_string()));
    }

    #[test]
    fn test_phone_transfer_request() {
        let request = InternalTransferApplyRequest {
            coin: "BTC".to_string(),
            user_account_type: 2,
            user_account: "1234567890".to_string(),
            amount: dec!(0.001),
            calling_code: Some("+1".to_string()),
            wallet_type: 3,
            transfer_client_id: None,
            timestamp: 1640995200000,
            recv_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"userAccountType\":2"));
        assert!(json.contains("\"callingCode\":\"+1\""));
        assert!(json.contains("\"walletType\":3"));
        assert!(!json.contains("transferClientId"));
        assert!(!json.contains("recvWindow"));
    }
}
