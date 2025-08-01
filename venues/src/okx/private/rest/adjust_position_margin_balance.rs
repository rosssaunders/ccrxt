use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};


const ACCOUNT_POSITION_MARGIN_BALANCE_ENDPOINT: &str = "api/v5/account/position/margin-balance";
/// Request to adjust position margin balance
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPositionMarginBalanceRequest {
    /// Instrument ID, e.g. "BTC-USDT-SWAP"
    pub inst_id: String,

    /// Position side
    /// "long", "short"
    /// Required for long/short mode
    pub pos_side: String,

    /// Type of adjust
    /// "add": add margin, "reduce": reduce margin
    pub r#type: String,

    /// Amount to be increased or decreased
    pub amt: String,

    /// Currency of the margin to be increased or decreased
    /// Only applicable to MARGIN orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Whether auto-loan is allowed
    /// true or false, the default is false
    /// Only applicable to MARGIN orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
}

/// Response from adjusting position margin balance
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPositionMarginBalanceResponse {
    /// Instrument ID
    pub inst_id: String,

    /// Position side
    pub pos_side: String,

    /// Amount
    pub amt: String,

    /// Type
    pub r#type: String,
}

impl RestClient {
    /// Adjust position margin balance
    ///
    /// # Arguments
    /// * `request` - The adjust position margin balance request
    ///
    /// # Returns
    /// A result containing the response or an error
    pub async fn adjust_position_margin_balance(
        &self,
        request: &AdjustPositionMarginBalanceRequest,
    ) -> RestResult<OkxApiResponse<AdjustPositionMarginBalanceResponse>> {
        self.send_request(
            ACCOUNT_POSITION_MARGIN_BALANCE_ENDPOINT,
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
    fn test_adjust_position_margin_balance_request_serialization() {
        let request = AdjustPositionMarginBalanceRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            pos_side: "long".to_string(),
            r#type: "add".to_string(),
            amt: "100.5".to_string(),
            ccy: Some("USDT".to_string()),
            auto: Some(false),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT-SWAP\""));
        assert!(json.contains("\"posSide\":\"long\""));
        assert!(json.contains("\"type\":\"add\""));
        assert!(json.contains("\"amt\":\"100.5\""));
        assert!(json.contains("\"ccy\":\"USDT\""));
        assert!(json.contains("\"auto\":false"));
    }

    #[test]
    fn test_adjust_position_margin_balance_minimal_request() {
        let request = AdjustPositionMarginBalanceRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            pos_side: "short".to_string(),
            r#type: "reduce".to_string(),
            amt: "50".to_string(),
            ccy: None,
            auto: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT-SWAP\""));
        assert!(json.contains("\"type\":\"reduce\""));
        assert!(!json.contains("\"ccy\""));
        assert!(!json.contains("\"auto\""));
    }

    #[test]
    fn test_adjust_position_margin_balance_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "posSide": "long",
                    "amt": "100.5",
                    "type": "add"
                }
            ]
        }"#;

        let response: OkxApiResponse<AdjustPositionMarginBalanceResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = &response.data[0];
        assert_eq!(result.inst_id, "BTC-USDT-SWAP");
        assert_eq!(result.pos_side, "long");
        assert_eq!(result.amt, "100.5");
        assert_eq!(result.r#type, "add");
    }
}
