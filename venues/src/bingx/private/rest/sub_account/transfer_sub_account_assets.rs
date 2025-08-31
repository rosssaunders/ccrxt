use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const TRANSFER_SUB_ACCOUNT_ASSETS_ENDPOINT: &str =
    "/openApi/account/transfer/v1/subAccount/transferAsset";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferSubAccountAssetsRequest {
    /// Name of the asset, e.g., USDT
    pub asset_name: String,

    /// Transfer amount
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub transfer_amount: f64,

    /// Sender UID
    pub from_uid: i64,

    /// Sender sub/master account type: 1-Master account; 2-Sub-account
    pub from_type: i64,

    /// Sender account type: 1-Fund account; 2-Contract account; 3-Perpetual USD-based account
    pub from_account_type: i64,

    /// Receiver UID
    pub to_uid: i64,

    /// Receiver sub/master account type: 1-Master account; 2-Sub-account
    pub to_type: i64,

    /// Receiver account type: 1-Fund account; 2-Contract account; 3-Perpetual USD-based account
    pub to_account_type: i64,

    /// Transfer remark
    pub remark: String,

    /// Execution window time, cannot exceed 60000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Current timestamp, e.g., 1658748648396
    pub timestamp: i64,
}

/// Response for transferring sub-account assets
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferSubAccountAssetsResponse {
    /// Transfer record ID
    pub tran_id: String,
}

impl RestClient {
    /// Sub-Account Asset Transfer Interface (For Master Account Operations Only)
    ///
    /// Sub-Account Asset Transfer Interface, The user who verifies the signature of
    /// this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Sub-Account%20Asset%20Transfer%20Interface%20(For%20Master%20Account%20Operations%20Only))
    ///
    /// Rate limit: IP rate limit group 1
    ///
    /// # Arguments
    /// * `request` - The transfer sub-account assets request parameters
    ///
    /// # Returns
    /// A result containing the transfer record ID or an error
    pub async fn transfer_sub_account_assets(
        &self,
        request: &TransferSubAccountAssetsRequest,
    ) -> RestResult<TransferSubAccountAssetsResponse> {
        self.send_post_signed_request(
            TRANSFER_SUB_ACCOUNT_ASSETS_ENDPOINT,
            request,
            EndpointType::Account,
        )
        .await
    }
}

// Serde helper module for display/fromstr conversion
mod serde_with {
    pub mod rust {
        pub mod display_fromstr {
            use std::fmt::Display;

            use serde::{Serialize, Serializer};

            pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
            where
                T: Display,
                S: Serializer,
            {
                value.to_string().serialize(serializer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_sub_account_assets_request_serialization() {
        let request = TransferSubAccountAssetsRequest {
            asset_name: "USDT".to_string(),
            transfer_amount: 100.5,
            from_uid: 123456789,
            from_type: 1,
            from_account_type: 1,
            to_uid: 987654321,
            to_type: 2,
            to_account_type: 3,
            remark: "Transfer to sub account".to_string(),
            recv_window: Some(60000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"assetName\":\"USDT\""));
        assert!(json.contains("\"transferAmount\":\"100.5\""));
        assert!(json.contains("\"fromUid\":123456789"));
        assert!(json.contains("\"fromType\":1"));
        assert!(json.contains("\"fromAccountType\":1"));
        assert!(json.contains("\"toUid\":987654321"));
        assert!(json.contains("\"toType\":2"));
        assert!(json.contains("\"toAccountType\":3"));
        assert!(json.contains("\"remark\":\"Transfer to sub account\""));
        assert!(json.contains("\"recvWindow\":60000"));
        assert!(json.contains("\"timestamp\":1658748648396"));
    }

    #[test]
    fn test_transfer_sub_account_assets_request_serialization_minimal() {
        let request = TransferSubAccountAssetsRequest {
            asset_name: "BTC".to_string(),
            transfer_amount: 0.001,
            from_uid: 111111111,
            from_type: 2,
            from_account_type: 2,
            to_uid: 222222222,
            to_type: 1,
            to_account_type: 1,
            remark: "BTC transfer".to_string(),
            recv_window: None,
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"assetName\":\"BTC\""));
        assert!(json.contains("\"transferAmount\":\"0.001\""));
        assert!(json.contains("\"fromUid\":111111111"));
        assert!(json.contains("\"fromType\":2"));
        assert!(json.contains("\"fromAccountType\":2"));
        assert!(json.contains("\"toUid\":222222222"));
        assert!(json.contains("\"toType\":1"));
        assert!(json.contains("\"toAccountType\":1"));
        assert!(json.contains("\"remark\":\"BTC transfer\""));
        assert!(json.contains("\"timestamp\":1658748648396"));
        assert!(!json.contains("\"recvWindow\""));
    }

    #[test]
    fn test_transfer_sub_account_assets_response_deserialization() {
        let json = r#"{"tranId":"123456789012345678901234567890"}"#;
        let response: TransferSubAccountAssetsResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.tran_id, "123456789012345678901234567890");
    }

