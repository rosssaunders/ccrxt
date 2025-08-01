use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};


const TRADE_CLOSE_POSITION_ENDPOINT: &str = "api/v5/trade/close-position";
/// Request to close a position
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionRequest {
    /// Instrument ID, e.g. "BTC-USDT-SWAP"
    pub inst_id: String,

    /// Position side
    /// The value can only be `long` or `short`.
    /// Only applicable to FUTURES/SWAP in the `long/short` mode. Required in the `long/short` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,

    /// Margin mode
    /// `cross`: cross, `isolated`: isolated
    pub mgn_mode: String,

    /// Currency, only applicable to `MARGIN` orders in `Single-currency margin`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Whether the order is auto-margin decrease only
    /// `true` or `false`, the default is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_ccy: Option<bool>,

    /// Client Order ID as assigned by the client
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,

    /// Order tag
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Response from closing a position
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionResponse {
    /// Instrument ID
    pub inst_id: String,

    /// Position side
    pub pos_side: Option<String>,

    /// Client Order ID as assigned by the client
    pub cl_ord_id: Option<String>,

    /// Order tag
    pub tag: Option<String>,
}

impl RestClient {
    /// Close a position
    ///
    /// # Arguments
    /// * `request` - The close position request
    ///
    /// # Returns
    /// A result containing the close position response or an error
    pub async fn close_position(
        &self,
        request: &ClosePositionRequest,
    ) -> RestResult<OkxApiResponse<ClosePositionResponse>> {
        self.send_request(
            TRADE_CLOSE_POSITION_ENDPOINT,
            reqwest::Method::POST,
            Some(request),
            EndpointType::PrivateTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_close_position_request_serialization() {
        let request = ClosePositionRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            pos_side: Some("long".to_string()),
            mgn_mode: "isolated".to_string(),
            ccy: None,
            auto_ccy: Some(false),
            cl_ord_id: Some("close_pos_123".to_string()),
            tag: Some("my_tag".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT-SWAP\""));
        assert!(json.contains("\"posSide\":\"long\""));
        assert!(json.contains("\"mgnMode\":\"isolated\""));
        assert!(json.contains("\"autoCcy\":false"));
        assert!(json.contains("\"clOrdId\":\"close_pos_123\""));
        assert!(json.contains("\"tag\":\"my_tag\""));
    }

    #[test]
    fn test_close_position_minimal_request() {
        let request = ClosePositionRequest {
            inst_id: "ETH-USDT-SWAP".to_string(),
            pos_side: None,
            mgn_mode: "cross".to_string(),
            ccy: None,
            auto_ccy: None,
            cl_ord_id: None,
            tag: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"ETH-USDT-SWAP\""));
        assert!(json.contains("\"mgnMode\":\"cross\""));
        assert!(!json.contains("posSide"));
        assert!(!json.contains("autoCcy"));
        assert!(!json.contains("clOrdId"));
        assert!(!json.contains("tag"));
    }

    #[test]
    fn test_close_position_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "posSide": "long",
                    "clOrdId": "close_pos_123",
                    "tag": "my_tag"
                }
            ]
        }"#;

        let response: OkxApiResponse<ClosePositionResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].inst_id, "BTC-USDT-SWAP");
        assert_eq!(response.data[0].pos_side, Some("long".to_string()));
        assert_eq!(
            response.data[0].cl_ord_id,
            Some("close_pos_123".to_string())
        );
        assert_eq!(response.data[0].tag, Some("my_tag".to_string()));
    }
}
