use crate::bingx::enums::SubAccountTransferType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransferRequest {
    /// From email (master account email for master->sub, sub account email for sub->master)
    pub from_email: String,
    /// To email (sub account email for master->sub, master account email for sub->master)
    pub to_email: String,
    /// Asset symbol
    pub asset: String,
    /// Amount to transfer
    pub amount: String,
    /// Transfer type
    pub r#type: SubAccountTransferType,
    /// Timestamp in ms
    pub recv_window: Option<i64>,
    /// Timestamp in ms
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransferResponse {
    /// Transaction ID
    pub txn_id: String,
    /// Whether transfer was successful
    pub success: bool,
}

impl SubAccountTransferRequest {
    pub fn new(
        from_email: String,
        to_email: String,
        asset: String,
        amount: String,
        transfer_type: SubAccountTransferType,
        timestamp: i64,
    ) -> Self {
        Self {
            from_email,
            to_email,
            asset,
            amount,
            r#type: transfer_type,
            recv_window: None,
            timestamp,
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_account_transfer_request_serialization() {
        let request = SubAccountTransferRequest::new(
            "master@example.com".to_string(),
            "sub@example.com".to_string(),
            "USDT".to_string(),
            "100.0".to_string(),
            SubAccountTransferType::ToSub,
            1640995200000,
        )
        .recv_window(5000);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"fromEmail\":\"master@example.com\""));
        assert!(json.contains("\"toEmail\":\"sub@example.com\""));
        assert!(json.contains("\"asset\":\"USDT\""));
        assert!(json.contains("\"amount\":\"100.0\""));
        assert!(json.contains("\"type\":\"1\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1640995200000"));
    }

    #[test]
    fn test_sub_account_transfer_response_deserialization() {
        let json = r#"
        {
            "txnId": "TXN123456789",
            "success": true
        }
        "#;

        let response: SubAccountTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.txn_id, "TXN123456789");
        assert!(response.success);
    }
}
