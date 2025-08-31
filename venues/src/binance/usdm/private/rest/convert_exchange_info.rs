use serde::{Deserialize, Serialize};

use crate::binance::usdm::{RestResult, private_client::UsdmClient};

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

/// Information about a convertible trading pair and its limits.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertExchangeInfo {
    /// Asset user spends (e.g., "BTC").
    pub from_asset: String,

    /// Asset user receives (e.g., "USDT").
    pub to_asset: String,

    /// Minimum amount of `from_asset` allowed for conversion (as string).
    pub from_asset_min_amount: String,

    /// Maximum amount of `from_asset` allowed for conversion (as string).
    pub from_asset_max_amount: String,

    /// Minimum amount of `to_asset` allowed for conversion (as string).
    pub to_asset_min_amount: String,

    /// Maximum amount of `to_asset` allowed for conversion (as string).
    pub to_asset_max_amount: String,
}

/// Response from convert exchange info endpoint.
///
/// This is a transparent wrapper for a list of `ConvertExchangeInfo`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct ConvertExchangeInfoResponse(pub Vec<ConvertExchangeInfo>);

impl UsdmClient {
    /// List All Convert Pairs
    ///
    /// Query for all convertible token pairs and the tokens' respective upper/lower limits.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/convert)
    ///
    /// Rate limit: 20 (IP)
    ///
    /// # Arguments
    /// * `params` - The convert exchange info request parameters
    ///
    /// # Returns
    /// `ConvertExchangeInfoResponse` - List of convert pairs and their limits
    pub async fn get_convert_exchange_info(
        &self,
        params: GetConvertExchangeInfoRequest,
    ) -> RestResult<ConvertExchangeInfoResponse> {
        self.send_get_signed_request(CONVERT_EXCHANGE_INFO_ENDPOINT, params, 20, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

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
        let response: ConvertExchangeInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0.len(), 1);
        let info = &response.0[0];
        assert_eq!(info.from_asset, "USDT");
        assert_eq!(info.to_asset, "BNB");
        assert_eq!(info.from_asset_min_amount, "0.1");
        assert_eq!(info.from_asset_max_amount, "100");
        assert_eq!(info.to_asset_min_amount, "0.001");
        assert_eq!(info.to_asset_max_amount, "1");
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

    #[test]
    fn test_convert_exchange_info_partial_fields() {
        // Test deserialization with only required fields
        let json = r#"
        [
            {
                "fromAsset": "BTC",
                "toAsset": "USDT",
                "fromAssetMinAmount": "0.0004",
                "fromAssetMaxAmount": "50",
                "toAssetMinAmount": "20",
                "toAssetMaxAmount": "2500000"
            }
        ]
        "#;
        let response: ConvertExchangeInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0[0].from_asset, "BTC");
        assert_eq!(response.0[0].to_asset, "USDT");
    }
}
