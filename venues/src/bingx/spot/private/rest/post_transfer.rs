use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const ASSET_TRANSFER_NEW_ENDPOINT: &str = "/openApi/api/asset/v1/transfer";

/// Request to create a new asset transfer between accounts
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetTransferNewRequest {
    /// From account: fund=Funding Account, spot=Spot Account, stdFutures=Standard Contract, coinMPerp=COIN-M Perpetual Future, USDTMPerp=Perpetual Future
    pub from_account: String,

    /// To account: fund=Funding Account, spot=Spot Account, stdFutures=Standard Contract, coinMPerp=COIN-M Perpetual Future, USDTMPerp=Perpetual Future
    pub to_account: String,

    /// Coin name e.g. USDT
    pub asset: String,

    /// Transfer amount
    pub amount: Decimal,

    /// Execution window time, cannot be greater than 60000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Current timestamp e.g. 1658748648396
    pub timestamp: i64,
}

/// Response from the new asset transfer endpoint
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetTransferNewResponse {
    /// Transfer ID
    pub transfer_id: String,
}

impl RestClient {
    /// Create a new asset transfer between accounts
    ///
    /// Transfer assets between different account types within the same user account.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/common/account-api.html#Asset%20Transfer%20New)
    ///
    /// Rate limit: 2/s by UID & 2 by IP in group
    ///
    /// # Arguments
    /// * `request` - The asset transfer request
    ///
    /// # Returns
    /// A result containing the transfer response or an error
    pub async fn asset_transfer_new(
        &self,
        request: &AssetTransferNewRequest,
    ) -> RestResult<AssetTransferNewResponse> {
        self.send_post_signed_request(ASSET_TRANSFER_NEW_ENDPOINT, request, EndpointType::Trading)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_asset_transfer_new_request_serialization() {
        let request = AssetTransferNewRequest {
            from_account: "fund".to_string(),
            to_account: "spot".to_string(),
            asset: "USDT".to_string(),
            amount: dec!(100.5),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"fromAccount\":\"fund\""));
        assert!(json.contains("\"toAccount\":\"spot\""));
        assert!(json.contains("\"asset\":\"USDT\""));
        assert!(json.contains("\"amount\":\"100.5\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1658748648396"));
    }

    #[test]
    fn test_asset_transfer_new_response_deserialization() {
        let json = r#"
        {
            "transferId": "TRANSFER123456789"
        }
        "#;

        let response: AssetTransferNewResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transfer_id, "TRANSFER123456789");
    }

    #[test]
    fn test_minimal_request() {
        let request = AssetTransferNewRequest {
            from_account: "spot".to_string(),
            to_account: "USDTMPerp".to_string(),
            asset: "BTC".to_string(),
            amount: dec!(0.001),
            recv_window: None,
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"fromAccount\":\"spot\""));
        assert!(json.contains("\"toAccount\":\"USDTMPerp\""));
        assert!(json.contains("\"asset\":\"BTC\""));
        assert!(json.contains("\"amount\":\"0.001\""));
        assert!(!json.contains("recvWindow"));
        assert!(json.contains("\"timestamp\":1658748648396"));
    }

    #[test]
    fn test_futures_transfer() {
        let request = AssetTransferNewRequest {
            from_account: "stdFutures".to_string(),
            to_account: "coinMPerp".to_string(),
            asset: "ETH".to_string(),
            amount: dec!(1.5),
            recv_window: Some(60000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"fromAccount\":\"stdFutures\""));
        assert!(json.contains("\"toAccount\":\"coinMPerp\""));
        assert!(json.contains("\"recvWindow\":60000"));
    }
}
