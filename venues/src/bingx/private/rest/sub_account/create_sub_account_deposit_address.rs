use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const CREATE_SUB_ACCOUNT_DEPOSIT_ADDRESS_ENDPOINT: &str =
    "/openApi/wallets/v1/capital/deposit/createSubAddress";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountDepositAddressRequest {
    /// Currency name
    pub coin: String,

    /// Sub-account UID
    pub sub_uid: i64,

    /// Network name
    pub network: String,

    /// Account type: 1 fund account, 2 standard futures account, 3 USDⓢ-M Perp
    pub wallet_type: i32,

    /// Request timestamp in milliseconds
    pub timestamp: i64,

    /// Request valid time window, in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response for creating sub-account deposit address
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountDepositAddressResponse {
    /// Address
    pub address: String,

    /// Address tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<String>,

    /// Deposit address with prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_with_prefix: Option<String>,

    /// Currency name
    pub coin: String,

    /// Network name
    pub network: String,

    /// Address status: 0 for activated, 1 for pending, 2 for not applied
    pub status: i32,

    /// Creation time in Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: i64,

    /// Account type: 1 for Fund Account, 2 for Standard Account, 3 for Perpetual Account, 15 for Spot Account
    pub wallet_type: i32,
}

impl RestClient {
    /// Create deposit address for sub-account
    ///
    /// This node is used for the master user to create a recharge address for the
    /// sub-user. Each currency supports only one recharge address, limited to the
    /// master user. The user who verifies the signature of this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Create%20deposit%20address%20for%20sub-account)
    ///
    /// Rate limit: UID 5/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The create deposit address request parameters
    ///
    /// # Returns
    /// A result containing the deposit address information or an error
    pub async fn create_sub_account_deposit_address(
        &self,
        request: &CreateSubAccountDepositAddressRequest,
    ) -> RestResult<CreateSubAccountDepositAddressResponse> {
        self.send_post_signed_request(
            CREATE_SUB_ACCOUNT_DEPOSIT_ADDRESS_ENDPOINT,
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
    fn test_create_sub_account_deposit_address_request_serialization() {
        let request = CreateSubAccountDepositAddressRequest {
            coin: "USDT".to_string(),
            sub_uid: 123456789,
            network: "TRC20".to_string(),
            wallet_type: 1,
            timestamp: 1640995200000,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"subUid\":123456789"));
        assert!(json.contains("\"network\":\"TRC20\""));
        assert!(json.contains("\"walletType\":1"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(json.contains("\"recvWindow\":5000"));
    }

    #[test]
    fn test_create_sub_account_deposit_address_request_serialization_minimal() {
        let request = CreateSubAccountDepositAddressRequest {
            coin: "BTC".to_string(),
            sub_uid: 987654321,
            network: "BTC".to_string(),
            wallet_type: 3,
            timestamp: 1640995200000,
            recv_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"BTC\""));
        assert!(json.contains("\"subUid\":987654321"));
        assert!(json.contains("\"network\":\"BTC\""));
        assert!(json.contains("\"walletType\":3"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(!json.contains("\"recvWindow\""));
    }

    #[test]
    fn test_create_sub_account_deposit_address_response_deserialization() {
        let json = r#"{
            "address": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
            "addressTag": "memo123",
            "addressWithPrefix": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
            "coin": "USDT",
            "network": "TRC20",
            "status": 0,
            "ts": 1597026383085,
            "walletType": 1
        }"#;

        let response: CreateSubAccountDepositAddressResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.address, "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s");
        assert_eq!(response.address_tag, Some("memo123".to_string()));
        assert_eq!(
            response.address_with_prefix,
            Some("TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s".to_string())
        );
        assert_eq!(response.coin, "USDT");
        assert_eq!(response.network, "TRC20");
        assert_eq!(response.status, 0);
        assert_eq!(response.ts, 1597026383085);
        assert_eq!(response.wallet_type, 1);
    }

    #[test]
    fn test_create_sub_account_deposit_address_response_deserialization_minimal() {
        let json = r#"{
            "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "coin": "BTC",
            "network": "BTC",
            "status": 1,
            "ts": 1597026383085,
            "walletType": 3
        }"#;

        let response: CreateSubAccountDepositAddressResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.address, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(response.address_tag, None);
        assert_eq!(response.address_with_prefix, None);
        assert_eq!(response.coin, "BTC");
        assert_eq!(response.network, "BTC");
        assert_eq!(response.status, 1);
        assert_eq!(response.ts, 1597026383085);
        assert_eq!(response.wallet_type, 3);
    }

    #[test]
    fn test_create_sub_account_deposit_address_status_values() {
        // Test different status values
        let test_cases = vec![(0, "activated"), (1, "pending"), (2, "not applied")];

        for (status_code, _description) in test_cases {
            let json = format!(
                r#"{{
                "address": "test_address",
                "coin": "TEST",
                "network": "TEST",
                "status": {},
                "ts": 1597026383085,
                "walletType": 1
            }}"#,
                status_code
            );

            let response: CreateSubAccountDepositAddressResponse =
                serde_json::from_str(&json).unwrap();
            assert_eq!(response.status, status_code);
        }
    }

    #[test]
    fn test_create_sub_account_deposit_address_wallet_type_values() {
        // Test different wallet type values
        let test_cases = vec![
            (1, "Fund Account"),
            (2, "Standard Account"),
            (3, "Perpetual Account"),
            (15, "Spot Account"),
        ];

        for (wallet_type_code, _description) in test_cases {
            let json = format!(
                r#"{{
                "address": "test_address",
                "coin": "TEST",
                "network": "TEST",
                "status": 0,
                "ts": 1597026383085,
                "walletType": {}
            }}"#,
                wallet_type_code
            );

            let response: CreateSubAccountDepositAddressResponse =
                serde_json::from_str(&json).unwrap();
            assert_eq!(response.wallet_type, wallet_type_code);
        }
    }
}
