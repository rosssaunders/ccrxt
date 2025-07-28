use serde::{Deserialize, Serialize};

use crate::binance::coinm::{MarginType, RestResult, private::rest::client::RestClient};

const MARGIN_TYPE_ENDPOINT: &str = "/dapi/v1/marginType";

/// Request parameters for changing margin type (POST /dapi/v1/marginType).
#[derive(Debug, Clone, Serialize)]
pub struct ChangeMarginTypeRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Margin type: ISOLATED or CROSS.
    #[serde(rename = "marginType")]
    pub margin_type: MarginType,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for changing margin type (POST /dapi/v1/marginType).
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeMarginTypeResponse {
    /// Response code (200 for success).
    pub code: u32,

    /// Response message.
    pub msg: String,
}

impl RestClient {
    /// Changes user's margin type (TRADE) for a specific symbol on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type
    ///
    /// POST /dapi/v1/marginType
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Changes user's margin type in the specific symbol market.
    /// For Hedge Mode, LONG and SHORT positions of one symbol use the same margin type.
    /// With ISOLATED margin type, margins of the LONG and SHORT positions are isolated from each other.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ChangeMarginTypeRequest`])
    ///
    /// # Returns
    /// A [`ChangeMarginTypeResponse`] with the operation result.
    pub async fn change_margin_type(
        &self,
        params: ChangeMarginTypeRequest,
    ) -> RestResult<ChangeMarginTypeResponse> {
        self.send_signed_request(MARGIN_TYPE_ENDPOINT, reqwest::Method::POST, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_margin_type_request_serialization_isolated() {
        let request = ChangeMarginTypeRequest {
            symbol: "BTCUSD_PERP".to_string(),
            margin_type: MarginType::Isolated,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("marginType=ISOLATED"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_change_margin_type_request_serialization_crossed() {
        let request = ChangeMarginTypeRequest {
            symbol: "ETHUSD_PERP".to_string(),
            margin_type: MarginType::Cross,
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("marginType=CROSS"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_change_margin_type_response_deserialization() {
        let json = r#"{
            "code": 200,
            "msg": "success"
        }"#;

        let response: ChangeMarginTypeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }

    #[test]
    fn test_change_margin_type_response_deserialization_with_different_message() {
        let json = r#"{
            "code": 200,
            "msg": "Margin type changed successfully"
        }"#;

        let response: ChangeMarginTypeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "Margin type changed successfully");
    }
}
