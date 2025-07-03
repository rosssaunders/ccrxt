//! Transfer endpoint for Bitget Spot API
//!
//! This endpoint allows transferring funds between different account types.
//!
//! Reference: https://www.bitget.com/api-doc/spot/wallet/Transfer
//! Endpoint: POST /api/v2/spot/wallet/transfer
//! Rate limit: 10 requests/second/UID

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::RestResult;

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

impl TransferRequest {
    /// Create a new transfer request
    pub fn new(
        coin: impl Into<String>,
        amount: impl Into<String>,
        from_type: AccountType,
        to_type: AccountType,
    ) -> Self {
        Self {
            coin: coin.into(),
            amount: amount.into(),
            from_type,
            to_type,
            client_id: None,
            sub_account_uid: None,
            request_time: None,
            receive_window: None,
        }
    }

    /// Set a client transfer ID for idempotency
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Set sub-account UID (for sub-account transfers)
    pub fn sub_account_uid(mut self, sub_account_uid: impl Into<String>) -> Self {
        self.sub_account_uid = Some(sub_account_uid.into());
        self
    }

    /// Set the request timestamp
    pub fn request_time(mut self, request_time: i64) -> Self {
        self.request_time = Some(request_time);
        self
    }

    /// Set the receive window
    pub fn receive_window(mut self, receive_window: i64) -> Self {
        self.receive_window = Some(receive_window);
        self
    }
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
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            "/api/v2/spot/wallet/transfer",
            reqwest::Method::POST,
            None,        // No query parameters
            Some(&body), // JSON body
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
        let request = TransferRequest::new(
            "USDT",
            "100.50",
            AccountType::Spot,
            AccountType::Margin,
        );

        assert_eq!(request.coin, "USDT");
        assert_eq!(request.amount, "100.50");
        assert_eq!(request.from_type, AccountType::Spot);
        assert_eq!(request.to_type, AccountType::Margin);
        assert!(request.client_id.is_none());
        assert!(request.sub_account_uid.is_none());
    }

    #[test]
    fn test_transfer_request_builder() {
        let request = TransferRequest::new(
            "BTC",
            "0.001",
            AccountType::Main,
            AccountType::Sub,
        )
        .client_id("transfer-123")
        .sub_account_uid("sub_987654321")
        .request_time(1640995200000);

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
        let request = TransferRequest::new(
            "USDT",
            "100.50",
            AccountType::Spot,
            AccountType::Futures,
        )
        .client_id("my-transfer-123");

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
        assert_eq!(
            serde_json::to_string(&AccountType::P2P).unwrap(),
            "\"p2p\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::Main).unwrap(),
            "\"main\""
        );
        assert_eq!(
            serde_json::to_string(&AccountType::Sub).unwrap(),
            "\"sub\""
        );
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