    #[test]
    fn test_account_type_values() {
        // Test different account type values as per documentation
        let test_cases = vec![
            (1, "Fund account"),
            (2, "Contract account"),
            (3, "Perpetual USD-based account"),
        ];

        for (account_type_code, _description) in test_cases {
            let request = TransferSubAccountAssetsRequest {
                asset_name: "TEST".to_string(),
                transfer_amount: 1.0,
                from_uid: 123456789,
                from_type: 1,
                from_account_type: account_type_code,
                to_uid: 987654321,
                to_type: 2,
                to_account_type: account_type_code,
                remark: "test".to_string(),
                recv_window: None,
                timestamp: 1658748648396,
            };

            let json = serde_json::to_string(&request).unwrap();
            assert!(json.contains(&format!("\"fromAccountType\":{}", account_type_code)));
            assert!(json.contains(&format!("\"toAccountType\":{}", account_type_code)));
        }
    }

    #[test]
    fn test_user_type_values() {
        // Test different user type values as per documentation
        let test_cases = vec![(1, "Master account"), (2, "Sub-account")];

        for (user_type_code, _description) in test_cases {
            let request = TransferSubAccountAssetsRequest {
                asset_name: "TEST".to_string(),
                transfer_amount: 1.0,
                from_uid: 123456789,
                from_type: user_type_code,
                from_account_type: 1,
                to_uid: 987654321,
                to_type: user_type_code,
                to_account_type: 1,
                remark: "test".to_string(),
                recv_window: None,
                timestamp: 1658748648396,
            };

            let json = serde_json::to_string(&request).unwrap();
            assert!(json.contains(&format!("\"fromType\":{}", user_type_code)));
            assert!(json.contains(&format!("\"toType\":{}", user_type_code)));
        }
    }

    #[test]
    fn test_transfer_amount_precision() {
        // Test different transfer amount precisions
        let test_cases = vec![100.0, 100.5, 0.001, 0.12345678, 1000000.0];

        for amount in test_cases {
            let request = TransferSubAccountAssetsRequest {
                asset_name: "TEST".to_string(),
                transfer_amount: amount,
                from_uid: 123456789,
                from_type: 1,
                from_account_type: 1,
                to_uid: 987654321,
                to_type: 2,
                to_account_type: 1,
                remark: "test".to_string(),
                recv_window: None,
                timestamp: 1658748648396,
            };

            let json = serde_json::to_string(&request).unwrap();
            let expected_amount_str = format!("\"transferAmount\":\"{}\"", amount);
            assert!(json.contains(&expected_amount_str));
        }
    }

    #[test]
    fn test_recv_window_boundary() {
        // Test maximum recv_window value
        let request = TransferSubAccountAssetsRequest {
            asset_name: "TEST".to_string(),
            transfer_amount: 1.0,
            from_uid: 123456789,
            from_type: 1,
            from_account_type: 1,
            to_uid: 987654321,
            to_type: 2,
            to_account_type: 1,
            remark: "test".to_string(),
            recv_window: Some(60000), // Maximum allowed value
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"recvWindow\":60000"));
    }

    #[test]
    fn test_empty_remark() {
        let request = TransferSubAccountAssetsRequest {
            asset_name: "TEST".to_string(),
            transfer_amount: 1.0,
            from_uid: 123456789,
            from_type: 1,
            from_account_type: 1,
            to_uid: 987654321,
            to_type: 2,
            to_account_type: 1,
            remark: "".to_string(), // Empty remark should be allowed
            recv_window: None,
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"remark\":\"\""));
    }

    #[test]
    fn test_long_remark() {
        let long_remark = "a".repeat(200); // Test with long remark
        let request = TransferSubAccountAssetsRequest {
            asset_name: "TEST".to_string(),
            transfer_amount: 1.0,
            from_uid: 123456789,
            from_type: 1,
            from_account_type: 1,
            to_uid: 987654321,
            to_type: 2,
            to_account_type: 1,
            remark: long_remark.clone(),
            recv_window: None,
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(&format!("\"remark\":\"{}\"", long_remark)));
    }
}
