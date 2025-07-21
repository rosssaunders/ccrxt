use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const CONVERT_EXCHANGE_INFO_ENDPOINT: &str = "/fapi/v1/convert/exchangeInfo";

/// Request parameters for getting convert exchange info.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertExchangeInfoRequest {
    /// From asset (optional). User spends coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_asset: Option<String>,

    /// To asset (optional). User receives coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_asset: Option<String>,

    /// Request timestamp in milliseconds since epoch.
    pub timestamp: u64,

    /// Optional receive window (milliseconds). If not set, default is used by API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Convert exchange info for a trading pair.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertExchangeInfo {
    /// From asset symbol.
    pub from_asset: String,

    /// To asset symbol.
    pub to_asset: String,

    /// From asset minimum amount.
    pub from_asset_min_amount: String,

    /// From asset maximum amount.
    pub from_asset_max_amount: String,

    /// To asset minimum amount.
    pub to_asset_min_amount: String,

    /// To asset maximum amount.
    pub to_asset_max_amount: String,
}

/// Response from convert exchange info endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct ConvertExchangeInfoResponse {
    /// List of convert exchange info pairs.
    pub pairs: Vec<ConvertExchangeInfo>,
}

impl UsdmClient {
    /// List All Convert Pairs
    ///
    /// Query for all convertible token pairs and the tokens' respective upper/lower limits.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/convert
    ///
    /// Rate limit: 20
    ///
    /// # Arguments
    /// * `params` - The convert exchange info request parameters
    ///
    /// # Returns
    /// ConvertExchangeInfoResponse - List of convert pairs and their limits
    pub async fn get_convert_exchange_info(
        &self,
        params: GetConvertExchangeInfoRequest,
    ) -> RestResult<ConvertExchangeInfoResponse> {
        self.send_signed_request(
            CONVERT_EXCHANGE_INFO_ENDPOINT,
            Method::GET,
            params,
            20,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_exchange_info_response_deserialization() {
        let json = r#"
        [
            {
                "fromAsset": "USDT",
                "toAsset": "BNB",
                "fromAssetMinAmount": "0.1",
                "fromAssetMaxAmount": "100",
                "toAssetMinAmount": "0.001",
                "toAssetMaxAmount": "1"
            }
        ]
        "#;

        let response: Vec<ConvertExchangeInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].from_asset, "USDT");
        assert_eq!(response[0].to_asset, "BNB");
    }

    #[test]
    fn test_get_convert_exchange_info_request_serialization() {
        let request = GetConvertExchangeInfoRequest {
            from_asset: Some("BTC".to_string()),
            to_asset: Some("USDT".to_string()),
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("fromAsset=BTC"));
        assert!(serialized.contains("toAsset=USDT"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }
}
