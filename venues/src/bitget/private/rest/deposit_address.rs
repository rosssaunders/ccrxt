use crate::bitget::{BitgetRestClient, enums::*, error::BitgetError};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Get Deposit Address
///
/// Get Deposit Address.
///
/// Frequency limit: 10 times/1s (User ID)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositAddressRequest {
    /// Coin name, e.g. USDT
    /// All coin names can be returned from Get Coin Info interface
    pub coin: String,

    /// Chain name, e.g. trc20
    /// You can get the chain names via Get Coin Info interface
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,

    /// Bitcoin Lightning Network withdrawal amount, limit: 0.000001 - 0.01
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositAddressResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: DepositAddressInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositAddressInfo {
    /// Deposit address
    pub address: String,
    /// Chain name
    pub chain: String,
    /// Token name
    pub coin: String,
    /// Tag
    pub tag: String,
    /// Blockchain address
    pub url: String,
}

impl GetDepositAddressRequest {
    pub fn new(coin: impl Into<String>) -> Self {
        Self {
            coin: coin.into(),
            chain: None,
            size: None,
        }
    }

    pub fn chain(mut self, chain: impl Into<String>) -> Self {
        self.chain = Some(chain.into());
        self
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl BitgetRequest for GetDepositAddressRequest {
    type Response = GetDepositAddressResponse;

    fn path(&self) -> String {
        "/api/v2/spot/wallet/deposit-address".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

impl BitgetRestClient {
    /// Get Deposit Address
    ///
    /// Get Deposit Address.
    pub async fn get_deposit_address(
        &self,
        request: GetDepositAddressRequest,
    ) -> Result<GetDepositAddressResponse, BitgetError> {
        self.send_request(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_deposit_address_request_serialization() {
        let request = GetDepositAddressRequest::new("USDT").chain("trc20");

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"coin\":\"USDT\""));
        assert!(serialized.contains("\"chain\":\"trc20\""));
    }

    #[test]
    fn test_get_deposit_address_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1683875302853,
            "data": {
                "address": "TJRyWwFs9wTFGZg3JbrVriFbNfCug5tDeC",
                "chain": "TRC20",
                "coin": "USDT",
                "tag": "",
                "url": "https://tronscan.org/#/address/TJRyWwFs9wTFGZg3JbrVriFbNfCug5tDeC"
            }
        }"#;

        let response: GetDepositAddressResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.coin, "USDT");
        assert_eq!(response.data.chain, "TRC20");
        assert!(!response.data.address.is_empty());
    }

    #[tokio::test]
    async fn test_get_deposit_address_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetDepositAddressRequest::new("USDT").chain("trc20");

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_deposit_address(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
