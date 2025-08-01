use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult, enums::SubAccountTransferType};

const SUB_ACCOUNT_TRANSFER_ENDPOINT: &str = "/openApi/subAccount/v1/subAccountTransfer";

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

impl RestClient {
    /// Execute sub-account transfer
    ///
    /// Transfer funds between master account and sub-account.
    ///
    /// # Arguments
    /// * `request` - The sub-account transfer request
    ///
    /// # Returns
    /// A result containing the sub-account transfer response or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 1/s
    ///
    /// # API Permissions
    /// - SubAccount-Write permission required
    pub async fn sub_account_transfer(
        &self,
        request: &SubAccountTransferRequest,
    ) -> RestResult<SubAccountTransferResponse> {
        self.send_post_signed_request(SUB_ACCOUNT_TRANSFER_ENDPOINT, request, EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_account_transfer_request_serialization() {
        let request = SubAccountTransferRequest {
            from_email: "master@example.com".to_string(),
            to_email: "sub@example.com".to_string(),
            asset: "USDT".to_string(),
            amount: "100.0".to_string(),
            r#type: SubAccountTransferType::ToSub,
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

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
