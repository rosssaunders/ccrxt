use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_MOVE_POSITIONS_HISTORY_ENDPOINT: &str = "api/v5/account/move-positions-history";

/// Request to get move positions history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMovePositionsHistoryRequest {
    /// Pagination of data to return records newer than the requested timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records earlier than the requested timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 100; the default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// Instrument type: FUTURES, SWAP, OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
}

/// Move positions history record
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionsHistory {
    /// Instrument ID
    pub inst_id: String,

    /// From account
    pub from: String,

    /// To account
    pub to: String,

    /// Amount
    pub amt: String,

    /// Timestamp
    pub ts: String,
}

impl RestClient {
    /// Get move positions history
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-move-position-history
    ///
    /// # Arguments
    /// * `request` - The get move positions history request
    ///
    /// # Returns
    /// A result containing the move positions history or an error
    pub async fn get_move_positions_history(
        &self,
        request: &GetMovePositionsHistoryRequest,
    ) -> RestResult<MovePositionsHistory> {
        self.send_request(
            ACCOUNT_MOVE_POSITIONS_HISTORY_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_move_positions_history_request_serialization() {
        let request = GetMovePositionsHistoryRequest {
            after: None,
            before: None,
            limit: Some("50".to_string()),
            inst_type: Some("SWAP".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("instType=SWAP"));
    }

    #[test]
    fn test_move_positions_history_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "from": "main",
                    "to": "sub1",
                    "amt": "1",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: OkxApiResponse<MovePositionsHistory> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let history = &response.data[0];
        assert_eq!(history.inst_id, "BTC-USDT-SWAP");
        assert_eq!(history.from, "main");
        assert_eq!(history.to, "sub1");
        assert_eq!(history.amt, "1");
        assert_eq!(history.ts, "1597026383085");
    }
}
