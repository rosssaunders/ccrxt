use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const GET_SUB_ACCOUNT_DEPOSIT_ADDRESS_ENDPOINT: &str =
    "/openApi/wallets/v1/capital/subAccount/deposit/address";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountDepositAddressRequest {
    /// Name of the transfer coin
    pub coin: String,

    /// Sub-account UID
    pub sub_uid: i64,

    /// Starting record number, default is 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Page size, default is 100, maximum is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Timestamp of the request in milliseconds
    pub timestamp: i64,

    /// Request valid time window, in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Individual deposit address information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    /// Coin name
    pub coin: String,

    /// Network name
    pub network: String,

    /// Deposit address
    pub address: String,

    /// Deposit address with prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_with_prefix: Option<String>,

    /// Address tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Status: 0 Activated, 1 Applied, 2 Not applied
    pub status: i32,

    /// Account type: 1 for Fund Account, 2 for Standard Account, 3 for Perpetual Account, 15 for Spot Account
    pub wallet_type: i32,
}

/// Response for getting sub-account deposit address
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountDepositAddressResponse {
    /// List of deposit addresses
    pub data: Vec<DepositAddress>,

    /// Total number of addresses
    pub total: i32,
}

impl RestClient {
    /// Get sub-account deposit address
    ///
    /// This endpoint is used for the parent user to query the deposit address of a
    /// specific coin for a child user in the blockchain where the child user is
    /// located. The user who verifies the signature of this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Get%20sub-account%20deposit%20address)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The get deposit address request parameters
    ///
    /// # Returns
    /// A result containing the deposit address information or an error
    pub async fn get_sub_account_deposit_address(
        &self,
        request: &GetSubAccountDepositAddressRequest,
    ) -> RestResult<GetSubAccountDepositAddressResponse> {
        self.send_get_signed_request(
            GET_SUB_ACCOUNT_DEPOSIT_ADDRESS_ENDPOINT,
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
    fn test_get_sub_account_deposit_address_request_serialization() {
        let request = GetSubAccountDepositAddressRequest {
            coin: "USDT".to_string(),
            sub_uid: 123456789,
            offset: Some(0),
            limit: Some(100),
            timestamp: 1640995200000,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"subUid\":123456789"));
        assert!(json.contains("\"offset\":0"));
        assert!(json.contains("\"limit\":100"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(json.contains("\"recvWindow\":5000"));
    }

    #[test]
    fn test_get_sub_account_deposit_address_request_serialization_minimal() {
        let request = GetSubAccountDepositAddressRequest {
            coin: "BTC".to_string(),
            sub_uid: 987654321,
            offset: None,
            limit: None,
            timestamp: 1640995200000,
            recv_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"coin\":\"BTC\""));
        assert!(json.contains("\"subUid\":987654321"));
        assert!(json.contains("\"timestamp\":1640995200000"));
        assert!(!json.contains("\"offset\""));
        assert!(!json.contains("\"limit\""));
        assert!(!json.contains("\"recvWindow\""));
    }

    #[test]
    fn test_deposit_address_deserialization() {
        let json = r#"{
            "coin": "USDT",
            "network": "TRC20",
            "address": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
            "addressWithPrefix": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
            "tag": "memo123",
            "status": 0,
            "walletType": 1
        }"#;

        let address: DepositAddress = serde_json::from_str(json).unwrap();

        assert_eq!(address.coin, "USDT");
        assert_eq!(address.network, "TRC20");
        assert_eq!(address.address, "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s");
        assert_eq!(
            address.address_with_prefix,
            Some("TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s".to_string())
        );
        assert_eq!(address.tag, Some("memo123".to_string()));
        assert_eq!(address.status, 0);
        assert_eq!(address.wallet_type, 1);
    }

    #[test]
    fn test_deposit_address_deserialization_minimal() {
        let json = r#"{
            "coin": "BTC",
            "network": "BTC",
            "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "status": 1,
            "walletType": 3
        }"#;

        let address: DepositAddress = serde_json::from_str(json).unwrap();

        assert_eq!(address.coin, "BTC");
        assert_eq!(address.network, "BTC");
        assert_eq!(address.address, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert_eq!(address.address_with_prefix, None);
        assert_eq!(address.tag, None);
        assert_eq!(address.status, 1);
        assert_eq!(address.wallet_type, 3);
    }

    #[test]
    fn test_get_sub_account_deposit_address_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "coin": "USDT",
                    "network": "TRC20",
                    "address": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
                    "addressWithPrefix": "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s",
                    "tag": "memo123",
                    "status": 0,
                    "walletType": 1
                },
                {
                    "coin": "USDT",
                    "network": "ERC20",
                    "address": "0x742f5b4f5f4d4F3a4B5A3B5c8D8e9F0a1B2c3D4e",
                    "status": 0,
                    "walletType": 1
                }
            ],
            "total": 2
        }"#;

        let response: GetSubAccountDepositAddressResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.total, 2);
        assert_eq!(response.data.len(), 2);

        let first_address = &response.data[0];
        assert_eq!(first_address.coin, "USDT");
        assert_eq!(first_address.network, "TRC20");
        assert_eq!(first_address.address, "TUELLQv4jAQYLnQu7qhLLmv2kfQ6N1nC1s");
        assert_eq!(first_address.tag, Some("memo123".to_string()));

        let second_address = &response.data[1];
        assert_eq!(second_address.coin, "USDT");
        assert_eq!(second_address.network, "ERC20");
        assert_eq!(
            second_address.address,
            "0x742f5b4f5f4d4F3a4B5A3B5c8D8e9F0a1B2c3D4e"
        );
        assert_eq!(second_address.tag, None);
    }

    #[test]
    fn test_deposit_address_status_values() {
        // Test different status values
        let test_cases = vec![(0, "Activated"), (1, "Applied"), (2, "Not applied")];

        for (status_code, _description) in test_cases {
            let json = format!(
                r#"{{
                "coin": "TEST",
                "network": "TEST",
                "address": "test_address",
                "status": {},
                "walletType": 1
            }}"#,
                status_code
            );

            let address: DepositAddress = serde_json::from_str(&json).unwrap();
            assert_eq!(address.status, status_code);
        }
    }

    #[test]
    fn test_deposit_address_wallet_type_values() {
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
                "coin": "TEST",
                "network": "TEST",
                "address": "test_address",
                "status": 0,
                "walletType": {}
            }}"#,
                wallet_type_code
            );

            let address: DepositAddress = serde_json::from_str(&json).unwrap();
            assert_eq!(address.wallet_type, wallet_type_code);
        }
    }

    #[test]
    fn test_get_sub_account_deposit_address_response_empty() {
        let json = r#"{"data": [], "total": 0}"#;
        let response: GetSubAccountDepositAddressResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.total, 0);
        assert_eq!(response.data.len(), 0);
    }
}
