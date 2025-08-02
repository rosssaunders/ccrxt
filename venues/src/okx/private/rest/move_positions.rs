use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};


const ACCOUNT_MOVE_POSITIONS_ENDPOINT: &str = "api/v5/account/move-positions";
/// Request to move positions
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionsRequest {
    /// Sub account name to move to
    pub sub_acct: String,

    /// Instrument ID
    pub inst_id: String,

    /// Currency
    pub ccy: String,

    /// From account
    pub from: String,

    /// To account
    pub to: String,

    /// Amount
    pub amt: String,
}

/// Response for move positions
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionsResponse {
    /// Sub account name
    pub sub_acct: String,

    /// Instrument ID
    pub inst_id: String,

    /// Currency
    pub ccy: String,

    /// From account
    pub from: String,

    /// To account
    pub to: String,

    /// Amount
    pub amt: String,
}

impl RestClient {
    /// Move positions
    ///
    /// # Arguments
    /// * `request` - The move positions request
    ///
    /// # Returns
    /// A result containing the move positions response or an error
    pub async fn move_positions(
        &self,
        request: &MovePositionsRequest,
    ) -> RestResult<OkxApiResponse<MovePositionsResponse>> {
        self.send_request(
            ACCOUNT_MOVE_POSITIONS_ENDPOINT,
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
    fn test_move_positions_request_serialization() {
        let request = MovePositionsRequest {
            sub_acct: "sub1".to_string(),
            inst_id: "BTC-USDT-SWAP".to_string(),
            ccy: "BTC".to_string(),
            from: "main".to_string(),
            to: "sub".to_string(),
            amt: "1".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"subAcct\":\"sub1\""));
        assert!(json.contains("\"instId\":\"BTC-USDT-SWAP\""));
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"from\":\"main\""));
        assert!(json.contains("\"to\":\"sub\""));
        assert!(json.contains("\"amt\":\"1\""));
    }

    #[test]
    fn test_move_positions_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "subAcct": "sub1",
                    "instId": "BTC-USDT-SWAP",
                    "ccy": "BTC",
                    "from": "main",
                    "to": "sub",
                    "amt": "1"
                }
            ]
        }"#;

        let response: OkxApiResponse<MovePositionsResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = response.data.first();
        assert!(result.is_some(), "Expected at least one result in response");
        let result = result.unwrap();
        assert_eq!(result.sub_acct, "sub1");
        assert_eq!(result.inst_id, "BTC-USDT-SWAP");
        assert_eq!(result.ccy, "BTC");
        assert_eq!(result.from, "main");
        assert_eq!(result.to, "sub");
        assert_eq!(result.amt, "1");
    }
}
