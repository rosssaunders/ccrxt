use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const MARGIN_ASSET_TRANSFER_ENDPOINT: &str = "/spot/v1/margin/isolated/transfer";

/// Request parameters for margin asset transfer
#[derive(Debug, Serialize, Default)]
pub struct MarginAssetTransferRequest {
    /// Trading pair (e.g. BMX_USDT)
    pub symbol: String,
    /// Currency
    pub currency: String,
    /// Amount of transfers (precision: 8 decimal places)
    pub amount: String,
    /// Transfer direction
    /// - `in` = Transfer in
    /// - `out` = Transfer out
    pub side: String,
}

/// Response for margin asset transfer endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAssetTransferResponse {
    /// Transfer order id, only successful transfers will be returned
    pub transfer_id: String,
}

impl RestClient {
    /// Margin Asset Transfer
    ///
    /// For fund transfers between a margin account and spot account
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/spot/#margin-asset-transfer-signed
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Margin asset transfer response with transfer ID
    pub async fn margin_asset_transfer(
        &self,
        request: MarginAssetTransferRequest,
    ) -> RestResult<MarginAssetTransferResponse> {
        self.send_post_signed_request(MARGIN_ASSET_TRANSFER_ENDPOINT, request,
            EndpointType::MarginLoan,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_in_request() {
        let request = MarginAssetTransferRequest {
            symbol: "BTC_USDT".to_string(),
            currency: "BTC".to_string(),
            amount: "1".to_string(),
            side: "in".to_string(),
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.amount, "1");
        assert_eq!(request.side, "in");
    }

    #[test]
    fn test_transfer_out_request() {
        let request = MarginAssetTransferRequest {
            symbol: "ETH_USDT".to_string(),
            currency: "ETH".to_string(),
            amount: "0.5".to_string(),
            side: "out".to_string(),
        };

        assert_eq!(request.symbol, "ETH_USDT");
        assert_eq!(request.currency, "ETH");
        assert_eq!(request.amount, "0.5");
        assert_eq!(request.side, "out");
    }

    #[test]
    fn test_margin_asset_transfer_request_serialization() {
        let request = MarginAssetTransferRequest {
            symbol: "BTC_USDT".to_string(),
            currency: "BTC".to_string(),
            amount: "1".to_string(),
            side: "in".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BTC_USDT"));
        assert!(serialized.contains("BTC"));
        assert!(serialized.contains("\"1\""));
        assert!(serialized.contains("in"));
    }

    #[test]
    fn test_margin_asset_transfer_response_structure() {
        let response = MarginAssetTransferResponse {
            transfer_id: "124532".to_string(),
        };

        assert_eq!(response.transfer_id, "124532");
    }

    #[test]
    fn test_transfer_response_serialization_roundtrip() {
        let response = MarginAssetTransferResponse {
            transfer_id: "98765".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: MarginAssetTransferResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.transfer_id, deserialized.transfer_id);
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "transfer_id": "124532"
        }"#;

        let response: MarginAssetTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transfer_id, "124532");
    }

    #[test]
    fn test_request_validation() {
        let transfer_in = MarginAssetTransferRequest {
            symbol: "BTC_USDT".to_string(),
            currency: "BTC".to_string(),
            amount: "0.001".to_string(),
            side: "in".to_string(),
        };
        assert_eq!(transfer_in.side, "in");

        let transfer_out = MarginAssetTransferRequest {
            symbol: "BTC_USDT".to_string(),
            currency: "USDT".to_string(),
            amount: "100.00".to_string(),
            side: "out".to_string(),
        };
        assert_eq!(transfer_out.side, "out");
        assert_eq!(transfer_out.currency, "USDT");
    }
}
