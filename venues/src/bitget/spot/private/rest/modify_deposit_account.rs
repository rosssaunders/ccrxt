use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{RestResult, enums::*};

/// Modify Deposit Account
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

impl RestClient {
    /// Modify Deposit Account
    ///
    /// Modify the auto-transfer account type of deposit.
    ///
    /// [API Documentation](https://www.bitget.com/api-doc/spot/asset/Modify-Deposit-Account)
    ///
    /// Frequency limit: 10 times/1s (User ID)
    ///
    /// Returns a `RestResult<ModifyDepositAccountResponse>` containing the result or an error.
    pub async fn modify_deposit_account(
        &self,
        params: ModifyDepositAccountRequest,
    ) -> RestResult<ModifyDepositAccountResponse> {
        let endpoint = "/api/v2/spot/wallet/modify-deposit-account";
        self.send_post_signed_request(endpoint, params, 10, false, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_deposit_account_request_serialization() {
        let request = ModifyDepositAccountRequest {
            coin: "USDT".to_string(),
            account_type: DepositAccountType::UsdtFutures,
        };

        let serialized = serde_json::to_string(&request).unwrap();

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
        let _request = ModifyDepositAccountRequest {
            coin: "USDT".to_string(),
            account_type: DepositAccountType::UsdtFutures,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.modify_deposit_account(request).await.unwrap();
        // log::info!("Response: {:?}", response);
    }
}
