use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

const ACCOUNT_POSITIONS_HISTORY_ENDPOINT: &str = "api/v5/account/positions-history";

/// Request to get account positions history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsHistoryRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// Instrument ID, e.g. "BTC-USDT-SWAP"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Margin mode
    /// "cross", "isolated"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,

    /// The type of closing position
    /// "1": close position partially; "2": close all positions; "3": liquidation; "4": partial liquidation; "5": ADL
    /// It is the latest type if there are several types for the same position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Pagination of data to return records earlier than the requested posId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested posId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. Maximum is 100. Default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Position history details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionHistory {
    /// Instrument ID
    pub inst_id: String,

    /// Instrument type
    pub inst_type: String,

    /// Margin mode
    pub mgn_mode: String,

    /// The type of closing position
    pub r#type: String,

    /// Creation time
    pub c_time: String,

    /// Update time
    pub u_time: String,

    /// Average price of opening position
    pub open_avg_px: String,

    /// Average price of closing position
    pub close_avg_px: String,

    /// Position ID
    pub pos_id: String,

    /// Maximum position size
    pub max_pos: String,

    /// Position side
    pub pos_side: String,

    /// Quantity of position closed
    pub close_pos_sz: String,

    /// Profit and loss
    pub pnl: String,

    /// Profit and loss ratio
    pub pnl_ratio: String,

    /// Lever
    pub lever: String,

    /// Direction
    /// "short" or "long"
    pub direction: String,

    /// Trigger price
    pub trigger_px: Option<String>,

    /// Underlying
    pub uly: Option<String>,

    /// Currency used for margin
    pub ccy: String,
}

impl RestClient {
    /// Get account positions history
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-positions-history)
    ///
    /// # Arguments
    /// * `request` - The get positions history request
    ///
    /// # Returns
    /// A result containing the positions history or an error
    pub async fn get_positions_history(
        &self,
        request: &GetPositionsHistoryRequest,
    ) -> RestResult<PositionHistory> {
        self.send_get_request(
            ACCOUNT_POSITIONS_HISTORY_ENDPOINT,
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
    fn test_get_positions_history_request_serialization() {
        let request = GetPositionsHistoryRequest {
            inst_type: Some(InstrumentType::Swap),
            inst_id: Some("BTC-USDT-SWAP".to_string()),
            mgn_mode: Some("cross".to_string()),
            r#type: Some("1".to_string()),
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SWAP"));
        assert!(serialized.contains("instId=BTC-USDT-SWAP"));
        assert!(serialized.contains("mgnMode=cross"));
        assert!(serialized.contains("type=1"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_positions_history_minimal_request() {
        let request = GetPositionsHistoryRequest {
            inst_type: None,
            inst_id: None,
            mgn_mode: None,
            r#type: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_position_history_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "instType": "SWAP",
                    "mgnMode": "cross",
                    "type": "1",
                    "cTime": "1597026383085",
                    "uTime": "1597026383085",
                    "openAvgPx": "50000",
                    "closeAvgPx": "51000",
                    "posId": "123456789",
                    "maxPos": "1000",
                    "posSide": "long",
                    "closePosSz": "500",
                    "pnl": "100.5",
                    "pnlRatio": "0.002",
                    "lever": "10",
                    "direction": "long",
                    "triggerPx": "",
                    "uly": "BTC-USD",
                    "ccy": "USDT"
                }
            ]
        }"#;

        let response: OkxApiResponse<PositionHistory> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let position = &response.data[0];
        assert_eq!(position.inst_id, "BTC-USDT-SWAP");
        assert_eq!(position.pos_id, "123456789");
        assert_eq!(position.pnl, "100.5");
        assert_eq!(position.direction, "long");
    }
}
