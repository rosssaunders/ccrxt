use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{RestResult, enums::*};

const TRANSFERABLE_COIN_LIST_ENDPOINT: &str = "/api/v2/spot/wallet/transfer-coin-info";

/// Request parameters for getting transferable coin list.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableCoinListRequest {
    /// Account type to transfer from.
    /// Supported values: spot, p2p, coin_futures, usdt_futures, usdc_futures, crossed_margin, isolated_margin.
    #[serde(rename = "fromType")]
    pub from_type: AccountType,

    /// Account type to transfer to.
    /// Supported values: spot, p2p, coin_futures, usdt_futures, usdc_futures, crossed_margin, isolated_margin.
    #[serde(rename = "toType")]
    pub to_type: AccountType,
}

/// Response from the get transferable coin list endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetTransferableCoinListResponse {
    /// Response code indicating success or failure.
    pub code: String,

    /// Response message providing additional information.
    pub msg: String,

    /// Timestamp when the request was processed (milliseconds since epoch).
    #[serde(rename = "requestTime")]
    pub request_time: u64,

    /// List of coins that can be transferred between the specified account types.
    /// Represents the intersection of transfer_in and transfer_out supported coins.
    pub data: Vec<String>,
}

impl RestClient {
    /// GET Transferable Coin List
    ///
    /// Get transferable coin list.
    ///
    /// [docs]: https://www.bitget.com/api-doc/spot/account/Get-Transfer-Coins
    ///
    /// Rate limit: 10 times/1s (User ID)
    ///
    /// # Arguments
    /// * `request` - The request parameters containing from and to account types
    ///
    /// # Returns
    /// List of coins that can be transferred between the specified account types
    pub async fn get_transferable_coin_list(
        &self,
        request: GetTransferableCoinListRequest,
    ) -> RestResult<GetTransferableCoinListResponse> {
        self.send_get_signed_request(TRANSFERABLE_COIN_LIST_ENDPOINT, request,
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

        assert!(serialized.contains("\"fromType\""));
        assert!(serialized.contains("\"toType\""));
        assert!(serialized.contains("\"spot\""));
        assert!(serialized.contains("\"isolated_margin\""));
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
        assert_eq!(response.msg, "success");
        assert_eq!(response.request_time, 1683875302853);
        assert_eq!(response.data.len(), 3);
        assert!(response.data.contains(&"BTC".to_string()));
        assert!(response.data.contains(&"USDT".to_string()));
        assert!(response.data.contains(&"ETH".to_string()));
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            TRANSFERABLE_COIN_LIST_ENDPOINT,
            "/api/v2/spot/wallet/transfer-coin-info"
        );
    }

    #[test]
    fn test_integration_scenario() {
        // Test that we can create a request, serialize it, and deserialize a response
        let request = GetTransferableCoinListRequest {
            from_type: AccountType::Spot,
            to_type: AccountType::CrossedMargin,
        };

        let serialized_request = serde_json::to_string(&request).unwrap();
        assert!(serialized_request.contains("spot"));
        assert!(serialized_request.contains("crossed_margin"));

        let response_json = r#"
        {
            "code": "00000", 
            "msg": "success",
            "requestTime": 1683875302853,
            "data": ["BTC", "ETH", "USDT"]
        }"#;

        let response: GetTransferableCoinListResponse =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.data.len(), 3);
    }
}
