use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const GET_TRANSFERABLE_AMOUNTS_ENDPOINT: &str =
    "/openApi/account/transfer/v1/subAccount/transferAsset/supportCoins";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableAmountsRequest {
    /// Sender UID
    pub from_uid: i64,

    /// Sender account type: 1-Fund account; 2-Contract account; 3-Perpetual USD-based account
    pub from_account_type: i64,

    /// Receiver UID
    pub to_uid: i64,

    /// Receiver account type: 1-Fund account; 2-Contract account; 3-Perpetual USD-based account
    pub to_account_type: i64,

    /// Execution window time, cannot exceed 60000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Current timestamp, e.g., 1658748648396
    pub timestamp: i64,
}

/// Individual coin information with transferable amount
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferableCoin {
    /// Coin ID
    pub id: i64,

    /// Coin name, e.g., USDT
    pub name: String,

    /// Available transfer amount
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub available_amount: Option<String>,
}

/// Response for getting transferable amounts
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableAmountsResponse {
    /// List of supported coins
    pub coins: Vec<TransferableCoin>,
}

impl RestClient {
    /// Query the transferable amount of funds
    ///
    /// Query the transferable amount of funds in the parent-child account, The user who
    /// verifies the signature of this API must be main account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/sub-account#Query%20the%20transferable%20amount%20of%20funds%20in%20the%20parent-child%20account%20(only%20for%20parent%20account%20operations).)
    ///
    /// Rate limit: IP rate limit group 1
    ///
    /// # Arguments
    /// * `request` - The get transferable amounts request parameters
    ///
    /// # Returns
    /// A result containing the transferable amounts information or an error
    pub async fn get_transferable_amounts(
        &self,
        request: &GetTransferableAmountsRequest,
    ) -> RestResult<GetTransferableAmountsResponse> {
        self.send_post_signed_request(
            GET_TRANSFERABLE_AMOUNTS_ENDPOINT,
            request,
            EndpointType::Account,
        )
        .await
    }
}

// Serde helper module for handling empty strings as None
mod serde_with {
    pub mod rust {
        pub mod string_empty_as_none {
            use serde::{Deserialize, Deserializer};

            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_transferable_amounts_request_serialization() {
        let request = GetTransferableAmountsRequest {
            from_uid: 123456789,
            from_account_type: 1,
            to_uid: 987654321,
            to_account_type: 3,
            recv_window: Some(60000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"fromUid\":123456789"));
        assert!(json.contains("\"fromAccountType\":1"));
        assert!(json.contains("\"toUid\":987654321"));
        assert!(json.contains("\"toAccountType\":3"));
        assert!(json.contains("\"recvWindow\":60000"));
        assert!(json.contains("\"timestamp\":1658748648396"));
    }

    #[test]
    fn test_get_transferable_amounts_request_serialization_minimal() {
        let request = GetTransferableAmountsRequest {
            from_uid: 111111111,
            from_account_type: 2,
            to_uid: 222222222,
            to_account_type: 1,
            recv_window: None,
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"fromUid\":111111111"));
        assert!(json.contains("\"fromAccountType\":2"));
        assert!(json.contains("\"toUid\":222222222"));
        assert!(json.contains("\"toAccountType\":1"));
        assert!(json.contains("\"timestamp\":1658748648396"));
        assert!(!json.contains("\"recvWindow\""));
    }

    #[test]
    fn test_transferable_coin_deserialization() {
        let json = r#"{
            "id": 1001,
            "name": "USDT",
            "availableAmount": "1000.50000000"
        }"#;

        let coin: TransferableCoin = serde_json::from_str(json).unwrap();

        assert_eq!(coin.id, 1001);
        assert_eq!(coin.name, "USDT");
        assert_eq!(coin.available_amount, Some("1000.50000000".to_string()));
    }

    #[test]
    fn test_transferable_coin_deserialization_empty_amount() {
        let json = r#"{
            "id": 1002,
            "name": "BTC",
            "availableAmount": ""
        }"#;

        let coin: TransferableCoin = serde_json::from_str(json).unwrap();

        assert_eq!(coin.id, 1002);
        assert_eq!(coin.name, "BTC");
        assert_eq!(coin.available_amount, None);
    }

    #[test]
    fn test_get_transferable_amounts_response_deserialization() {
        let json = r#"{
            "coins": [
                {
                    "id": 1001,
                    "name": "USDT",
                    "availableAmount": "1000.50000000"
                },
                {
                    "id": 1002,
                    "name": "BTC",
                    "availableAmount": "0.12345678"
                },
                {
                    "id": 1003,
                    "name": "ETH",
                    "availableAmount": ""
                }
            ]
        }"#;

        let response: GetTransferableAmountsResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.coins.len(), 3);

        let usdt_coin = &response.coins[0];
        assert_eq!(usdt_coin.id, 1001);
        assert_eq!(usdt_coin.name, "USDT");
        assert_eq!(
            usdt_coin.available_amount,
            Some("1000.50000000".to_string())
        );

        let btc_coin = &response.coins[1];
        assert_eq!(btc_coin.id, 1002);
        assert_eq!(btc_coin.name, "BTC");
        assert_eq!(btc_coin.available_amount, Some("0.12345678".to_string()));

        let eth_coin = &response.coins[2];
        assert_eq!(eth_coin.id, 1003);
        assert_eq!(eth_coin.name, "ETH");
        assert_eq!(eth_coin.available_amount, None);
    }

    #[test]
    fn test_get_transferable_amounts_response_empty() {
        let json = r#"{"coins": []}"#;
        let response: GetTransferableAmountsResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.coins.len(), 0);
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
            let request = GetTransferableAmountsRequest {
                from_uid: 123456789,
                from_account_type: account_type_code,
                to_uid: 987654321,
                to_account_type: account_type_code,
                recv_window: None,
                timestamp: 1658748648396,
            };

            let json = serde_json::to_string(&request).unwrap();
            assert!(json.contains(&format!("\"fromAccountType\":{}", account_type_code)));
            assert!(json.contains(&format!("\"toAccountType\":{}", account_type_code)));
        }
    }

    #[test]
    fn test_recv_window_boundary() {
        // Test maximum recv_window value
        let request = GetTransferableAmountsRequest {
            from_uid: 123456789,
            from_account_type: 1,
            to_uid: 987654321,
            to_account_type: 1,
            recv_window: Some(60000), // Maximum allowed value
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"recvWindow\":60000"));
    }
}
