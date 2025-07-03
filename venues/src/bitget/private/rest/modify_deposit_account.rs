use crate::bitget::{BitgetRestClient, enums::*, error::BitgetError};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Modify Deposit Account
///
/// Modify the auto-transfer account type of deposit.
///
/// Frequency limit: 10 times/1s (User ID)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyDepositAccountRequest {
    /// Currency of transfer
    pub coin: String,

    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: DepositAccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyDepositAccountResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    /// Success/fail status
    pub data: String,
}

impl ModifyDepositAccountRequest {
    pub fn new(coin: impl Into<String>, account_type: DepositAccountType) -> Self {
        Self {
            coin: coin.into(),
            account_type,
        }
    }
}

impl BitgetRequest for ModifyDepositAccountRequest {
    type Response = ModifyDepositAccountResponse;

    fn path(&self) -> String {
        "/api/v2/spot/wallet/modify-deposit-account".to_string()
    }

    fn method(&self) -> String {
        "POST".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

impl BitgetRestClient {
    /// Modify Deposit Account
    ///
    /// Modify the auto-transfer account type of deposit.
    pub async fn modify_deposit_account(
        &self,
        request: ModifyDepositAccountRequest,
    ) -> Result<ModifyDepositAccountResponse, BitgetError> {
        self.send_request(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_deposit_account_request_serialization() {
        let request = ModifyDepositAccountRequest::new("USDT", DepositAccountType::UsdtFutures);

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"coin\":\"USDT\""));
        assert!(serialized.contains("\"accountType\""));
    }

    #[test]
    fn test_modify_deposit_account_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1683875302853,
            "data": "success"
        }"#;

        let response: ModifyDepositAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data, "success");
    }

    #[tokio::test]
    async fn test_modify_deposit_account_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = ModifyDepositAccountRequest::new("USDT", DepositAccountType::UsdtFutures);

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.modify_deposit_account(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
