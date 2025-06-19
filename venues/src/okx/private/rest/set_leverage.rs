use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to set leverage
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest {
    /// Instrument ID, e.g. "BTC-USDT-SWAP"
    pub inst_id: String,

    /// Margin mode
    /// "isolated", "cross"
    pub mgn_mode: String,

    /// Leverage
    pub lever: String,

    /// Position side
    /// Required in long/short mode for FUTURES/SWAP
    /// "long", "short", "net"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
}

/// Response from setting leverage
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageResponse {
    /// Instrument ID
    pub inst_id: String,

    /// Margin mode
    pub mgn_mode: String,

    /// Leverage
    pub lever: String,

    /// Position side
    pub pos_side: String,
}

impl RestClient {
    /// Set leverage
    ///
    /// # Arguments
    /// * `request` - The set leverage request
    ///
    /// # Returns
    /// A result containing the leverage response or an error
    pub async fn set_leverage(&self, request: &SetLeverageRequest) -> RestResult<OkxApiResponse<SetLeverageResponse>> {
        self.send_request(
            "api/v5/account/set-leverage",
            reqwest::Method::POST,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_leverage_request_serialization() {
        let request = SetLeverageRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            mgn_mode: "isolated".to_string(),
            lever: "10".to_string(),
            pos_side: Some("long".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT-SWAP\""));
        assert!(json.contains("\"mgnMode\":\"isolated\""));
        assert!(json.contains("\"lever\":\"10\""));
        assert!(json.contains("\"posSide\":\"long\""));
    }

    #[test]
    fn test_set_leverage_request_without_pos_side() {
        let request = SetLeverageRequest {
            inst_id: "BTC-USDT".to_string(),
            mgn_mode: "cross".to_string(),
            lever: "5".to_string(),
            pos_side: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"mgnMode\":\"cross\""));
        assert!(json.contains("\"lever\":\"5\""));
        assert!(!json.contains("\"posSide\""));
    }

    #[test]
    fn test_set_leverage_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "mgnMode": "isolated",
                    "lever": "10",
                    "posSide": "long"
                }
            ]
        }"#;

        let response: OkxApiResponse<SetLeverageResponse> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = &response.data[0];
        assert_eq!(result.inst_id, "BTC-USDT-SWAP");
        assert_eq!(result.mgn_mode, "isolated");
        assert_eq!(result.lever, "10");
        assert_eq!(result.pos_side, "long");
    }
}