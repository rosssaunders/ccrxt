use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{MarginType, RestResult};

const MARGIN_TYPE_ENDPOINT: &str = "/fapi/v1/marginType";

/// Request parameters for the Change Margin Type endpoint.
///
/// Changes the margin type for a specific symbol. For Hedge Mode, LONG and SHORT positions of one symbol use the same margin type.
/// With ISOLATED margin type, margins of the LONG and SHORT positions are isolated from each other.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginTypeRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Required.
    pub symbol: String,

    /// New margin type. Valid values: ISOLATED, CROSSED. Required.
    pub margin_type: MarginType,

    /// The number of milliseconds the request is valid for after timestamp. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch). Required.
    pub timestamp: u64,
}

/// Response from the Change Margin Type endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginTypeResponse {
    /// Response code. 200 indicates success.
    pub code: i32,

    /// Response message from the API.
    pub msg: String,
}

impl UsdmClient {
    /// Change Margin Type (TRADE)
    ///
    /// Changes the margin type for a specific symbol. For Hedge Mode, LONG and SHORT positions of one symbol use the same margin type.
    /// With ISOLATED margin type, margins of the LONG and SHORT positions are isolated from each other.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Margin-Type)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Response confirming the margin type change.
    pub async fn change_margin_type(
        &self,
        request: ChangeMarginTypeRequest,
    ) -> RestResult<ChangeMarginTypeResponse> {
        self.send_post_signed_request(MARGIN_TYPE_ENDPOINT, request, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::usdm::MarginType;

    #[test]
    fn test_change_margin_type_request_serialization_cross() {
        let request = ChangeMarginTypeRequest {
            symbol: "BTCUSDT".to_string(),
            margin_type: MarginType::Cross,
            recv_window: Some(5000),
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("marginType=CROSS"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_change_margin_type_request_serialization_isolated() {
        let request = ChangeMarginTypeRequest {
            symbol: "ETHUSDT".to_string(),
            margin_type: MarginType::Isolated,
            recv_window: None,
            timestamp: 9876543210,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("marginType=ISOLATED"));
        assert!(!serialized.contains("recvWindow"));
        assert!(serialized.contains("timestamp=9876543210"));
    }

    #[test]
    fn test_change_margin_type_response_deserialization() {
        let json = r#"{"code":200,"msg":"success"}"#;
        let response: ChangeMarginTypeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }
}
