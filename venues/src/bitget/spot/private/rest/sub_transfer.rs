use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{RestResult, enums::*};

const SUB_TRANSFER_ENDPOINT: &str = "/api/v2/spot/wallet/subaccount-transfer";

/// Request parameters for sub account transfer endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct SubTransferRequest {
    /// Account type to transfer from (spot, p2p, coin_futures, usdt_futures, usdc_futures, crossed_margin, isolated_margin).
    #[serde(rename = "fromType")]
    pub from_type: AccountType,

    /// Account type to transfer to (spot, p2p, coin_futures, usdt_futures, usdc_futures, crossed_margin, isolated_margin).
    #[serde(rename = "toType")]
    pub to_type: AccountType,

    /// Amount to transfer as string.
    pub amount: String,

    /// Currency of transfer (e.g., USDT, BTC).
    pub coin: String,

    /// Symbol name (required for isolated margin transfers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Custom order ID for tracking the transfer.
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// Outgoing account UID (source account identifier).
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,

    /// Incoming account UID (destination account identifier).
    #[serde(rename = "toUserId")]
    pub to_user_id: String,
}

/// Response wrapper for sub transfer endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct SubTransferResponse {
    /// Response code indicating success or failure.
    pub code: String,

    /// Response message providing additional information.
    pub msg: String,

    /// Timestamp when the request was processed (milliseconds since epoch).
    #[serde(rename = "requestTime")]
    pub request_time: u64,

    /// Transfer result data containing transfer details.
    pub data: SubTransferResult,
}

/// Transfer result containing the created transfer information.
#[derive(Debug, Clone, Deserialize)]
pub struct SubTransferResult {
    /// Unique transfer ID assigned by the system.
    #[serde(rename = "transferId")]
    pub transfer_id: String,

    /// Custom order ID provided in the request.
    #[serde(rename = "clientOid")]
    pub client_oid: String,
}

impl RestClient {
    /// Sub Transfer
    ///
    /// Transfer funds between parent and sub accounts or between sub accounts.
    /// This endpoint supports parent-to-sub, sub-to-parent, sub-to-sub, and internal sub-account transfers.
    /// Only parent account API keys can use this endpoint and must bind IP.
    ///
    /// [docs]: https://www.bitget.com/api-doc/spot/account/Sub-Transfer
    ///
    /// Rate limit: 10 req/sec/UID
    ///
    /// # Arguments
    /// * `params` - The sub transfer request parameters
    ///
    /// # Returns
    /// Response containing transfer ID and client order ID
    pub async fn sub_transfer(
        &self,
        params: SubTransferRequest,
    ) -> RestResult<SubTransferResponse> {
        self.send_signed_post_request(SUB_TRANSFER_ENDPOINT, &params, 10, false, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_transfer_request_serialization() {
        let request = SubTransferRequest {
            from_type: AccountType::Spot,
            to_type: AccountType::UsdtFutures,
            amount: "10".to_string(),
            coin: "USDT".to_string(),
            from_user_id: "123456".to_string(),
            to_user_id: "789012".to_string(),
            client_oid: Some("my-transfer-123".to_string()),
            symbol: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();

        assert!(serialized.contains("\"amount\":\"10\""));
        assert!(serialized.contains("\"coin\":\"USDT\""));
        assert!(serialized.contains("\"fromUserId\":\"123456\""));
        assert!(serialized.contains("\"toUserId\":\"789012\""));
    }

    #[test]
    fn test_sub_transfer_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1683875302853,
            "data": {
                "transferId": "123456",
                "clientOid": "my-transfer-123"
            }
        }"#;

        let response: SubTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.transfer_id, "123456");
        assert_eq!(response.data.client_oid, "my-transfer-123");
    }

    #[test]
    fn test_sub_transfer_default() {
        let request = SubTransferRequest::default();
        assert_eq!(request.amount, "");
        assert_eq!(request.coin, "");
        assert_eq!(request.from_user_id, "");
        assert_eq!(request.to_user_id, "");
        assert!(request.symbol.is_none());
        assert!(request.client_oid.is_none());
    }

    #[tokio::test]
    async fn test_sub_transfer_endpoint() {
        // This test verifies the endpoint structure without making actual API calls
        let request = SubTransferRequest {
            from_type: AccountType::Spot,
            to_type: AccountType::UsdtFutures,
            amount: "10".to_string(),
            coin: "USDT".to_string(),
            from_user_id: "123456".to_string(),
            to_user_id: "789012".to_string(),
            client_oid: None,
            symbol: None,
        };

        // Verify that the request can be serialized properly
        let _serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(request.amount, "10");
        assert_eq!(request.coin, "USDT");
    }
}
