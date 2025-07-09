use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bybit::{EndpointType, RestResult, TransferType};

/// Request parameters for internal transfer
#[derive(Debug, Clone, Serialize)]
pub struct TransferRequest {
    /// Transfer ID (UUID). Used for idempotency
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    /// Coin to transfer, e.g. BTC, ETH, USDT
    pub coin: String,
    /// Transfer amount
    pub amount: String,
    /// Source account type
    #[serde(rename = "fromAccountType")]
    pub from_account_type: TransferType,
    /// Destination account type  
    #[serde(rename = "toAccountType")]
    pub to_account_type: TransferType,
}

/// Response for transfer endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResponse {
    /// Response code (0 for success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    /// Response message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    /// Extended response information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    /// Response data
    pub result: TransferResult,
    /// Response timestamp
    pub time: u64,
}

/// Result data for transfer response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// Transfer ID
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    /// Transfer status
    pub status: String,
}

impl RestClient {
    /// Internal transfer between accounts
    ///
    /// Transfer funds between different account types (SPOT, CONTRACT, UNIFIED, etc.)
    /// within the same user account.
    ///
    /// # Arguments
    /// * `request` - The transfer request parameters
    ///
    /// # Rate Limit
    /// 1 request per second
    ///
    /// # Returns
    /// A result containing the transfer response or an error
    pub async fn internal_transfer(&self, request: TransferRequest) -> RestResult<TransferResponse> {
        self.send_signed_request(
            "/v5/asset/transfer/inter-transfer",
            reqwest::Method::POST,
            request,
            EndpointType::Asset,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_request_structure() {
        let request = TransferRequest {
            transfer_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            coin: "USDT".to_string(),
            amount: "100.5".to_string(),
            from_account_type: TransferType::Spot,
            to_account_type: TransferType::Unified,
        };
        
        assert_eq!(request.coin, "USDT");
        assert_eq!(request.amount, "100.5");
        assert_eq!(request.from_account_type, TransferType::Spot);
        assert_eq!(request.to_account_type, TransferType::Unified);
    }

    #[test]
    fn test_transfer_request_serialization() {
        let request = TransferRequest {
            transfer_id: "test-transfer-123".to_string(),
            coin: "BTC".to_string(),
            amount: "0.001".to_string(),
            from_account_type: TransferType::Contract,
            to_account_type: TransferType::Spot,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"transferId\":\"test-transfer-123\""));
        assert!(serialized.contains("\"coin\":\"BTC\""));
        assert!(serialized.contains("\"amount\":\"0.001\""));
        assert!(serialized.contains("\"fromAccountType\":\"CONTRACT\""));
        assert!(serialized.contains("\"toAccountType\":\"SPOT\""));
    }

    #[test]
    fn test_transfer_response_structure() {
        let response_json = r#"
        {
            "retCode": 0,
            "retMsg": "success",
            "retExtInfo": {},
            "result": {
                "transferId": "550e8400-e29b-41d4-a716-446655440000",
                "status": "SUCCESS"
            },
            "time": 1672734174346
        }
        "#;

        let response: TransferResponse = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.ret_code, 0);
        assert_eq!(response.ret_msg, "success");
        assert_eq!(response.result.transfer_id, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(response.result.status, "SUCCESS");
        assert_eq!(response.time, 1672734174346);
    }

    #[test]
    fn test_all_transfer_types() {
        // Test that all transfer types serialize correctly
        let types = vec![
            TransferType::Spot,
            TransferType::Contract,
            TransferType::Unified,
            TransferType::Option,
            TransferType::Fund,
        ];

        for transfer_type in types {
            let serialized = serde_json::to_string(&transfer_type).unwrap();
            // Verify it can be serialized without error
            assert!(!serialized.is_empty());
        }
    }
}