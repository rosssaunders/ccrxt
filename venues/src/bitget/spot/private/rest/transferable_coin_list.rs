use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{Errors, RestResult, enums::*};

const TRANSFERABLE_COIN_LIST_ENDPOINT: &str = "/api/v2/spot/wallet/transfer-coinlist";

/// Get Transferable Coin List
///
/// Get transferable coin list.
///
/// Frequency limit: 10 times/1s (User ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransferableCoinListRequest {
    /// Account type to transfer from
    #[serde(rename = "fromType")]
    pub from_type: AccountType,

    /// Account type to transfer to
    #[serde(rename = "toType")]
    pub to_type: AccountType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransferableCoinListResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    /// Transfer_in and transfer_out of accounts supports coins intersection
    pub data: Vec<String>,
}

impl RestClient {
    /// Get Transferable Coin List
    ///
    /// Get transferable coin list.
    pub async fn get_transferable_coin_list(
        &self,
        request: GetTransferableCoinListRequest,
    ) -> RestResult<GetTransferableCoinListResponse> {
        self.send_signed_request(
            "/api/v2/spot/wallet/transfer-coin-info",
            reqwest::Method::GET,
            None,
            Some(
                &serde_json::to_string(&request)
                    .map_err(|e| Errors::Error(format!("Serialization error: {e}")))?,
            ),
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
    fn test_get_transferable_coin_list_request_serialization() {
        let request = GetTransferableCoinListRequest {
            from_type: AccountType::Spot,
            to_type: AccountType::IsolatedMargin,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"fromType\""));
        assert!(serialized.contains("\"toType\""));
    }

    #[test]
    fn test_get_transferable_coin_list_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1683875302853,
            "data": [
                "BTC",
                "USDT",
                "ETH"
            ]
        }"#;

        let response: GetTransferableCoinListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.len(), 3);
        assert!(response.data.contains(&"BTC".to_string()));
        assert!(response.data.contains(&"USDT".to_string()));
        assert!(response.data.contains(&"ETH".to_string()));
    }

    #[tokio::test]
    async fn test_get_transferable_coin_list_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetTransferableCoinListRequest {
            from_type: AccountType::Spot,
            to_type: AccountType::IsolatedMargin,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_transferable_coin_list(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
