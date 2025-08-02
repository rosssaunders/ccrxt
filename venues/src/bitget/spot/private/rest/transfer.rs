use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::spot::RestResult;

/// Endpoint for transferring funds
const TRANSFER_ENDPOINT: &str = "/api/v2/spot/wallet/transfer";

/// Account type for transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    /// Spot trading account
    Spot,
    /// Margin trading account
    Margin,
    /// Futures trading account
    Futures,
    /// P2P trading account
    P2P,
    /// Savings account
    Savings,
    /// Main account (for sub-account transfers)
    Main,
    /// Sub account (for sub-account transfers)
    Sub,
}

/// Request parameters for internal transfer
#[derive(Debug, Clone, Serialize)]
pub struct TransferRequest {
    /// Currency to transfer, e.g. USDT
    pub coin: String,

    /// Transfer amount
    pub amount: String,

    /// Source account type
    #[serde(rename = "fromType")]
    pub from_type: AccountType,

    /// Destination account type
    #[serde(rename = "toType")]
    pub to_type: AccountType,

    /// Client transfer ID (optional, for idempotency)
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// Sub account UID (required for sub-account transfers)
    #[serde(rename = "subAccountUid", skip_serializing_if = "Option::is_none")]
    pub sub_account_uid: Option<String>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Transfer status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransferStatus {
    /// Transfer successful
    Success,
    /// Transfer pending
    Pending,
    /// Transfer failed
    Failed,
}

/// Response from transfer request
#[derive(Debug, Clone, Deserialize)]
pub struct TransferResponse {
    /// Transfer ID
    #[serde(rename = "transferId")]
    pub transfer_id: String,

    /// Client transfer ID (if provided)
    #[serde(rename = "clientOid")]
    pub client_id: Option<String>,

    /// Transfer status
    pub status: TransferStatus,

    /// Transfer timestamp (Unix milliseconds)
    #[serde(rename = "transTime")]
    pub transfer_time: i64,
}

impl RestClient {
    /// Transfer funds between accounts
    ///
    /// Transfers funds between different account types (spot, margin, futures, etc.)
    /// or between main and sub-accounts.
    ///
    /// # Arguments
    /// * `request` - The transfer request parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the transfer response or an error
    pub async fn transfer(&self, request: TransferRequest) -> RestResult<TransferResponse> {
        self.send_signed_post_request(
            TRANSFER_ENDPOINT,
            &request,
            10,          // 10 requests per second rate limit
            false,       // This is not an order placement endpoint
            None,        // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_request_new() {
        let request = TransferRequest {
            coin: "USDT".to_string(),
            amount: "100.50".to_string(),
            from_type: AccountType::Spot,
            to_type: AccountType::Margin,
            client_id: None,
            sub_account_uid: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.coin, "USDT");
        assert_eq!(request.amount, "100.50");
        assert_eq!(request.from_type, AccountType::Spot);
        assert_eq!(request.to_type, AccountType::Margin);
        assert!(request.client_id.is_none());
        assert!(request.sub_account_uid.is_none());
    }

    #[test]
    fn test_transfer_request_builder() {
        let request = TransferRequest {
            coin: "BTC".to_string(),
            amount: "0.001".to_string(),
            from_type: AccountType::Main,
            to_type: AccountType::Sub,
            client_id: Some("transfer-123".to_string()),
            sub_account_uid: Some("sub_987654321".to_string()),
            request_time: Some(1640995200000),
            receive_window: None,
        };

        assert_eq!(request.coin, "BTC");
        assert_eq!(request.amount, "0.001");
        assert_eq!(request.from_type, AccountType::Main);
        assert_eq!(request.to_type, AccountType::Sub);
        assert_eq!(request.client_id, Some("transfer-123".to_string()));
        assert_eq!(request.sub_account_uid, Some("sub_987654321".to_string()));
        assert_eq!(request.request_time, Some(1640995200000));
    }

    #[test]
    fn test_transfer_request_serialization() {
        let request = TransferRequest {
            coin: "USDT".to_string(),
            amount: "100.50".to_string(),
            from_type: AccountType::Spot,
            to_type: AccountType::Futures,
            client_id: Some("my-transfer-123".to_string()),
            sub_account_uid: None,
            request_time: None,
            receive_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"amount\":\"100.50\""));
        assert!(json.contains("\"fromType\":\"spot\""));
        assert!(json.contains("\"toType\":\"futures\""));
        assert!(json.contains("\"clientOid\":\"my-transfer-123\""));
    }

    #[test]
    fn test_account_type_serialization() {
        assert_eq!(
            serde_json::to_string(&AccountType::Spot).unwrap(),
            "\"spot\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::Margin).unwrap(),
            "\"margin\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::Futures).unwrap(),
            "\"futures\""
        );
        assert_eq!(serde_json::to_string(&AccountType::P2P).unwrap(), "\"p2p\"");
        assert_eq!(
            serde_json::to_string(&AccountType::Main).unwrap(),
            "\"main\""
        );
        assert_eq!(serde_json::to_string(&AccountType::Sub).unwrap(), "\"sub\"");
    }

    #[test]
    fn test_transfer_status_deserialization() {
        assert_eq!(
            serde_json::from_str::<TransferStatus>("\"success\"").unwrap(),
            TransferStatus::Success
        );
        assert_eq!(
            serde_json::from_str::<TransferStatus>("\"pending\"").unwrap(),
            TransferStatus::Pending
        );
        assert_eq!(
            serde_json::from_str::<TransferStatus>("\"failed\"").unwrap(),
            TransferStatus::Failed
        );
    }

    #[test]
    fn test_transfer_response_deserialization() {
        let json = r#"{
            "transferId": "transfer_123456789",
            "clientOid": "my-transfer-123",
            "status": "success",
            "transTime": 1640995200000
        }"#;

        let response: TransferResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.transfer_id, "transfer_123456789");
        assert_eq!(response.client_id, Some("my-transfer-123".to_string()));
        assert_eq!(response.status, TransferStatus::Success);
        assert_eq!(response.transfer_time, 1640995200000);
    }

    #[test]
    fn test_transfer_response_deserialization_no_client_id() {
        let json = r#"{
            "transferId": "transfer_123456789",
            "clientOid": null,
            "status": "pending",
            "transTime": 1640995200000
        }"#;

        let response: TransferResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.transfer_id, "transfer_123456789");
        assert!(response.client_id.is_none());
        assert_eq!(response.status, TransferStatus::Pending);
        assert_eq!(response.transfer_time, 1640995200000);
    }
}
