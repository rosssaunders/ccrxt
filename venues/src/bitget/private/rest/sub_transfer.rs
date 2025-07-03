use crate::bitget::{BitgetRestClient, enums::*, error::BitgetError};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Sub Transfer
///
/// The types of transfers supported by this interface include:
/// - Parent account transfer to sub-accounts (only parent account APIKey has access)
/// - Sub-accounts to parent account (only parent account APIKey has access)
/// - Sub-accounts transfer to sub-accounts (only the parent account APIKey has access and the sub-accounts belong to same parent account)
/// - Sub-account inner accounts transfer, e.g. spot to futures (only the parent account APIKey has access, and the fromUserId & toUserId should be same)
///
/// Only the parent account API Key can use this endpoint, and the API Key must bind IP.
///
/// Rate limit: 10 req/sec/UID
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

impl SubTransferRequest {
    pub fn new(
        from_type: AccountType,
        to_type: AccountType,
        amount: impl Into<String>,
        coin: impl Into<String>,
        from_user_id: impl Into<String>,
        to_user_id: impl Into<String>,
    ) -> Self {
        Self {
            from_type,
            to_type,
            amount: amount.into(),
            coin: coin.into(),
            symbol: None,
            client_oid: None,
            from_user_id: from_user_id.into(),
            to_user_id: to_user_id.into(),
        }
    }

    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn client_oid(mut self, client_oid: impl Into<String>) -> Self {
        self.client_oid = Some(client_oid.into());
        self
    }
}

impl BitgetRequest for SubTransferRequest {
    type Response = SubTransferResponse;

    fn path(&self) -> String {
        "/api/v2/spot/wallet/subaccount-transfer".to_string()
    }

    fn method(&self) -> String {
        "POST".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

impl BitgetRestClient {
    /// Sub Transfer
    ///
    /// Transfer funds between parent and sub accounts or between sub accounts.
    pub async fn sub_transfer(
        &self,
        request: SubTransferRequest,
    ) -> Result<SubTransferResponse, BitgetError> {
        self.send_request(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_transfer_request_serialization() {
        let request = SubTransferRequest::new(
            AccountType::Spot,
            AccountType::UsdtFutures,
            "10",
            "USDT",
            "123456",
            "789012",
        )
        .client_oid("my-transfer-123");

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
        let _request = SubTransferRequest::new(
            AccountType::Spot,
            AccountType::UsdtFutures,
            "10",
            "USDT",
            "123456",
            "789012",
        );

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.sub_transfer(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
