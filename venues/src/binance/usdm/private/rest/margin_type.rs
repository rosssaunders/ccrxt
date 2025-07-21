use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{MarginType, RestResult};

const MARGIN_TYPE_ENDPOINT: &str = "/fapi/v1/marginType";

/// Request parameters for changing margin type.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginTypeRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// New margin type (ISOLATED or CROSSED).
    pub margin_type: MarginType,
}

/// Response from changing margin type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginTypeResponse {
    /// Response code (200 indicates success).
    pub code: i32,

    /// Response message.
    pub msg: String,
}

impl UsdmClient {
    /// Change Margin Type (TRADE)
    ///
    /// Change user's margin type in the specific symbol market.
    /// For Hedge Mode, LONG and SHORT positions of one symbol use the same margin type.
    /// With ISOLATED margin type, margins of the LONG and SHORT positions are isolated from each other.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Margin-Type
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
        self.send_signed_request(
            MARGIN_TYPE_ENDPOINT,
            reqwest::Method::POST,
            request,
            1,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_margin_type_request_serialization() {
        let request = ChangeMarginTypeRequest {
            symbol: "BTCUSDT".to_string(),
            margin_type: MarginType::Cross,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("marginType=CROSSED"));
    }

    #[test]
    fn test_change_margin_type_response_deserialization() {
        let json = r#"{"code":200,"msg":"success"}"#;
        let response: ChangeMarginTypeResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }

    #[test]
    fn test_change_margin_type_request_isolated() {
        let request = ChangeMarginTypeRequest {
            symbol: "ETHUSDT".to_string(),
            margin_type: MarginType::Isolated,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("marginType=ISOLATED"));
    }
}
