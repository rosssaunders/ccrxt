use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::RestResult;

const SUBACCOUNT_ASSETS_ENDPOINT: &str = "/api/v2/spot/account/subaccount-assets";

/// Get Sub-accounts Assets
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

impl RestClient {
    /// Get Sub-accounts Assets
    ///
    /// Get Sub-accounts Assets (only return the sub-accounts which assets > 0).
    /// ND Brokers are not allowed to call this endpoint.
    ///
    /// Frequency limit: 10 times/1s (User ID)
    pub async fn get_subaccount_assets(
        &self,
        request: GetSubaccountAssetsRequest,
    ) -> RestResult<GetSubaccountAssetsResponse> {
        self.send_get_signed_request(SUBACCOUNT_ASSETS_ENDPOINT, request,
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
    fn test_get_subaccount_assets_request_serialization() {
        let request = GetSubaccountAssetsRequest {
            limit: Some("20".to_string()),
            ..Default::default()
        };

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
        let _request = GetSubaccountAssetsRequest {
            limit: Some("10".to_string()),
            ..Default::default()
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_subaccount_assets(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
