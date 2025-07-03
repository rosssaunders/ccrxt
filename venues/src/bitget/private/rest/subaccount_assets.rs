use crate::bitget::{BitgetRestClient, enums::*, error::BitgetError};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Get Sub-accounts Assets
///
/// Get Sub-accounts Assets (only return the sub-accounts which assets > 0).
/// ND Brokers are not allowed to call this endpoint.
///
/// Frequency limit: 10 times/1s (User ID)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubaccountAssetsRequest {
    /// Cursor ID
    /// Pagination cursor. Do not pass it in the first request.
    /// For subsequent requests, pass the last ID returned previously.
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,

    /// The number of sub-accounts returned per page.
    /// The default value is 10, and the maximum value is 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubaccountAssetsResponse {
    pub code: String,
    pub message: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: Vec<SubaccountAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountAsset {
    /// Cursor ID
    pub id: String,
    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// List of spot assets
    #[serde(rename = "assetsList")]
    pub assets_list: Vec<AssetDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDetail {
    /// Token name
    pub coin: String,
    /// Available assets
    pub available: String,
    /// Restricted availability (For spot copy trading)
    #[serde(rename = "limitAvailable")]
    pub limit_available: String,
    /// Assets frozen
    pub frozen: String,
    /// Assets locked
    pub locked: String,
    /// Update time, Unix, ms
    #[serde(rename = "uTime")]
    pub u_time: String,
}

impl GetSubaccountAssetsRequest {
    pub fn new() -> Self {
        Self {
            id_less_than: None,
            limit: None,
        }
    }

    pub fn id_less_than(mut self, id_less_than: impl Into<String>) -> Self {
        self.id_less_than = Some(id_less_than.into());
        self
    }

    pub fn limit(mut self, limit: impl Into<String>) -> Self {
        self.limit = Some(limit.into());
        self
    }
}

impl Default for GetSubaccountAssetsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl BitgetRequest for GetSubaccountAssetsRequest {
    type Response = GetSubaccountAssetsResponse;

    fn path(&self) -> String {
        "/api/v2/spot/account/subaccount-assets".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

impl BitgetRestClient {
    /// Get Sub-accounts Assets
    ///
    /// Get Sub-accounts Assets (only return the sub-accounts which assets > 0).
    /// ND Brokers are not allowed to call this endpoint.
    pub async fn get_subaccount_assets(
        &self,
        request: GetSubaccountAssetsRequest,
    ) -> Result<GetSubaccountAssetsResponse, BitgetError> {
        self.send_request(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subaccount_assets_request_serialization() {
        let request = GetSubaccountAssetsRequest::new().limit("20");

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"limit\":\"20\""));
    }

    #[test]
    fn test_get_subaccount_assets_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "message": "success",
            "requestTime": 1695808949356,
            "data": [
                {
                    "id": "1111",
                    "userId": "1234567890",
                    "assetsList": [
                        {
                            "coin": "BTC",
                            "available": "1.1",
                            "limitAvailable": "12.1",
                            "frozen": "0",
                            "locked": "1.1",
                            "uTime": "1337654897651"
                        }
                    ]
                }
            ]
        }"#;

        let response: GetSubaccountAssetsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].assets_list[0].coin, "BTC");
    }

    #[tokio::test]
    async fn test_get_subaccount_assets_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetSubaccountAssetsRequest::new().limit("10");

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_subaccount_assets(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
