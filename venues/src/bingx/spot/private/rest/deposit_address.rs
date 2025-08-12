use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{AddressStatus, EndpointType, RestResult};

const DEPOSIT_ADDRESS_ENDPOINT: &str = "/openApi/wallets/v1/capital/deposit/address";

/// Request for getting main account deposit address
#[derive(Debug, Clone, Serialize)]
pub struct GetDepositAddressRequest {
    /// Name of the coin for transfer
    pub coin: String,
    /// Starting record number (optional, default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Page size (optional, default 100, max 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Execution window time, cannot be greater than 60000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
    /// Request timestamp in milliseconds
    pub timestamp: i64,
}

/// Deposit address information
#[derive(Debug, Clone, Deserialize)]
pub struct DepositAddress {
    /// Coin name
    pub coin: String,
    /// Network name
    pub network: String,
    /// Deposit address
    pub address: String,
    /// Deposit address with prefix
    #[serde(rename = "addressWithPrefix")]
    pub address_with_prefix: String,
    /// Address tag
    pub tag: Option<String>,
    /// Status: 0 for activated, 1 for applied, 2 for not applied
    pub status: AddressStatus,
}

/// Response for main account deposit address
#[derive(Debug, Clone, Deserialize)]
pub struct GetDepositAddressResponse {
    /// List of deposit addresses
    pub data: Vec<DepositAddress>,
    /// Total number of addresses
    pub total: i32,
}

impl RestClient {
    /// Get main account deposit address
    ///
    /// This endpoint is used for a mother account to query the deposit address
    /// of a specific coin in the blockchain it belongs to. Only available for mother accounts.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/wallet-api.html#Main%20Account%20Deposit%20Address)
    ///
    /// # Arguments
    /// * `request` - The deposit address request parameters
    ///
    /// # Returns
    /// A result containing the deposit address response or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 2/s
    /// - IP rate limit group 2
    ///
    /// # API Permissions
    /// - Read permission required
    pub async fn get_deposit_address(
        &self,
        request: &GetDepositAddressRequest,
    ) -> RestResult<GetDepositAddressResponse> {
        self.send_get_signed_request(
            DEPOSIT_ADDRESS_ENDPOINT,
            request,
            EndpointType::AccountApiGroup2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_address_request_serialization() {
        let request = GetDepositAddressRequest {
            coin: "BTC".to_string(),
            offset: Some(0),
            limit: Some(100),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("coin=BTC"));
        assert!(serialized.contains("offset=0"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_deposit_address_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "coin": "BTC",
                    "network": "BTC",
                    "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                    "addressWithPrefix": "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                    "tag": "",
                    "status": 0
                }
            ],
            "total": 1
        }"#;

        let response: GetDepositAddressResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total, 1);
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].coin, "BTC");
        assert_eq!(response.data[0].status, AddressStatus::Activated);
    }
}
