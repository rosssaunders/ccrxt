use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{Errors, RestResult, enums::*};

/// Sub Transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTransferRequest {
    /// Account type to transfer from
    #[serde(rename = "fromType")]
    pub from_type: AccountType,

    /// Account type to transfer to
    #[serde(rename = "toType")]
    pub to_type: AccountType,

    /// Amount to transfer
    pub amount: String,

    /// Currency of transfer
    pub coin: String,

    /// Symbol name (Required in Isolated margin (spot) transferring)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Custom order ID
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// Outgoing Account UID
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,

    /// Incoming Account UID
    #[serde(rename = "toUserId")]
    pub to_user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTransferResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: SubTransferResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTransferResult {
    /// Transfer ID
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    /// Custom order ID
    #[serde(rename = "clientOid")]
    pub client_oid: String,
}

impl RestClient {
    /// Sub Transfer
    ///
    /// Transfer funds between parent and sub accounts or between sub accounts.
    ///
    /// [docs]: https://www.bitget.com/api-doc/spot/wallet/Transfer-to-Subaccount
    ///
    /// Rate limit: 10 req/sec/UID
    ///
    /// Returns a `RestResult<SubTransferResponse>` containing the transfer result or an error.
    pub async fn sub_transfer(
        &self,
        params: SubTransferRequest,
    ) -> RestResult<SubTransferResponse> {
        let endpoint = "/api/v2/spot/wallet/subaccount-transfer";
        let body = serde_json::to_string(&params)
            .map_err(|e| Errors::Error(format!("Serialization error: {e}")))?;
        self.send_signed_request::<SubTransferResponse>(
            endpoint,
            reqwest::Method::POST,
            None,
            Some(&body),
            10,
            false,
            None,
        )
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
        println!("Serialized request: {}", serialized);

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

    #[tokio::test]
    async fn test_sub_transfer_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = SubTransferRequest {
            from_type: AccountType::Spot,
            to_type: AccountType::UsdtFutures,
            amount: "10".to_string(),
            coin: "USDT".to_string(),
            from_user_id: "123456".to_string(),
            to_user_id: "789012".to_string(),
            client_oid: None,
            symbol: None,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.sub_transfer(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
