use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, TransferType, PrivateRestClient as RestClient},
};

const UNIVERSAL_TRANSFER_ENDPOINT: &str = "/sapi/v1/asset/transfer";

/// Request parameters for universal transfer.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferRequest {
    /// Transfer type
    #[serde(rename = "type")]
    pub transfer_type: TransferType,

    /// Asset to transfer
    pub asset: String,

    /// Amount to transfer
    pub amount: String,

    /// From symbol (required for isolated margin transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_symbol: Option<String>,

    /// To symbol (required for isolated margin transfers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_symbol: Option<String>,

    /// This parameter cannot be used in combination with other parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for universal transfer.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferResponse {
    /// Transaction id
    pub tran_id: u64,
}

impl RestClient {
    /// Perform universal transfer on Binance.
    ///
    /// See: <https://binance-docs.github.io/apidocs/spot/en/>
    /// POST /sapi/v1/asset/transfer
    /// Weight: 900
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`UniversalTransferRequest`])
    ///
    /// # Returns
    /// A [`UniversalTransferResponse`] object with transaction details.
    pub async fn universal_transfer(
        &self,
        params: UniversalTransferRequest,
    ) -> RestResult<UniversalTransferResponse> {
        let weight = 900;
        self.send_post_signed_request(
            UNIVERSAL_TRANSFER_ENDPOINT,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_transfer_request_serialization() {
        let request = UniversalTransferRequest {
            transfer_type: TransferType::MainUmfuture,
            asset: "USDT".to_string(),
            amount: "100.50".to_string(),
            from_symbol: None,
            to_symbol: None,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=MAIN_UMFUTURE"));
        assert!(serialized.contains("asset=USDT"));
        assert!(serialized.contains("amount=100.50"));
        assert!(!serialized.contains("fromSymbol"));
        assert!(!serialized.contains("toSymbol"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_universal_transfer_request_with_symbols() {
        let request = UniversalTransferRequest {
            transfer_type: TransferType::IsolatedmarginMargin,
            asset: "BTC".to_string(),
            amount: "0.001".to_string(),
            from_symbol: Some("BTCUSDT".to_string()),
            to_symbol: None,
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=ISOLATEDMARGIN_MARGIN"));
        assert!(serialized.contains("asset=BTC"));
        assert!(serialized.contains("amount=0.001"));
        assert!(serialized.contains("fromSymbol=BTCUSDT"));
        assert!(!serialized.contains("toSymbol"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_universal_transfer_request_bidirectional() {
        let request = UniversalTransferRequest {
            transfer_type: TransferType::MarginIsolatedmargin,
            asset: "ETH".to_string(),
            amount: "0.5".to_string(),
            from_symbol: None,
            to_symbol: Some("ETHUSDT".to_string()),
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=MARGIN_ISOLATEDMARGIN"));
        assert!(serialized.contains("asset=ETH"));
        assert!(serialized.contains("amount=0.5"));
        assert!(!serialized.contains("fromSymbol"));
        assert!(serialized.contains("toSymbol=ETHUSDT"));
    }

    #[test]
    fn test_universal_transfer_response_deserialization() {
        let json = r#"{
            "tranId": 13526853623
        }"#;

        let response: UniversalTransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.tran_id, 13526853623);
    }
}
